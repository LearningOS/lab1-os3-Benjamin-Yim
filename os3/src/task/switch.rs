core::arch::global_asm!(include_str!("switch.S"));

use super::TaskContext;

extern "C"{
    // 保存当前寄存器到 current_task_cx_ptr 
    // 切换 next_task_cx_ptr 上下文到当前执行栈和寄存器中
    pub fn __switch(
        current_task_cx_ptr: *mut TaskContext,
        next_task_cx_ptr: *const TaskContext
    );
}