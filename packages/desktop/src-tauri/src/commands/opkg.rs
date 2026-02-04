use crate::fs::copy_dir_recursive;
use crate::opkg::opkg_install as opkg_install_inner;
use crate::types::ExecResult;
use crate::workspace::state::load_workspace_state;
use tauri::AppHandle;

#[tauri::command]
pub fn opkg_install(project_dir: String, package: String) -> Result<ExecResult, String> {
    let project_dir = project_dir.trim().to_string();
    if project_dir.is_empty() {
        return Err("projectDir is required".to_string());
    }

    let package = package.trim().to_string();
    if package.is_empty() {
        return Err("package is required".to_string());
    }

    opkg_install_inner(&project_dir, &package)
}

#[tauri::command]
pub fn import_skill(
    app: AppHandle,
    project_dir: String,
    source_dir: String,
    overwrite: bool,
) -> Result<ExecResult, String> {
    let project_dir = project_dir.trim().to_string();
    if project_dir.is_empty() {
        return Err("projectDir is required".to_string());
    }

    let source_dir = source_dir.trim().to_string();
    if source_dir.is_empty() {
        return Err("sourceDir is required".to_string());
    }

    // Validate source_dir is within authorized roots for security
    let src = std::path::PathBuf::from(&source_dir);
    let canonical_src = std::fs::canonicalize(&src)
        .map_err(|e| format!("Failed to resolve source directory: {e}"))?;
    
    if !canonical_src.is_dir() {
        return Err("Source directory must be a directory".to_string());
    }

    // Load authorized roots and verify source is within them
    let state = load_workspace_state(&app)?;
    let mut allowed = false;
    
    for workspace in state.workspaces {
        let workspace_path = std::path::PathBuf::from(&workspace.path);
        if let Ok(canonical_root) = std::fs::canonicalize(&workspace_path) {
            if canonical_src.starts_with(&canonical_root) {
                allowed = true;
                break;
            }
        }
    }
    
    if !allowed {
        return Err("Source directory is not within an authorized workspace root. Only directories within workspace roots can be imported for security.".to_string());
    }

    let name = canonical_src
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Failed to infer skill name from directory".to_string())?;

    let dest = std::path::PathBuf::from(&project_dir)
        .join(".opencode")
        .join("skills")
        .join(name);

    if dest.exists() {
        if overwrite {
            std::fs::remove_dir_all(&dest).map_err(|e| {
                format!(
                    "Failed to remove existing skill dir {}: {e}",
                    dest.display()
                )
            })?;
        } else {
            return Err(format!("Skill already exists at {}", dest.display()));
        }
    }

    copy_dir_recursive(&canonical_src, &dest)?;

    Ok(ExecResult {
        ok: true,
        status: 0,
        stdout: format!("Imported skill to {}", dest.display()),
        stderr: String::new(),
    })
}
