cargo build -Z build-std=core --target avr-atmega8.json --release
avr-objcopy -O ihex target/avr-atmega8/release/trafficlight-avr.elf target/avr-atmega8/release/trafficlight-avr.hex
avrdude -p m8 -c avr911 -P com5 -U flash:w:target/avr-atmega8/release/trafficlight-avr.hex:i
