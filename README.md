# Installation

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  

`rustup target add thumbv7em-none-eabihf`  

`sudo apt install build-essential`  

`cargo install cargo-binutils`  

`rustup component add llvm-tools-preview`  

`sudo apt install libusb-1.0-0-dev`  

`cargo install sccache --locked`

`cargo install flip-link`

`cargo install cargo-flash`  

`cargo install cargo-make`

`cargo install cargo-watch`

`cargo install bacon`

# Developing

Automatically compile on save, useful when dealing with compiler bugs  

`cargo watch -x run`

`bacon`

# Compiling 

`cargo make build`

> NOTE: Builds in debug mode for faster compilation times

To ensure caching, replace the `rustc-wrapper` path in `.cargo/config.toml` to 
the appropriate **absolute** path.


# Flashing

`cargo make flash`

> This will also compile the code in **release** mode

# Running

Have two terminal instances accessible

`cargo make openocd`

opens an openocd session, this is where your serial prints show up

`cargo make run`

this runs the runner and attaches the gdb session, press 'c' to continue on the gdb session


