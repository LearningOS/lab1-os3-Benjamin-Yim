use crate::task::suspend_current_and_run_next;
use crate::task::exit_current_and_run_next;

pub fn sys_yield() -> isize {
    // suspend_current_and_run_next 接口，这个接口如字面含义，
    // 就是暂停当前的应用并切换到下个应用
    suspend_current_and_run_next();
    0
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}