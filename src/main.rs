#![no_std]
#![no_main]

use core::panic::PanicInfo;

//entry point for the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Woohoo!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;