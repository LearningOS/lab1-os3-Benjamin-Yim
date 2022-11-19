use crate::config::CLOCK_FREQ;
use crate::sbi::set_timer;
use riscv::register::time;

const TICKS_PER_SEC: usize = 100;
const MICRO_PER_SEC: usize = 1_000_000;

pub fn get_time() -> usize {
    time::read()
}

pub fn get_time_us() -> usize {
    time::read() / (CLOCK_FREQ / MICRO_PER_SEC)
}

/**
 * timer 子模块的 set_next_trigger 函数对 set_timer 进行了封装， 
 * 它首先读取当前 mtime 的值，然后计算出 10ms 之内计数器的增量，
 * 再将 mtimecmp 设置为二者的和
 */
pub fn set_next_trigger() {
    // CLOCK_FREQ 是一个预先获取到的各平台不同的时钟频率，
    // 单位为赫兹，也就是一秒钟之内计数器的增量
    set_timer(get_time() + CLOCK_FREQ / TICKS_PER_SEC);
}
