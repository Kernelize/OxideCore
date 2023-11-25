use core::arch::global_asm;

use riscv::register::{stvec, scause, stval};

use crate::{batch::{TrapContext, run_next_app}, syscall::syscall, kprintln};

global_asm!(include_str!("trap/trap.S"));

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    // cx is a pointer to the TrapContext (TrapFrame)
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        scause::Trap::Exception(scause::Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        scause::Trap::Exception(scause::Exception::StoreFault) |
        scause::Trap::Exception(scause::Exception::StorePageFault) => {
            kprintln!("PageFault in appication, kernel killed it!");
            run_next_app();
        }
        scause::Trap::Exception(scause::Exception::IllegalInstruction) => {
            kprintln!("IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}
