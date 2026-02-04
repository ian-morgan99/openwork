use std::process::{Command, Stdio};

use crate::platform::configure_hidden;
use crate::types::ExecResult;

/// Validates that a package name is safe to use as a command argument
/// Prevents command injection by restricting to safe characters
fn validate_package_name(package: &str) -> Result<String, String> {
    let trimmed = package.trim();
    
    if trimmed.is_empty() {
        return Err("Package name cannot be empty".to_string());
    }
    
    if trimmed.len() > 256 {
        return Err("Package name too long (max 256 characters)".to_string());
    }
    
    // Allow alphanumeric, hyphens, underscores, slashes (for scoped packages), dots, and @ (for npm scopes)
    // This covers npm packages like @org/package and git URLs
    for ch in trimmed.chars() {
        if !ch.is_ascii_alphanumeric() 
            && ch != '-' 
            && ch != '_' 
            && ch != '/' 
            && ch != '.' 
            && ch != '@' 
            && ch != ':' {
            return Err(format!("Package name contains invalid character: '{ch}'. Only alphanumeric, -, _, /, ., @, and : are allowed."));
        }
    }
    
    // Prevent starting with dangerous characters
    if trimmed.starts_with('-') || trimmed.starts_with('.') {
        return Err("Package name cannot start with '-' or '.'".to_string());
    }
    
    Ok(trimmed.to_string())
}

pub fn run_capture_optional(command: &mut Command) -> Result<Option<ExecResult>, String> {
    match command.output() {
        Ok(output) => {
            let status = output.status.code().unwrap_or(-1);
            Ok(Some(ExecResult {
                ok: output.status.success(),
                status,
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            }))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!(
            "Failed to run {}: {e}",
            command.get_program().to_string_lossy()
        )),
    }
}

pub fn opkg_install(project_dir: &str, package: &str) -> Result<ExecResult, String> {
    // Validate package name for security
    let safe_package = validate_package_name(package)?;
    
    let mut opkg = Command::new("opkg");
    configure_hidden(&mut opkg);
    opkg.arg("install")
        .arg(&safe_package)
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(result) = run_capture_optional(&mut opkg)? {
        return Ok(result);
    }

    let mut openpackage = Command::new("openpackage");
    configure_hidden(&mut openpackage);
    openpackage
        .arg("install")
        .arg(&safe_package)
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(result) = run_capture_optional(&mut openpackage)? {
        return Ok(result);
    }

    let mut pnpm = Command::new("pnpm");
    configure_hidden(&mut pnpm);
    pnpm.arg("dlx")
        .arg("opkg")
        .arg("install")
        .arg(&safe_package)
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(result) = run_capture_optional(&mut pnpm)? {
        return Ok(result);
    }

    let mut npx = Command::new("npx");
    configure_hidden(&mut npx);
    npx.arg("opkg")
        .arg("install")
        .arg(&safe_package)
        .current_dir(project_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(result) = run_capture_optional(&mut npx)? {
        return Ok(result);
    }

    Ok(ExecResult {
    ok: false,
    status: -1,
    stdout: String::new(),
    stderr: "OpenPackage CLI not found. Install with `npm install -g opkg` (or `openpackage`), or ensure pnpm/npx is available.".to_string(),
  })
}
