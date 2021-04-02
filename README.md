# Smartgarden Hardware Hub
![example workflow](https://github.com/lucaired/smartgarden-hardware-control/actions/workflows/rust.yml/badge.svg)

You will need:
- A Raspberry Pi with USB port and networking capabilities, i'm using a model 3 B V1.2, but feel free to check out the other [Pis](https://www.raspberrypi.org/products/).

## Setup Rust on a Raspberry Pi

Download the installer:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Select `2) Custom installation`, for `Default host triple?` enter `arm-unknown-linux-gnueabihf` and the `nightly` toolchain.

Since `rocket` requires *nightly*, we need to set this.

## Start the fan service
`cargo run`

## Future Work
- [ ] add scheduled activation of fan
