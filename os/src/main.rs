#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod batch;
mod console;
mod cpu;
mod lang_items;
mod logging;
mod logo;
mod sbi;
mod sync;
mod trap;
mod syscall;

use core::arch::global_asm;
use log::{debug, error, info, trace, warn};

use crate::logo::LOGO;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("\n");
    kprintln!("OxideCore is booting...");
    println!("{}", LOGO);
    dump_mem_layout();
    trap::init();
    batch::init();
    kprintln!("OxideCore is ready!");
    batch::run_next_app();
    unreachable!()
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
