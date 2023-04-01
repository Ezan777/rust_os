#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!"; // Define our static byte string

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let vga_buffer = 0xb8000 as *mut u8; // We cast the integer to a raw pointer

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // We include our code inside unsafe block to tell the Rust compiler we are absolutely sure that these operations are valid
            *vga_buffer.offset(i as isize * 2) = byte; // This is our character
            *vga_buffer.offset(i as isize *2 + 1) = 0xb; // This is the color byte
        }
    }

    loop {}
}