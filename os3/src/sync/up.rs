
use core::cell::{RefCell, RefMut};
// RefCell 参考文档：https://kaisery.github.io/trpl-zh-cn/ch15-05-interior-mutability.html
// RefCell 作用是在编译器认为出现借用问题时，无法通过编译。但是开发者确认是安全的所以
// 通过 RefCell 方法在运行时检测
pub struct UPSafeCell<T>{
    inner: RefCell<T>
}

unsafe impl<T> Sync for UPSafeCell<T>{

}

impl<T> UPSafeCell<T>{
    pub unsafe fn new(value: T) -> Self {
        Self{
            inner: RefCell::new(value)
        }
    }

    pub fn exclusive_access(&self) -> RefMut<'_, T>{
        self.inner.borrow_mut()
    }
}