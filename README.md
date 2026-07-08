# APM (Advanced Package Manager)

**APM** is a lightweight, universal wrapper for Linux package managers. It provides a unified command-line interface to perform common package management tasks across different distributions, such as Arch Linux and Debian/Ubuntu-based systems.

## Features
*   **Unified Interface**: Use the same simple commands (`-S`, `-R`, `-U`) regardless of your distribution.
*   **Cross-Distro Support**: Automatically detects the system's native package manager (`pacman`/`yay` for Arch-based, `apt` for Debian-based).
*   **Safety First**: Requires interactive confirmation before performing system-wide changes.
*   **Performance**: Built with Rust, ensuring a fast, memory-safe, and lightweight experience.

## Usage

| Action | Command |
| :--- | :--- |
| **Install a package** | `apm -S <package_name>` |
| **Remove a package** | `apm -R <package_name>` |
| **Upgrade the system** | `apm -U` |

## Installation

### From Source
1.  **Clone the repository**:
    ```bash
    git clone [https://github.com/yourusername/apm.git](https://github.com/yourusername/apm.git)
    cd apm
    ```
2.  **Build the project**:
    ```bash
    cargo build --release
    ```
3.  **Install the binary**:
    ```bash
    sudo cp target/release/apm /usr/local/bin/
    ```

## License
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
