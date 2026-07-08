# APM (Any Package Manager)

**APM** is a lightweight, universal wrapper for Linux package managers. It provides a unified command-line interface to perform common package management tasks across different distributions, such as Arch Linux and Debian/Ubuntu-based systems.

## Features
* **Unified Interface**: Use the same simple commands (`-S`, `-R`, `-U`) regardless of your distribution.
* **Cross-Distro Support**: Automatically detects the system's native package manager (`pacman`/`yay` for Arch-based, `apt` for Debian-based).
* **Safety First**: Requires interactive confirmation before performing system-wide changes.
* **Performance**: Built with Rust, ensuring a fast, memory-safe, and lightweight experience.

## Usage

| Action | Command |
| :--- | :--- |
| **Install a package** | `apm -S <package_name>` |
| **Remove a package** | `apm -R <package_name>` |
| **Upgrade the system** | `apm -U` |

## Installation

### From Source
1. **Clone the repository**:
   ```bash
   git clone [https://github.com/selc0m/APM---Any-pkg-manager.git](https://github.com/selc0m/APM---Any-pkg-manager.git)
   cd APM---Any-pkg-manager
