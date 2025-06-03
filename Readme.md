
<br/>
<div align="center">
<a href="https://github.com/ShaanCoding/ReadME-Generator">
<img src="https://cdn-icons-png.flaticon.com/512/1208/1208881.png" alt="Logo" width="80" height="80">
</a>
<h3 align="center">RustCrypter</h3>
<p align="center">
AES-256-CBC file crypter written in Rust

<br/>
<br/>
  
<a href="https://github.com/ShaanCoding/ReadME-Generator/issues/new?labels=bug&template=bug-report---.md">Report Bug</a>

</p>
</div>

## About The Project

This is a simple cli program that can help you encrypt some files using AES CBC mode with 256 bit key and 128 bit IV (Initialization Vector). This program can encrypt all types of files, including raw txt files, images and binaries. 
### Built With

This project uses cross-rs for cross compilation for different systems and openssl-rust as the rust implementation of libssl, which provides different encryption algorithms.

- [cross-rs](https://github.com/cross-rs/cross)
- [openssl-rust](https://github.com/sfackler/rust-openssl)
### Prerequisites

To build this program from source, you first need to install the rust toolchain and compiler. You can install it on UNIX-like systems using
- Linux/MacOS
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
On Windows, you can just download and run [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)
### Installation

 To build it for your system, you can just use 
  ```sh
  cargo build --release
  ```
If you want to cross compile it for a different system or arch, you can use the [cross-rs](https://github.com/cross-rs/cross)
## Usage

This program has a very simple cli flags to work, that are listed below. Please make sure that you dont put both encrypt and decrypt at the same time <br>
- **-e | --encrypt** — sets program mode to encryption
- **-d  | --decrypt** — sets program mode to decryption
- **-o | --output** — optional, sets output file name/path

Basic usage is 
   ```sh
   ./rustcrypter -e (--encrypt) -d (--decrypt) [-o outfilename] <inputfilename>
   ```
## Roadmap

- [x] Add error handling
- [ ] Add readable comments
- [ ] Optimize code
- [ ] Add recursive folder handling

## Acknowledgments

There are some resources that helped me in this project and can also provide additional information about this code

- [makeread.me](https://github.com/ShaanCoding/ReadME-Generator)
- [rust-docs](https://doc.rust-lang.org/book/ch01-00-getting-started.html)
- [openssl-rust](https://github.com/sfackler/rust-openssl)
- [cross-rs](https://github.com/cross-rs/cross)
- [rand](https://github.com/rust-random/rand)

## License

Distributed under the DFC License. See [DFC License](https://github.com/xNOTHlNGx/DFC-license/blob/main/LICENSE) for more information.
## Contact

- Telegram: [@xNOTHlNGx](https://t.me/xNOTHlNGx) 
- Email: [not_a_nothing@proton.me](mailto:not_a_nothing@proton.me)

Project Link: [https://github.com/xNOTHlNGx/rustcrypter](https://github.com/xNOTHlNGx/rustcrypter)

