# Installation

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  

`rustup target add thumbv7em-none-eabihf`  

`sudo apt install build-essential`  

`cargo install cargo-binutils`  

`rustup component add llvm-tools-preview`  

`sudo apt install libusb-1.0-0-dev`  

`cargo install cargo-flash`  

`cargo install cargo-make`

# Compiling 

`cargo make build`

> NOTE: Builds in debug mode for faster compilation times


# Flashing

`cargo make flash`

> This will also compile the code in **release** mode

# Running

Have two terminal instances accessible

`cargo make openocd`

opens an openocd session, this is where your serial prints show up

`cargo make run`

this runs the runner and attaches the gdb session, press 'c' to continue on the gdb session


