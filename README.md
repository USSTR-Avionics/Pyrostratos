# Installation

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`  

`rustup target add thumbv7em-none-eabihf`  

`sudo apt install build-essential`  

`cargo install cargo-binutils`  

`rustup component add llvm-tools-preview`  

`sudo apt install libusb-1.0-0-dev`  

`cargo install cargo-flash`  


# Compiling 

`cargo build --release`  


# Flashing

`cargo flash --chip STM32F303K8T6x --release`  

