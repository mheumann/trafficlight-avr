#![feature(llvm_asm)]

#![no_std]
#![no_main]

extern crate avr_std_stub;
extern crate avrd;
use avrd::atmega8::*;

#[no_mangle]
pub extern fn main() {
    unsafe {
        core::ptr::write_volatile(DDRB, 0xFF);

        loop {
            core::ptr::write_volatile(PORTB, 0b00000100);
            small_delay();

            core::ptr::write_volatile(PORTB, 0b00000110);
            small_delay();

            core::ptr::write_volatile(PORTB, 0b00000001);
            small_delay();

            core::ptr::write_volatile(PORTB, 0b00000010);
            small_delay();
        }
    }
}

/// A small busy loop.
fn small_delay() {
    for _ in 0..400000 {
        unsafe { llvm_asm!("" :::: "volatile")}
    }
}
