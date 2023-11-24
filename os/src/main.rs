#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
mod cpu;
mod logo;
mod console;
mod logging;
mod sync;
mod batch;

use core::arch::global_asm;
use log::{error, trace, info, debug, warn};

use crate::{sync::UPSafeCell, logo::LOGO,};

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("\nOxideCore is booting...");
    println!("{}", LOGO);
    dump_mem_layout();
    error!("shit!");
    panic!("Time to stop");
    loop {}
}

fn clear_bss() {
    #[allow(unused)]
    extern "C" {
        // Get symbol from linker.ld
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|p| unsafe {
        (p as *mut u8).write_volatile(0);
    });
}

fn dump_mem_layout() {
    #[allow(unused)]
    extern "C" {
        // Get symbol from linker.ld
        fn sbss();
        fn ebss();
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn skernel();
        fn ekernel();
    }
    info!(".kernel [{:#x}, {:#x})", skernel as usize, ekernel as usize);
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}
