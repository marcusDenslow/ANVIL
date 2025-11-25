#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ANVIL::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;


use bootloader::{ BootInfo, entry_point};
use core::panic::PanicInfo;
use ANVIL::println;

entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ANVIL::memory;
    use x86_64::{structures::paging::Page, VirtAddr};
    use ANVIL::memory::BootInfoFrameAllocator;


    println!("John Marston{}", "!");
    ANVIL::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    let x = Box::new(41);



    #[cfg(test)]
    test_main();

    println!("it did not crash!");
    ANVIL::hlt_loop();
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
    println!("{}", info);
    ANVIL::hlt_loop();
}
