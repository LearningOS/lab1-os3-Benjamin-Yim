#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    // ra 很重要，它记录了 __switch 函数返回之后应该跳转到哪里继续执行。
    ra: usize,
    // 栈地址
    sp: usize, 
    // s0~s11 是被调用者保存寄存器
    s: [usize; 12],
}
// 任务控制块相关信息（任务状态）、任务使用的系统调用及调用次数、任务总运行时长（单位ms）
impl TaskContext {
    /**
     * 任务上下文初始化，建立一个新的任务上线文
     */
    pub fn zero_init() -> Self{
        Self { ra: 0, sp: 0, s: [0;12] }
    }
    /**
     * goto_restore 保存传入的 sp，并将 ra 设置为 __restore 的入口地址，
     * 构造任务上下文后返回
     */
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
