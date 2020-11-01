# blink

A small Hello World Rust application for the AVR.

The program itself toggles three LEDs on PORTB0 to PORTB2 in a sequence of a traffic light.

Designed for the ATmega8.

[How to set up a cross compiler](https://github.com/avr-rust/rust)

## Prerequisites

  * A recent version of the nightly Rust compiler. Anything including or greater than `rustc 1.47.0-nightly (0820e54a8 2020-07-23)` can be used.
  * The rust-src rustup component - `$ rustup component add rust-src`
  * AVR-GCC on the system for linking
  * AVR-Libc on the system for support libraries

## Usage


Now to build, run:

```bash
rustup override set nightly

# Compile the crate to an ELF executable.
cargo build -Z build-std=core --target avr-atmega8.json --release

```
There should now be an ELF file at `target/avr-atmega328p/release/trafficlight-avr.elf`. It
can be flashed directly to an AVR microcontroller or ran inside a simulator.

To flash on myAVR MK2 board you can also simply run the script flash.sh


## Resources

  * The [AVR-Rust book](https://book.avr-rust.com)

