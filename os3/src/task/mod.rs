
#[allow(clippy::module_inception)]
mod task;
mod context;
mod switch;

use core::borrow::BorrowMut;
use core::cell::RefCell;

use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use lazy_static::*;
use crate::loader::{get_num_app,init_app_cx};
use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::sync::UPSafeCell;
use crate::timer::{get_time, get_time_ms};
pub use switch::__switch;
pub use task::{TaskControlBlock,TaskStatus};
pub use context::TaskContext;

/**
 * num_app: 表示应用数目,它在 TaskManager 初始化后将保持不变；
 * inner: 包裹在 TaskManagerInner 内的任务控制块数组 tasks
 */
pub struct TaskManager {
    num_app: usize, 
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize, // 正在执行的应用编号 current_task 会在执行过程中变化
}

impl TaskManager {

    /**
     * 运行第一个
     */
    fn run_first_task(&self){
        println!("TASK_MANAGER.run_first_task()");
        let mut inner = self.inner.exclusive_access();
        // 获取到第一个任务
        let task0 = &mut inner.tasks[0];
        // 修改第一个任务状态为 Ready
        task0.task_status = TaskStatus::Ready;
        // 解引用为裸指针
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // 在这之前必须删除所有局部变量，因为__switch不返回了
        unsafe {
            // 切换上下文
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task")
    }

    /**
     * 先修改当前应用的运行状态
     * 修改当前应用状态，同时也说明当前应用让出 CPU  运行，
     * 可将上次运行开始时时间减去当前时间，再累加以前的运行时间
     * 就可以获取当前应用总共运行了多久
     * 
     */
    fn mark_current_suspended(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
        if inner.tasks[current].task_run_start_time != 0 {
            inner.tasks[current].task_run_total_time = inner.tasks[current].task_run_total_time + (get_time_ms() -inner.tasks[current].task_run_start_time)
        }
    }

    fn current_task(&self) -> TaskControlBlock{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let current_task = inner.tasks[current];
        current_task
    }

    /**
     * 查找下一个任务并运行
     */
    fn run_next_task(&self){
        // 会调用 find_next_task 方法尝试寻找一个运行状态为 Ready 的应用并获得其 ID 
        if let Some(next) = self.find_next_task(){
            // println!("run next task id {}",next);
            let mut inner = self.inner.exclusive_access();
            let current  = inner.current_task;
            // 标记当前任务为运行状态
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.tasks[next].task_run_start_time = get_time_ms();
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            // 手动 drop 掉我们获取到的 TaskManagerInner 可变引用。 
            // 因为函数还没有返回， inner 不会自动销毁
            drop(inner);
            unsafe{
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // 返回用户模式
        } else {
            panic!("All applications completed!");
        }
    }

    /**
     * find_next_task 方法尝试寻找一个运行状态为 Ready 的应用并获得其 ID 
     */
    fn find_next_task(&self) -> Option<usize>{
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let task =  (current + 1..current + self.num_app + 1)
        .map(|id| id % self.num_app)
        .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready);
        // println!("task id {:?}",task);
        task
    }

    /**
     * 标记当前任务为退出状态
     */
    fn mark_current_exited(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        println!("[kernel] mark current task {} exit,task run total time,{}",current,inner.tasks[current].task_run_total_time);
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
}

lazy_static!{
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let array = &[0;MAX_SYSCALL_NUM];
        let mut tasks = [TaskControlBlock{
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            task_run_start_time: 0,
            task_run_total_time: 0,
            syscall_total: 0,
            syscall_arr:  array,
        }; MAX_APP_NUM];
        
        // CPU 第一次从内核态进入用户态的方法，只需在内核栈上压入构造好的 Trap 上下文， 然后 __restore 即可
        for (i,t) in tasks.iter_mut().enumerate().take(num_app) {
            // init_app_cx 在 loader 子模块中定义，它向内核栈压入了一个 Trap 上下文，并返回压入 Trap 上下文后 sp 的值。
            t.task_cx = TaskContext::goto_restore(init_app_cx(i));
            t.task_status = TaskStatus::Ready;
        }
        // println!("load.... ");
        TaskManager{
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner{
                    tasks,
                    current_task: 0,
                })
            },
        }
    };

    
}

pub fn run_first_task(){
    TASK_MANAGER.run_first_task();
}

fn run_next_task(){
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended(){
    TASK_MANAGER.mark_current_suspended();
} 

fn mark_current_exited(){
    TASK_MANAGER.mark_current_exited();
}

fn __current_task() -> TaskControlBlock{
    TASK_MANAGER.current_task()
}

pub fn suspend_current_and_run_next(){
    // 先修改当前应用的运行状态，然后尝试切换到下一个应用
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next(){
    // 先修改当前应用的运行状态，然后尝试切换到下一个应用
    mark_current_exited();
    run_next_task();
}

pub fn current_task() -> TaskControlBlock{
    __current_task()
}