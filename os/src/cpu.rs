use core::arch::asm;

pub fn id() -> usize {
    let cpu_id;
    unsafe {
        asm!{"mv {}, tp", out(reg) cpu_id};
    }
    cpu_id
}
