use core::arch::global_asm;

use riscv::register::stvec;

global_asm!(include_str!("trap/trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}
