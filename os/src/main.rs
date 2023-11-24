#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
mod cpu;
mod logo;
mod console;
mod logging;

use core::arch::global_asm;
use log::error;

use crate::{logo::LOGO,};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("\nOxideCore is booting...");
    println!("{}", LOGO);
    error!("shit!");
    panic!("G");
    loop {}
}

fn clear_bss() {
    extern "C" {
        // Boundaries of the .bss section, provided by the linker script
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|p| unsafe {
        (p as *mut u8).write_volatile(0);
    });
}
