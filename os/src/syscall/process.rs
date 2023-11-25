use crate::{kprintln, batch::run_next_app};

pub fn sys_exit(xstate: i32) -> ! {
    kprintln!("Application exited with code {}", xstate);
    run_next_app()
}
