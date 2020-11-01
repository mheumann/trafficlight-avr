#!/bin/bash
cargo build -Z build-std=core --target avr-atmega8.json --release
sudo avrdude -p m8 -c avr911 -P /dev/ttyUSB0 -U flash:w:target/avr-atmega8/release/trafficlight-avr.elf:e
