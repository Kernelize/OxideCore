use crate::{kprintln, println, sync::UPSafeCell};
use core::{self, arch::asm, slice};
use lazy_static::lazy_static;
use log::trace;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;
const MAX_APP_NUM: usize = 16;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

struct AppManager {
    num_app: usize,
    current_app: usize,
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        kprintln!("{} apps in total", self.num_app);
        for i in 0..self.num_app {
            kprintln!(
                "app {} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app = (self.current_app + 1) % self.num_app;
    }

    /// Load the app from data segment to app memory.
    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            panic!("app_id out of range");
        }
        kprintln!("Loading app {}...", app_id);
        // clear app memory
        slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
        let app_src = slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );
        let app_dst = slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
        app_dst.copy_from_slice(app_src);
        kprintln!("Load app {} done", app_id);
        // memory fence about fetching the instruction memory
        asm!("fence.i");
    }
}

// To initialize the app manager at runtime.
lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" { fn _num_app(); }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            // Get the app_start array from the linker script.
            let app_start_raw: &[usize] = core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub fn print_app_info() {
    APP_MANAGER.borrow_mut().print_app_info();
}

// Here we used APP_MANAGER for the first time, So in this function the APP_MANAGER will be initialized.
pub fn init() {
    print_app_info();
}

pub fn run_next_app() {
    let mut app_manager = APP_MANAGER.borrow_mut();
    let current_app = app_manager.get_current_app();
    unsafe {
        app_manager.load_app(current_app);
    }
    app_manager.move_to_next_app();
    drop(app_manager);
    
}

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

// In .bss section
static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE]};
static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE]};

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
}


