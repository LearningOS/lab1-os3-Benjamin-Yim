use core::cell::{Ref, RefCell, Cell};

use alloc::{boxed::Box, sync::Arc, vec::Vec};

use crate::{config::{MAX_APP_NUM, MAX_SYSCALL_NUM}, sync::UPSafeCell};

use super::TaskContext;

// 任务运行状态
#[derive(Copy,Clone,PartialEq,Debug)]
pub enum TaskStatus{
    UnInit, // 未初始化
    Ready,  // 准备运行
    Running,    // 正在运行
    Exited, // 已退出
}


// 任务控制块
#[derive(Clone)]
pub struct TaskControlBlock{
    // 当前任务状态
    pub task_status: TaskStatus,
    // 任务上下文
    pub task_cx: TaskContext,
    // 当前任务当次运行开始时间
    pub task_run_start_time: usize,
    // 当前任务运行总长
    pub task_run_total_time: usize,
    // 当前任务调用系统调用时时长
    pub task_run_syscall_total_time: usize,
    // 每次发生系统调用加1
    pub syscall_arr: Vec<u32>,
}