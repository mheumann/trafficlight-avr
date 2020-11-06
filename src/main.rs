#![feature(llvm_asm)]

#![no_std]
#![no_main]

extern crate avr_std_stub;
extern crate avrd;
use avrd::atmega8::*;
use core::ptr::*;

#[no_mangle]
pub extern fn main() {
    let red: u8 = 0b00000100;
    let yellow: u8 = 0b00000010;
    let green: u8 = 0b00000001;

    unsafe {
        write_volatile(DDRB, 0xFF);

        loop {
            write_volatile(PORTB, red);
            small_delay(600_000);

            write_volatile(PORTB, red | yellow);
            small_delay(200_000);

            write_volatile(PORTB, green);
            small_delay(600_000);

            write_volatile(PORTB, yellow);
            small_delay(400_000);
        }
    }
}

/// A small busy loop.
fn small_delay(time: u32) {
    for _ in 0..time {
        unsafe { llvm_asm!("" :::: "volatile")}
    }
}
