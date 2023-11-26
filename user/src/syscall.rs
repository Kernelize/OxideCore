use core::arch::asm;
use cty::{c_ulong, c_int};

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

// for c ffi
#[no_mangle]
pub extern "C" fn csyscall(id: c_ulong, arg1: c_ulong, arg2: c_ulong, arg3: c_ulong) -> c_int{
    syscall(id as usize, [arg1 as usize, arg2 as usize, arg3 as usize]) as c_int
}

pub fn sys_write(fd: usize, buf: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buf.as_ptr() as usize, buf.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}
