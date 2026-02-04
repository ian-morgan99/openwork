# Security Assessment and Guidelines for OpenWork

**Assessment Date:** February 2026  
**Version:** 0.11.9

## Executive Summary

OpenWork is a Tauri-based desktop application that provides a GUI for OpenCode with filesystem access capabilities. This security assessment identifies several areas requiring attention to ensure safe operation on user systems.

### Overall Security Posture: ⚠️ MODERATE RISK

**Key Findings:**
- ✅ Good: Input sanitization exists for command names and skill names
- ✅ Good: Path traversal protection in zip file extraction
- ✅ Good: Authorized roots system for filesystem access control
- ⚠️ Moderate: Process spawning with user-controlled input
- ⚠️ Moderate: CSP disabled in Tauri configuration
- ⚠️ Moderate: Broad HTTP permissions
- ⚠️ Moderate: No dependency vulnerability scanning in CI/CD

## Critical Security Features

### 1. Filesystem Access Control ✅

**Location:** `packages/desktop/src-tauri/src/commands/misc.rs`

OpenWork implements an **authorized roots** system that restricts filesystem access:

```rust
fn validate_project_dir(app: &AppHandle, project_dir: &str) -> Result<PathBuf, String> {
    let canonical = fs::canonicalize(&project_path)?;
    let roots = load_authorized_roots(app)?;
    
    // Verify project_dir is within authorized roots
    for root in roots {
        if canonical.starts_with(&root) {
            allowed = true;
            break;
        }
    }
    
    if !allowed {
        return Err("project_dir is not within an authorized root".to_string());
    }
    Ok(canonical)
}
```

**How it works:**
- Each workspace defines authorized root directories in `.opencode/openwork.json`
- All filesystem operations must occur within these authorized roots
- Paths are canonicalized to prevent symbolic link attacks
- Uses `starts_with` check to prevent directory traversal

**User Action Required:** Users must explicitly add directories to authorized roots through the UI.

### 2. Path Traversal Protection ✅

**Location:** `packages/desktop/src-tauri/src/commands/workspace.rs` (lines 714-721)

Zip file extraction includes path traversal checks:

```rust
if entry_path.components().any(|component| match component {
    std::path::Component::ParentDir
    | std::path::Component::RootDir
    | std::path::Component::Prefix(_) => true,
    _ => false,
}) {
    return Err("Archive contains an unsafe path".to_string());
}
```

This prevents malicious zip files from extracting to parent directories or absolute paths.

### 3. Input Sanitization ✅

**Location:** `packages/desktop/src-tauri/src/workspace/commands.rs`

Command and skill names are sanitized:

```rust
pub fn sanitize_command_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim().trim_start_matches('/');
    let mut out = String::new();
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
            out.push(ch);
        }
    }
    // Returns None if empty
    Some(out)
}
```

Server names are also validated in `misc.rs`:
```rust
fn validate_server_name(name: &str) -> Result<String, String> {
    if !trimmed.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err("server_name must be alphanumeric with '-' or '_'".to_string());
    }
    Ok(trimmed.to_string())
}
```

## Security Vulnerabilities and Risks

### 1. Content Security Policy Disabled ⚠️

**Location:** `packages/desktop/src-tauri/tauri.conf.json`

```json
"security": {
  "csp": null
}
```

**Risk:** Without CSP, the frontend is vulnerable to XSS attacks if malicious content is rendered.

**Impact:** Medium - Could allow code execution if untrusted data is displayed in the UI

**Recommendation:** Enable CSP with appropriate directives:
```json
"security": {
  "csp": "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; connect-src 'self' http://localhost:* https://*"
}
```

### 2. Overly Permissive HTTP Access ⚠️

**Location:** `packages/desktop/src-tauri/capabilities/default.json`

```json
"identifier": "http:default",
"allow": [
  { "url": "http://*" },
  { "url": "https://*" },
  { "url": "http://*:*/*" },
  { "url": "https://*:*/*" }
]
```

**Risk:** Application can connect to any HTTP/HTTPS endpoint

**Impact:** Medium - Could be exploited for SSRF attacks or data exfiltration

**Recommendation:** Restrict to specific domains or localhost only when possible

### 3. Process Spawning with User Input ⚠️

**Location:** Multiple files (`opkg.rs`, `engine/spawn.rs`, `scheduler.rs`)

**Risk:** The application spawns processes with user-controlled arguments:

```rust
// In opkg.rs
let mut opkg = Command::new("opkg");
opkg.arg("install").arg(package)  // package from user input
```

```rust
// In misc.rs
command_for_program(&program)
    .arg("mcp")
    .arg("auth")
    .arg(server_name)  // validated but still external input
```

**Current Mitigations:**
- ✅ Arguments are passed as separate args (not shell-interpreted)
- ✅ Server names are validated (alphanumeric + `-` + `_`)
- ✅ No direct shell execution (`sh -c`)

**Remaining Risk:** Low-Medium - While args are not shell-interpreted, malicious package names could still cause issues

**Recommendation:** 
- Add allow-list validation for package names
- Consider sandboxing spawned processes
- Add rate limiting on process spawning

### 4. Import Skill Directory Traversal Risk ⚠️

**Location:** `packages/desktop/src-tauri/src/commands/opkg.rs`

```rust
pub fn import_skill(project_dir: String, source_dir: String, overwrite: bool) {
    let src = std::path::PathBuf::from(&source_dir);
    let dest = std::path::PathBuf::from(&project_dir)
        .join(".opencode")
        .join("skills")
        .join(name);
    copy_dir_recursive(&src, &dest)?;
}
```

**Risk:** `source_dir` is user-provided but not validated against authorized roots

**Impact:** Medium - Users could import skills from anywhere on their filesystem

**Recommendation:** Add validation that `source_dir` is within authorized roots

### 5. No Dependency Vulnerability Scanning ⚠️

**Risk:** No automated scanning for vulnerable dependencies in Cargo.lock or package-lock files

**Recommendation:** Add to CI/CD pipeline:
```yaml
# For Rust dependencies
- cargo audit
# For npm dependencies  
- pnpm audit
```

### 6. Remote URL Validation ✅ (Good)

**Location:** `packages/desktop/src-tauri/src/commands/workspace.rs`

Remote URLs are validated:
```rust
if !base_url.starts_with("http://") && !base_url.starts_with("https://") {
    return Err("baseUrl must start with http:// or https://".to_string());
}
```

This prevents `file://` or other protocol exploits.

## Security Best Practices

### For Users

1. **Review Authorized Roots:** Only add directories you trust to authorized roots
2. **Verify Workspace Origins:** When importing workspace configs, ensure they're from trusted sources
3. **Monitor Scheduled Jobs:** Review scheduled jobs periodically (Settings → Scheduled)
4. **Keep Updated:** Enable auto-updates to receive security patches
5. **Network Access:** Be aware the app can connect to any HTTP/HTTPS endpoint

### For Developers

1. **Never Disable Security Features:** Keep path validation and sanitization in place
2. **Validate All Input:** Assume all IPC command arguments are potentially malicious
3. **Use Canonicalized Paths:** Always canonicalize paths before security checks
4. **Principle of Least Privilege:** Request minimal Tauri permissions needed
5. **Security Reviews:** Review all PRs touching security-sensitive code

## Testing Security

### Manual Security Testing

1. **Path Traversal Test:**
   ```bash
   # Try to create workspace outside authorized roots
   # Expected: Should be blocked
   ```

2. **Zip Bomb Test:**
   ```bash
   # Import a workspace with deeply nested zip
   # Expected: Should handle gracefully
   ```

3. **Command Injection Test:**
   ```bash
   # Try special characters in command names: `; rm -rf /`
   # Expected: Should be sanitized
   ```

### Automated Testing

Recommended additions:
- Property-based testing for path sanitization
- Fuzzing for command parsers
- Integration tests for authorization checks

## Incident Response

If you discover a security vulnerability:

1. **Do not** open a public GitHub issue
2. Email security contact: [Add security contact email]
3. Include:
   - Detailed description
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

## Compliance and Standards

OpenWork follows these security principles:
- ✅ Input validation on all user inputs
- ✅ Output encoding for display
- ✅ Path traversal prevention
- ✅ Principle of least privilege for file access
- ⚠️ Defense in depth (partially implemented)

## Recommendations for Immediate Action

### High Priority

1. **Enable CSP** - Add Content Security Policy to `tauri.conf.json`
2. **Add Dependency Scanning** - Add cargo-audit and pnpm audit to CI
3. **Validate Import Sources** - Check source_dir against authorized roots in import_skill

### Medium Priority

4. **Restrict HTTP Scope** - Limit HTTP permissions to necessary domains
5. **Add Security Testing** - Implement automated security tests
6. **Security Audit Log** - Add logging for security-relevant events

### Low Priority

7. **Rate Limiting** - Add rate limits on expensive operations
8. **Sandbox Processes** - Consider additional sandboxing for spawned processes
9. **Security Headers** - Review and harden HTTP response headers

## Is OpenWork Safe to Use?

**Yes, with caveats:**

✅ **Safe for trusted content:** If you only work with your own workspaces and don't import untrusted configs
✅ **Good foundational security:** Path validation and input sanitization are present
✅ **Active development:** Security issues can be addressed quickly

⚠️ **Use caution with:**
- Importing workspace configs from untrusted sources
- Adding untrusted directories to authorized roots
- Running with network access to sensitive internal networks

## Keeping OpenWork Safe

### As a User

1. **Update Regularly:** Keep OpenWork updated to latest version
2. **Trust but Verify:** Only import workspaces from trusted sources
3. **Principle of Least Access:** Only authorize directories you need to work with
4. **Monitor Activity:** Check scheduled jobs and active processes periodically
5. **Report Issues:** If something seems suspicious, report it

### As a Contributor

1. **Security-First Mindset:** Consider security implications of all changes
2. **Code Review:** All security-sensitive code requires thorough review
3. **Testing:** Add tests for security features
4. **Documentation:** Document security assumptions and requirements
5. **Stay Informed:** Keep up with Tauri security advisories

## Security Roadmap

Future security enhancements planned:
- [ ] Implement dependency vulnerability scanning in CI
- [ ] Add comprehensive security test suite
- [ ] Enable CSP with appropriate configuration
- [ ] Implement security audit logging
- [ ] Add sandboxing for spawned processes
- [ ] Regular third-party security audits

## Conclusion

OpenWork has a reasonable security foundation with good input validation and filesystem access controls. The main concerns are:
1. Disabled CSP (can be fixed easily)
2. Broad network permissions (requires careful configuration)
3. No automated dependency scanning (can be added to CI)

With the recommended fixes implemented, OpenWork is safe for personal and professional use, especially when working with trusted content.

---

**Last Updated:** February 4, 2026  
**Next Review:** August 2026
