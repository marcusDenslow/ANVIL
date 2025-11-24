#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ANVIL::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;
use ANVIL::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello wolrd{}", "!");

    ANVIL::init();

    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    loop {}
}


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}


#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ANVIL::test_panic_handler(info);
}
