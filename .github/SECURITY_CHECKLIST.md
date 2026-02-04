# Security Checklist for OpenWork Contributors

This checklist helps ensure that all code changes maintain OpenWork's security posture.

## Before Submitting a PR

### Code Review Checklist

#### Input Validation
- [ ] All user inputs are validated before use
- [ ] Path inputs are canonicalized and checked against authorized roots
- [ ] Command arguments don't allow shell injection
- [ ] File paths are checked for traversal attempts (`..`, absolute paths)
- [ ] String inputs have length limits
- [ ] Special characters are properly escaped or rejected

#### Tauri IPC Commands
- [ ] New commands have appropriate parameter validation
- [ ] Sensitive operations require explicit user authorization
- [ ] Error messages don't leak sensitive information
- [ ] Commands follow principle of least privilege

#### File System Operations
- [ ] All file operations are within authorized roots
- [ ] Paths are canonicalized before security checks
- [ ] Symlinks are handled safely (canonicalize first)
- [ ] File permissions are checked appropriately
- [ ] Temporary files are created securely

#### Process Spawning
- [ ] Arguments are passed as separate parameters (not shell-interpreted)
- [ ] Working directory is validated
- [ ] Environment variables are sanitized
- [ ] Process output is handled safely (no buffer overflows)
- [ ] Processes have appropriate timeouts

#### Network Operations
- [ ] URLs are validated (protocol, domain)
- [ ] TLS/SSL is used for sensitive communications
- [ ] Certificates are properly validated
- [ ] Network errors don't leak sensitive data
- [ ] Rate limiting is considered for external APIs

#### Data Handling
- [ ] Sensitive data is not logged
- [ ] User data is not transmitted without consent
- [ ] Secrets are not committed to version control
- [ ] Configuration files don't contain hardcoded credentials
- [ ] Sensitive data in memory is cleared when done

#### Dependencies
- [ ] New dependencies are from trusted sources
- [ ] Dependency versions are pinned or have safe ranges
- [ ] Transitive dependencies are reviewed
- [ ] Known vulnerabilities are avoided
- [ ] Licenses are compatible

## Security Testing

### Manual Testing
- [ ] Test with malicious inputs (XSS, SQL injection, path traversal)
- [ ] Test with edge cases (empty strings, very long strings, special characters)
- [ ] Test authorization bypasses
- [ ] Test race conditions
- [ ] Test with restricted permissions

### Automated Testing
- [ ] Unit tests cover security-critical code
- [ ] Integration tests verify authorization checks
- [ ] Property-based tests for parsers/validators
- [ ] Fuzzing for input handling (if applicable)

## Code Patterns to Avoid

### ❌ Don't Do This

```rust
// Shell injection risk
Command::new("sh").arg("-c").arg(user_input)

// Path traversal risk
let path = format!("/base/{}", user_input);

// SQL injection (if using SQL)
format!("SELECT * FROM users WHERE name = '{}'", name)

// Unvalidated redirect
window.location = user_input;

// Hardcoded secrets
const API_KEY = "sk-1234567890abcdef";
```

### ✅ Do This Instead

```rust
// Safe command execution
Command::new("program").arg(validated_input)

// Safe path construction
let base = PathBuf::from("/base");
let path = base.join(sanitized_input);
let canonical = fs::canonicalize(&path)?;
if !canonical.starts_with(&base) {
    return Err("Path traversal attempt");
}

// Parameterized queries (if using SQL)
db.execute("SELECT * FROM users WHERE name = ?", &[name])

// Validated redirect
if url.starts_with("https://trusted.com/") {
    window.location = url;
}

// Environment variables or secure config
let api_key = env::var("API_KEY")?;
```

## Specific Areas of Concern

### 1. Workspace Import (`workspace_import_config`)
**Risks:** Malicious workspace configs could escape authorized roots

**Checks:**
- [ ] Validate all paths in imported config
- [ ] Check for path traversal in zip entries
- [ ] Verify authorized roots are within expected bounds
- [ ] Sanitize skill and command names

### 2. Skill Import (`import_skill`)
**Risks:** Importing from unauthorized locations

**Checks:**
- [ ] Verify source directory is within authorized roots
- [ ] Check destination doesn't escape workspace
- [ ] Validate skill structure
- [ ] Scan for suspicious files

### 3. Package Installation (`opkg_install`)
**Risks:** Command injection through package names

**Checks:**
- [ ] Validate package name format
- [ ] Use argument array (not shell string)
- [ ] Check for suspicious characters
- [ ] Verify package source if possible

### 4. Command Execution (`opencode_mcp_auth`, scheduler commands)
**Risks:** Arbitrary command execution

**Checks:**
- [ ] Validate all command arguments
- [ ] Use restricted command set
- [ ] Run with minimal privileges
- [ ] Sanitize environment

### 5. Remote Workspace (`workspace_create_remote`)
**Risks:** SSRF, credential theft

**Checks:**
- [ ] Validate URL format
- [ ] Restrict to HTTP/HTTPS
- [ ] No file:// or other protocols
- [ ] Validate credentials securely

## Security Review Process

### For All PRs
1. Self-review using this checklist
2. Add security label if touching sensitive code
3. Document security implications in PR description

### For Security-Sensitive PRs
Additional requirements:
- [ ] Security-focused code review by maintainer
- [ ] Manual security testing performed
- [ ] Regression tests added
- [ ] Security.md updated if needed

### Security-Sensitive Areas
Mark PR with `security` label if changing:
- Tauri IPC command handlers
- File system operations
- Process spawning
- Workspace import/export
- Authorization checks
- Input validation
- Crypto/authentication

## Common Vulnerabilities in Tauri Apps

### 1. Command Injection
```rust
// Bad
Command::new("sh").arg("-c").arg(format!("echo {}", user_input))

// Good
Command::new("echo").arg(user_input)
```

### 2. Path Traversal
```rust
// Bad
let path = base_dir + "/" + user_path;

// Good
let path = PathBuf::from(base_dir).join(user_path);
let canonical = fs::canonicalize(&path)?;
if !canonical.starts_with(base_dir) {
    return Err("Invalid path");
}
```

### 3. XSS in WebView
```rust
// Bad (in frontend)
element.innerHTML = user_input;

// Good
element.textContent = user_input;
// or use a sanitization library
```

### 4. SSRF
```rust
// Bad
let response = http::get(&user_url)?;

// Good
if !is_safe_url(&user_url) {
    return Err("Invalid URL");
}
let response = http::get(&user_url)?;
```

### 5. Arbitrary File Read
```rust
// Bad
fs::read_to_string(&user_path)?

// Good
let canonical = fs::canonicalize(&user_path)?;
if is_within_authorized_roots(&canonical) {
    fs::read_to_string(&canonical)?
}
```

## Resources

### Security Documentation
- [SECURITY.md](./SECURITY.md) - Security architecture and vulnerabilities
- [SECURITY_GUIDE.md](./SECURITY_GUIDE.md) - User safety guide
- [Tauri Security Guide](https://tauri.app/v1/guides/security/)

### Tools
- `cargo audit` - Rust dependency vulnerabilities
- `pnpm audit` - npm dependency vulnerabilities
- `cargo clippy` - Rust linting (includes security checks)
- `semgrep` - Static analysis (optional)

### Best Practices
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)

## Questions?

If you're unsure about security implications:
1. Ask in the PR comments
2. Tag a maintainer with security expertise
3. Better to ask than to introduce a vulnerability

## Remember

**Security is not optional.** A single vulnerability can compromise all users. When in doubt:
- Ask for review
- Err on the side of caution
- Follow the principle of least privilege
- Document your security assumptions

---

**Last updated:** February 2026  
**Maintained by:** OpenWork Security Team
