#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ANVIL::test_runner)]
#![reexport_test_harness_main = "test_main"]


use bootloader::{ BootInfo, entry_point};
use core::panic::PanicInfo;
use ANVIL::println;

entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ANVIL::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello world{}", "!");
    ANVIL::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }


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
