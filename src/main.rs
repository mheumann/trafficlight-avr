#![feature(llvm_asm)]

#![no_std]
#![no_main]

extern crate avr_std_stub;
extern crate avrd;
use avrd::atmega8::*;
use core::ptr::*;

#[no_mangle]
pub extern fn main() {
    unsafe {
        write_volatile(DDRB, 0xFF);

        loop {
            write_volatile(PORTB, 0b00000100);
            small_delay(600_000);

            write_volatile(PORTB, 0b00000110);
            small_delay(200_000);

            write_volatile(PORTB, 0b00000001);
            small_delay(600_000);

            write_volatile(PORTB, 0b00000010);
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
