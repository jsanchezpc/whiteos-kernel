#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("WhiteOS\n").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        "AI powered"
    )
    .unwrap();

    loop {}
}
