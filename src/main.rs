#![no_std] // no_std -> Build without Rust's standard library.
#![no_main] // no_main -> Use a custom entry point.

use core::panic::PanicInfo; // use -> Import information about Rust panics.

// static -> Global, immutable sequence of bytes named HELLO.
static HELLO: &[u8] = b"Hello World";

#[unsafe(no_mangle)] // no_mangle -> Keep the _start name for the linker.
// extern "C" -> Use the C calling convention.
pub extern "C" fn _start() -> ! { // _start -> Entry point; ! means it never returns.
    let vga_buffer = 0xb8000 as *mut u8; // vga_buffer -> Writable pointer to VGA text memory.

    for (i, &byte) in HELLO.iter().enumerate() { // enumerate -> Get each byte and its index.
        unsafe { // unsafe -> Needed to write to screen memory.
            // offset -> Move the pointer by bytes here (u8 is one byte).
            // as isize -> Convert i to the number type offset expects.
            *vga_buffer.offset(i as isize * 2) = byte; // i * 2 -> Character position.
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // + 1 -> Color position.
        }
    }

    loop {} // loop -> Keep the kernel running.
}

#[panic_handler] // panic_handler -> Handle panics in this no_std program.
fn panic(_info: &PanicInfo) -> ! { // _info -> Panic details; ! -> Never returns.
    loop {} // loop -> Stop here after a panic.
}
