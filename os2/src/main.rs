#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
extern crate log;

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod logging;
mod sbi;
mod sync;
mod syscall;
mod trap;

// 这里我们引入了一段汇编代码 entry.asm
core::arch::global_asm!(include_str!("entry.asm"));
// 这里我们引入了一段汇编代码 link_app.S,是在 make run 构建操作系统时自动生成的
core::arch::global_asm!(include_str!("link_app.S"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trap::init();
    batch::init();
    batch::run_next_app();
}
