.altmacro
.macro SAVE_SN n
    sd s\n, (\n+2)*8(a0)
.endm
.macro LOAD_SN n
    ld s\n, (\n+2)*8(a1)
.endm
    .section .text
    .global __switch
__switch:
    # 传两个参数进来，a0 和  a1
    # a0 指向 current_task_cx_ptr
    # a1 指向 next_task_cx_ptr
    # __switch(
    #       current_task_cx_ptr: *mut TaskContext,
    #       next_task_cx_ptr: *const TaskContext
    # )
    # save kernel stack of current stack
    # a0 代表返回值或者参数0
    # sp 栈指针
    sd sp,8(a0)
    # save ra & s0-s11 of current execution
    # ra 很重要，它记录了 __switch 函数返回之后应该跳转到哪里继续执行。
    # 保存当前进程上下文到 current_task_cx_ptr 
    sd ra,0(a0)
    .set n,0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    #restore ra & s0-s11 of next execution
    # a1 代表返回值或者参数0
    # 从 next_task_cx_ptr 恢复下一个进程上线文
    ld ra,0(a1)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    # restore kernel stack of next stack
    ld sp, 8(a1)
    ret
