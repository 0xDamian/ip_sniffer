
## Overview
This is a simple port scanning tool written in Rust to identify open TCP ports on a target IP address. 

## Installation
1. Ensure Rust is installed (Linux):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
   Other installation methods depending on OS [here](https://forge.rust-lang.org/infra/other-installation-methods.html):
2. Clone the repository:
   ```bash
   git clone https://github.com/0xDamian/ip_sniffer.git
   cd ip_sniffer
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Usage
You can run it without having to build the binary.
```bash
cargo run -- <Options>
```
### Options
- `-j <NUMBER>`: Specify the number of threads (e.g., `-j 8`).
- `-h` or `--help`: Display help message.
- `<IP_ADDRESS>`: Target IPv4 or IPv6 address.
### E.g:
```bash
cargo run -- -j 1000 127.0.0.1
cargo run -- -h
cargo run -- 127.0.0.1
```
<img width="869" height="350" alt="image" src="https://github.com/user-attachments/assets/f7cbab44-f6dc-40d9-89cc-6bb4ff058743" />


## Requirements
- Rust 1.50+ (includes standard library for networking).
- Compatible OS: Cross-platform; Linux, macOS, or Windows.

## License
This project is licensed under the MIT License - basically, have fun with it.
