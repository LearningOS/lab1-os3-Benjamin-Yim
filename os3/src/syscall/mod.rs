const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
mod process;

use core::borrow::{BorrowMut, Borrow};

use alloc::vec::Vec;
use fs::*;
use process::*;

use crate::{task::{ current_task_mark_syscall, mark_task_run_syscall_total_time}, config::MAX_SYSCALL_NUM, timer::{get_time, get_time_us}};

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize{
    current_task_mark_syscall(syscall_id);
    match syscall_id{
        SYSCALL_WRITE => sys_write(args[0],args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TASK_INFO => sys_task_info(args[0] as *mut TaskInfo),
        _ => panic!("not wsupport syscall syscall_id:{}", syscall_id),
    } 
}