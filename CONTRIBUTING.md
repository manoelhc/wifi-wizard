# Contributing to WiFi Wizard

Thank you for your interest in contributing to WiFi Wizard! This document provides guidelines and information for developers.

## Development Setup

### Prerequisites
1. **Rust toolchain** (1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **System dependencies** (see INSTALL.md for your distribution)

3. **NetworkManager** running on your system
   ```bash
   sudo systemctl status NetworkManager
   ```

### Building for Development
```bash
# Clone the repository
git clone <repository-url>
cd wifi-wizard

# Build in debug mode (faster compilation)
cargo build

# Run the application
cargo run

# Build in release mode (optimized)
cargo build --release
```

### Code Structure
```
wifi-wizard/
├── src/
│   ├── main.rs              # Application entry point
│   ├── network_manager.rs   # D-Bus client for NetworkManager
│   └── ui.rs                # GTK4 user interface
├── Cargo.toml               # Rust dependencies
├── README.md                # User documentation
├── INSTALL.md               # Installation guide
├── SECURITY.md              # Security documentation
└── CONTRIBUTING.md          # This file
```

## Development Guidelines

### Code Style
- Follow Rust standard formatting: `cargo fmt`
- Check for common mistakes: `cargo clippy`
- Write clear, self-documenting code
- Add comments for complex logic

### Testing
```bash
# Check code without building
cargo check

# Run tests (if available)
cargo test

# Check for compilation warnings
cargo clippy
```

### Making Changes

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes**
   - Keep changes focused and atomic
   - Follow existing code patterns
   - Update documentation as needed

3. **Test your changes**
   - Build and run the application
   - Test all modified functionality
   - Verify no regressions in existing features

4. **Format and lint**
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```

5. **Commit your changes**
   ```bash
   git add .
   git commit -m "Brief description of changes"
   ```

## Feature Requests and Bug Reports

### Reporting Bugs
When reporting bugs, please include:
- Operating system and version
- NetworkManager version (`nmcli --version`)
- GTK4 version
- Steps to reproduce
- Expected vs actual behavior
- Error messages or logs

### Requesting Features
For feature requests, please describe:
- The use case
- Expected behavior
- Why it would be useful
- Any implementation suggestions

## Code Review Process

All contributions will be reviewed for:
- Code quality and style
- Functionality and correctness
- Performance implications
- Security considerations
- Documentation completeness

## Development Tips

### Debugging GTK4 Applications
```bash
# Enable GTK debug output
GTK_DEBUG=interactive cargo run

# Enable GLib debug messages
G_MESSAGES_DEBUG=all cargo run
```

### Testing D-Bus Communication
```bash
# Monitor D-Bus traffic
dbus-monitor --system "interface='org.freedesktop.NetworkManager'"

# List NetworkManager D-Bus objects
busctl tree org.freedesktop.NetworkManager

# Introspect NetworkManager
busctl introspect org.freedesktop.NetworkManager /org/freedesktop/NetworkManager
```

### Common Development Tasks

#### Adding a New UI Component
1. Add GTK widget creation in `src/ui.rs`
2. Connect event handlers
3. Update layout and styling
4. Test responsiveness

#### Adding NetworkManager Functionality
1. Define D-Bus interface in `src/network_manager.rs`
2. Create proxy methods
3. Add error handling
4. Test with actual NetworkManager

#### Updating Dependencies
```bash
# Check for outdated dependencies
cargo outdated

# Update dependencies
cargo update

# Update specific dependency
cargo update <crate-name>
```

## Architecture Notes

### Asynchronous Design
- Uses `tokio` for async runtime
- GTK4 integration via `glib::spawn_future_local`
- NetworkManager operations are non-blocking

### Error Handling
- Uses `anyhow::Result` for error propagation
- Provides user-friendly error messages
- Logs errors for debugging

### State Management
- Minimal shared state using `Rc<RefCell<>>`
- GTK signal handlers for UI updates
- Event-driven architecture

## Security Considerations

When contributing, please:
- Never log or print sensitive data (passwords)
- Validate all user inputs
- Use secure D-Bus communication
- Follow principle of least privilege
- Consider security implications of changes
- Update SECURITY.md if needed

## Documentation

Please update documentation when:
- Adding new features
- Changing user-facing behavior
- Modifying build/install process
- Fixing bugs that affect usage

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

## Questions?

If you have questions about development, feel free to:
- Open an issue for discussion
- Check existing issues and pull requests
- Review the code and documentation

Thank you for contributing to WiFi Wizard!
