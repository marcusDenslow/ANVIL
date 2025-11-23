#![no_std] //dont link the rust standard lib
#![no_main] //disable all rust-level entry points

use core::panic::PanicInfo;


mod vga_buffer;


#[unsafe(no_mangle)] //dont mangle the name of this function
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    println!("Hello world{}", "!");


    loop {}
}


//this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
