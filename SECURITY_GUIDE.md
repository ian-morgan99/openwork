# How to Keep OpenWork Safe on Your PC

## Quick Safety Checklist

✅ **Before Using OpenWork:**
- [ ] Only download from official sources (GitHub releases)
- [ ] Verify the installer signature (on macOS)
- [ ] Review what directories you give OpenWork access to
- [ ] Keep OpenWork updated to the latest version

✅ **While Using OpenWork:**
- [ ] Only import workspaces from people you trust
- [ ] Review authorized roots periodically
- [ ] Be cautious about running tasks from unknown sources
- [ ] Monitor scheduled tasks in Settings

## What OpenWork Can Access

OpenWork has been designed with security in mind, but it's important to understand what it can do:

### ✅ What OpenWork Can Do

1. **Read and write files** in directories you explicitly authorize (called "authorized roots")
2. **Connect to the internet** to update itself and communicate with AI services
3. **Run system commands** as needed for development tasks (like git, npm, etc.)
4. **Access your OpenCode configuration** and workspaces

### ❌ What OpenWork Cannot Do (Without Your Permission)

1. **Access files outside authorized roots** - All filesystem operations are restricted
2. **Run arbitrary code** without your interaction
3. **Send data** without your knowledge (check network connections if concerned)
4. **Modify system files** outside your user directory

## Setting Up Safely

### 1. Authorized Roots

**What are they?** Authorized roots are directories where OpenWork is allowed to read and write files.

**How to set them up safely:**
```
✅ Good Authorized Roots:
   - ~/Documents/my-project
   - ~/Code/openwork-workspace
   - ~/Desktop/work-folder

❌ Risky Authorized Roots:
   - ~ (your entire home directory)
   - / (your entire system)
   - /System (system folders)
   - ~/.ssh (sensitive credentials)
```

**Recommendation:** Create a dedicated directory for OpenWork projects (e.g., `~/OpenWork`) and only authorize that.

### 2. Workspace Safety

**When creating a workspace:**
- Use a new, empty directory
- Don't authorize directories containing sensitive files
- Review the workspace configuration before importing

**When importing a workspace:**
- Only import from trusted sources
- Review the `.opencode/openwork.json` file first
- Check for suspicious scheduled tasks
- Verify authorized roots make sense

### 3. Network Safety

OpenWork can connect to:
- GitHub for updates and repositories
- AI service providers (configured in OpenCode)
- Local development servers (localhost)
- Any HTTP/HTTPS endpoint

**What to watch for:**
- Unusual network activity
- Connections to unfamiliar domains
- High bandwidth usage

## Red Flags: When to Be Concerned

🚩 **Stop and investigate if:**
- OpenWork asks for access to system directories (/, /System, /usr)
- You're asked to import a workspace from an untrusted source
- Scheduled tasks appear that you didn't create
- You see unexpected network connections
- Files outside authorized roots are modified

## Security Features in This Version

OpenWork includes several security protections:

### ✅ Path Traversal Protection
Prevents malicious workspaces from accessing files outside authorized directories.

### ✅ Input Sanitization
All user inputs (command names, package names) are validated to prevent injection attacks.

### ✅ Zip File Validation
Imported workspace archives are checked for malicious paths.

### ✅ Content Security Policy
The UI is protected against cross-site scripting (XSS) attacks.

### ✅ Secure Process Spawning
External commands are executed safely without shell interpretation.

## Best Practices

### For Personal Use

1. **Keep it updated**: Enable auto-updates to get security patches
2. **Limit access**: Only authorize directories you're actively working on
3. **Review regularly**: Check Settings → Scheduled Jobs periodically
4. **Trust your sources**: Only import skills and workspaces from trusted developers
5. **Monitor activity**: Keep an eye on what OpenWork is doing

### For Professional/Enterprise Use

1. **Network isolation**: Run OpenWork on networks with appropriate firewall rules
2. **Access control**: Don't authorize company-wide shared drives
3. **Audit logs**: Review OpenWork activity regularly
4. **Update policy**: Establish a process for vetting updates
5. **Incident response**: Have a plan for security concerns

## Common Questions

### Q: Is it safe to give OpenWork access to my entire home directory?
**A:** No, it's safer to create a dedicated directory for OpenWork projects. This follows the principle of least privilege - only give access to what's needed.

### Q: Can OpenWork steal my passwords or SSH keys?
**A:** OpenWork doesn't actively seek out sensitive files, but if you authorize a directory containing them (like `~/.ssh`), they would be accessible. Don't authorize directories with sensitive data.

### Q: How do I know if a workspace is safe to import?
**A:** 
1. Check the source - do you trust who created it?
2. Review the `.opencode/openwork.json` file
3. Look for suspicious authorized roots
4. Check for scheduled tasks
5. When in doubt, don't import it

### Q: What should I do if I suspect a security issue?
**A:**
1. Stop using OpenWork immediately
2. Disconnect from the network if concerned about data exfiltration
3. Review OpenWork logs and activity
4. Report the issue to the OpenWork team (see SECURITY.md)
5. Consider restoring from a backup

### Q: Is OpenWork safe for sensitive projects?
**A:** OpenWork is as safe as the workspaces and configurations you use. For highly sensitive projects:
- Use a dedicated, isolated machine
- Review all configurations carefully
- Audit dependencies and updates
- Consider enterprise security solutions

## Technical Security Details

For developers and security professionals, see `SECURITY.md` for:
- Detailed security architecture
- Vulnerability assessment
- Security testing procedures
- Responsible disclosure policy

## Updates and Patches

OpenWork receives regular security updates. To stay protected:

1. **Enable auto-updates** (recommended)
2. **Check for updates manually** at least monthly
3. **Review release notes** for security fixes
4. **Subscribe to security announcements** on GitHub

## Support and Questions

If you have security concerns or questions:
- 📖 Read the full security documentation: `SECURITY.md`
- 🐛 Report security issues: [See SECURITY.md for contact info]
- 💬 General questions: GitHub Discussions
- 📧 Security-sensitive issues: Private disclosure preferred

## Summary

OpenWork is designed to be safe for everyday use when you follow these guidelines:

✅ **Safe practices:**
- Use dedicated project directories
- Only import from trusted sources
- Keep updated
- Review permissions regularly

❌ **Risky practices:**
- Authorizing entire home directory
- Importing unknown workspaces
- Ignoring update notifications
- Running with admin/root privileges

**Remember:** Security is a shared responsibility. OpenWork provides the tools and protections, but you need to use them wisely.

---

**Last updated:** February 2026  
**For issues:** See SECURITY.md for reporting process
