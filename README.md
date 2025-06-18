# Space Strategy Game Engine 🚀

[![Rust Version](https://img.shields.io/badge/rust-2024-blue.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Project Status](https://img.shields.io/badge/status-alpha-orange.svg)]()

A Rust-based space strategy game engine that simulates faction-based colonization and management across solar systems. This engine provides a robust foundation for building complex space strategy games with deep simulation mechanics.

## 🌟 Features

- **Universe Generation**: Procedurally generated solar systems with unique celestial bodies
- **Faction Management**: Complex faction dynamics with population growth and relationships
- **Resource Systems**: Deep economic simulation with production chains and trade networks
- **Building Systems**: Infrastructure and construction management
- **Strategic Map**: Hex-based strategic map for movement and positioning
- **Event System**: Comprehensive event handling and simulation

## 🚀 Getting Started

### Prerequisites

- Rust 2024 edition
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/cyrex562/harsh_realm.git
cd harsh_realm
```

2. Build the project:
```bash
cargo build
```

3. Run the tests:
```bash
cargo test
```

## 📚 Documentation

Detailed documentation is available in the `docs/` directory:
- [Game Mechanics](docs/game_mechanics.md)
- [Faction Structures](docs/faction_structures.md)
- [Development Guide](docs/development/README.md)
- [Design Documents](docs/design/README.md)

## 🛠️ Project Structure

```
harsh_realm/
├── src/           # Source code
├── docs/          # Documentation
│   ├── game_mechanics/    # Game mechanics documentation
│   ├── diagrams/          # System diagrams
│   ├── development/       # Development guides
│   ├── design/           # Design documents
│   └── components/       # Component documentation
├── Cargo.toml     # Project dependencies
└── README.md      # This file
```

## 🎮 Game Systems

### Core Systems
- Turn-based simulation engine
- Resource management and production
- Population and faction dynamics
- Building and infrastructure management
- Strategic map system
- Event handling and processing

### Future Systems
- Combat mechanics
- Research and technology trees
- Advanced space travel
- Enhanced diplomacy systems

## 📦 Version Management

This project follows [Semantic Versioning](https://semver.org/) (MAJOR.MINOR.PATCH):

- **MAJOR** version for incompatible API changes
- **MINOR** version for backwards-compatible functionality
- **PATCH** version for backwards-compatible bug fixes

### Automatic Version Updates

- The patch version is automatically incremented on each push
- Use the following commands to manually update versions:

```bash
# Bump major version (1.0.0 -> 2.0.0)
python3 scripts/version_manager.py major

# Bump minor version (1.0.0 -> 1.1.0)
python3 scripts/version_manager.py minor

# Bump patch version (1.0.0 -> 1.0.1)
python3 scripts/version_manager.py patch
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Uses [nalgebra](https://nalgebra.org/) for mathematical operations
- [hex2d](https://crates.io/crates/hex2d) for hex grid management
