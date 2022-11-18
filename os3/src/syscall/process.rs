use core::borrow::Borrow;

use crate::config::MAX_SYSCALL_NUM;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER, current_task, TaskControlBlock};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal{
    pub sec: usize,
    pub usec: usize,
}
#[derive(Debug)]
pub struct TaskInfo{
    // 任务状态
    status: TaskStatus,
    // 系统调用次数
    syscall_time: [u32;MAX_SYSCALL_NUM],
    // 任务运行时长
    time: usize,
}

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}


pub fn sys_yield() -> isize {
    // suspend_current_and_run_next 接口，这个接口如字面含义，
    // 就是暂停当前的应用并切换到下个应用
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(ts: *mut TimeVal,_tz: usize) -> isize{
    let us = get_time_us();
    unsafe{
        *ts = TimeVal{
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    }
    0
}

pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let current = current_task();
    unsafe {
        println!("[kernel] println task info");
        println!("[kernel] task info {}", current.task_run_total_time);
        println!("[kernel] task syscall total {}", current.syscall_total);
        let array = current.syscall_arr.clone();
        *ti = TaskInfo{
            status: current.task_status,
            time: current.syscall_total,
            syscall_time: array,
        }
    }

    0
}