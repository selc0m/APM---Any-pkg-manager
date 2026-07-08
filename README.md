### APM (Advanced Package Manager) v1.0.0 - Initial Release

Welcome to the first stable release of **APM**! 

APM is a lightweight, universal wrapper for Linux package managers. It provides a unified command-line interface to perform common package management tasks across different distributions.

#### **What's New in this Release**
*   **Unified CLI Interface**: Standardized commands (`-S`, `-R`, `-U`) for all supported systems.
*   **Intelligent Auto-detection**: Automatically identifies and utilizes the system's native package manager (`pacman`/`yay` for Arch-based, `apt` for Debian-based).
*   **Safety Features**: Added interactive confirmation prompts for all system-wide operations to prevent accidental changes.
*   **High Performance**: Built with Rust for optimal speed and minimal memory footprint.

#### **Supported Operations**
*   **Install**: `apm -S <package_name>`
*   **Remove**: `apm -R <package_name>`
*   **Upgrade**: `apm -U` (system-wide upgrade)

#### **Installation**
You can build APM from source:
1. Clone the repository: `git clone https://github.com/selc0m/APM---Any-pkg-manager.git`
2. Build the binary: `cargo build --release`
3. Install: `sudo cp target/release/apm /usr/local/bin/`
