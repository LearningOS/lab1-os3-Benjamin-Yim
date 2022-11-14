// 任务运行状态
#[derive(Copy,Clone,PartialEq)]
pub enum TaskStatus{
    UnInit, // 未初始化
    Ready,  // 准备运行
    Running,    // 正在运行
    Exited, // 已退出
}


// 任务控制块
#[derive(Copy,Clone)]
pub struct TaskControlBlock{
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}