#![no_std]
#![no_main]

use core::panic::PanicInfo;

//entry point for the kernel
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Woohoo!");
    
    loop {}
}

//when rust panics
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

mod vga_buffer;