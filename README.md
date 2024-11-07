# Grip - GitHub Release Installer & Package Manager

Grip is a fast, user-friendly package manager that installs packages directly from GitHub releases. It uses a registry system to map package names to GitHub repositories and automatically adds installed binaries to your PATH.

## Features

- 🚀 Direct installation from GitHub releases
- 📦 Multiple registry support with priority system
- 🔄 Automatic PATH management
- 💻 Cross-platform (Windows, macOS, Linux)
- 🎨 Interactive UI with progress bars
- 📋 Version selection for packages
- 🔍 Smart asset selection based on platform

## Installation

### From Source
```bash
git clone https://github.com/yourusername/grip
cd grip
cargo install --path .
```

### Using Cargo
```bash
cargo install grip
```

## Usage

### Installing Packages
```bash
# Install latest version
grip install ripgrep

# Install specific version
grip install bat --version v0.22.1

# Install specific asset
grip install delta --asset delta-0.16.5-x86_64-pc-windows-msvc.zip
```

### Managing Registries
```bash
# List configured registries
grip registry list

# Add a custom registry
grip registry add custom github.com/user/registry --priority 200

# Remove a registry
grip registry remove custom
```

## Registry Format

A Grip registry is a GitHub repository with the following structure:
```
registry/
├── README.md
├── packages/
│   ├── ripgrep.json
│   ├── bat.json
│   └── ...
└── templates/
    └── install.json
```

### Package Definition (packages/example.json)
```json
{
  "name": "ripgrep",
  "description": "Fast line-oriented search tool",
  "repository": "BurntSushi/ripgrep",
  "homepage": "https://github.com/BurntSushi/ripgrep",
  "tags": ["search", "grep", "cli"]
}
```

### Install.json example
```json
{
    "name": "example-tool",
    "version": "1.0.0",
    "commands": [
        {
            "platform": "windows",
            "command": "move",
            "args": ["example.exe", "%USERPROFILE%\\bin"]
        },
        {
            "platform": "unix",
            "command": "chmod",
            "args": ["+x", "example"]
        },
        {
            "platform": "unix",
            "command": "mv",
            "args": ["example", "$HOME/bin"]
        }
    ],
    "paths": [
        "$HOME/bin",
        "$HOME/.local/bin"
    ]
}
```

## Features in Detail

### Multiple Registries
- Use multiple package registries with priority ordering
- Higher priority registries are checked first
- Registry contents are cached locally
- Automatic updates on package installation

### Smart PATH Management
- Windows: Automatically updates system PATH through registry
- Unix: Updates shell configuration (.bashrc, .zshrc, .profile)
- Creates necessary directories and symlinks

### Version Management
- Interactive version selection from available releases
- Specific version installation via --version flag
- Platform-specific asset selection

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Creating a Registry

1. Create a new GitHub repository
2. Add package definitions in the `packages` directory
3. Each package should have its own JSON file
4. Optional: Add installation templates

### Example Registry Structure
```
your-registry/
├── README.md
├── packages/
│   ├── awesome-tool.json
│   └── cool-app.json
└── templates/
    └── install.json
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with Rust 🦀
- Uses GitHub's API for release management

## Technical Details

### Dependencies
- clap: Command line argument parsing
- tokio: Async runtime
- reqwest: HTTP client
- serde: JSON serialization
- colored: Terminal colors
- indicatif: Progress bars
- zip: Archive extraction

### Platform Support
- Windows
- macOS
- Linux
- Other Unix-like systems (partial support)

## FAQ

**Q: How is this different from cargo install?**
A: Grip installs any binary from GitHub releases, not just Rust packages. It also manages PATH and supports multiple registries.

**Q: Can I use private repositories?**
A: Yes, by setting up authentication through git's environment variables or configuration file.

**Q: How do updates work?**
A: Grip checks for updates when you install packages and maintains a local cache of registry contents.
