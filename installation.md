# Debian | Ubuntu | Arch based distributions

`git clone https://github.com/aenoshrajora/VulneraX`

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

`source $HOME/.cargo/env`

`cd VulneraX`

Debian/Ubuntu based distributions: `sudo apt install openssl-dev`

Arch based distrobutions: `sudo pacman -S openssl`

`cargo build --release`

`mv target/release/VulneraX .`

# Windows 10 (won't give you the best experience)

Go to the follwoing link to install Rust https://turreta.com/2019/09/06/how-to-install-rust-on-windows-10/

Then download the zip https://github.com/aenoshrajora/VulneraX/archive/master.zip and extract it

Go to the VulneraX directory and use do `cargo build --release`

The executable will be located at `target/release/VulneraX.exe`
