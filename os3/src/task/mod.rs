pub mod task;

use task::TaskControlBlock;

use crate::config::MAX_APP_NUM;
use self::task::TaskStatus;

pub struct TaskManager {
    num_app: isize,
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    task: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize, // 正在执行的应用编号 current_task 会在执行过程中变化
}

impl TaskManager {
    // 先修改当前应用的运行状态
    fn mark_current_suspended(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }
}

lazy_static!{
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock{
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }; MAX_APP_NUM];
        for (i,t) in tasks.iter_mut().enumerate().take(num_app) {
            t.task_cx = TaskContext::goto_restore(init_app_cx(i));
            t.task_status = TaskStatus::Ready;
        }

        TaskManager{
            num_app,
            inner: unsafe {
                UnSafeCell::new(TaskManagerInner{})
                tasks,
                current_task: 0,
            }
        }
    }
}

pub fn suspend_current_and_run_next(){
    // 先修改当前应用的运行状态，然后尝试切换到下一个应用
    TASK_MANAGER.mark_current_suspended();
    TASK_MANAGER.run_next_task();
}

pub fn exit_current_and_run_next(){
    // 先修改当前应用的运行状态，然后尝试切换到下一个应用
    TASK_MANAGER.mark_current_exited();
    TASK_MANAGER.run_next_task();
}