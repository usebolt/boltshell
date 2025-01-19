# Bolt Shell

**Bolt is a terminal shell made purely in Rust**

---

## Introduction

Bolt Shell is a modern, lightweight, and efficient terminal shell designed for developers and users who value performance and simplicity. Built entirely in Rust, Bolt leverages Rust's safety and concurrency features to deliver a reliable and fast command-line experience.

## Key Features

- **Fast and Lightweight**: Optimized for speed and minimal resource usage.
- **Cross-platform support**: Works for Windows, and Linux (with Mac testing ongoing)
- **User-Friendly**: Includes commands suitable for beginners and power users alike.
- **Safety First**: Built with Rust to minimize crashes and memory-related bugs.

## Installation

### Via Cargo
1. Simply run the following command:
```powershell
cargo install boltshell
```

### From Source
1. Ensure you have Rust installed on your system. If not, download it from [rust-lang.org](https://www.rust-lang.org/).
2. Clone the repository:
   ```powershell
   git clone https://github.com/usebolt/boltshell.git
   cd boltshell
   ```
3. Build the project:
   ```powershell
   cargo build --release
   ```
4. Run the shell:
   ```powershell
   .\target\release\boltshell.exe
   ```

### Precompiled Binaries
Precompiled binaries will be available soon for easier installation. Stay tuned!

## Usage

Once installed, launch Bolt Shell by running the following command:
```powershell
boltshell
```

### Basic Commands
- Navigate directories: `cd`
- List files: `ls`
- Execute programs: `program_name [arguments]`
- Exit the shell: `exit`

### More Commands
Listed when running `help`

### Prompt
Bolt Shell features a dynamic prompt displaying the current user, hostname, and working directory.

## Contributing

Contributions are welcome! To contribute:
1. Fork the repository.
2. Create a new branch for your feature or bug fix:
   ```powershell
   git checkout -b feature-name
   ```
3. Commit your changes:
   ```powershell
   git commit -m "Add feature-name"
   ```
4. Push to your branch:
   ```powershell
   git push origin feature-name
   ```
5. Open a pull request.

## License

Bolt Shell is released under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Acknowledgments

Special thanks to the Rust community for providing incredible tools and resources, and to all contributors who have helped shape Bolt Shell.

---

**Start using Bolt Shell today and experience the power of Rust in your terminal!**

Please note that the crates.io readme is outdated.
