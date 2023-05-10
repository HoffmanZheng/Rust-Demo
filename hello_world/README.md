# Hello world

### Install Rust on MacOS or Linux

1. `$ curl https://sh.rustup.rs -sSf | sh` to install Rust.

2. `$ source $HOME/.cargo/env` to enable the configuration in the current terminal without reopenning.

3. `rustup doc` to open the local documentation in the browser.

### Common sence of Rust

1. Rust style is to indent with four spaces, not a tab.

2. Methods end with exclamation mark call a Rust macro instead of a function.

3. Compiling and running are separate steps in Rust, `rustc main.rs` outputs a binary executable file on MacOS and Linux. With CMD on windows, a debug information containing `main.pdb` would be generated addtionally.

4. Rust is an ahead-of-time compiled language, the compiled executable could be given to someone else, and run without having Rust installed. 
