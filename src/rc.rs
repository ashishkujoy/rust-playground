use crate::cell::MyCell;
use core::ptr::NonNull;

struct MyRcInner<T> {
    value: T,
    ref_count: MyCell<usize>,
}

struct MyRc<T> {
    inner: NonNull<MyRcInner<T>>,
}

impl<T> MyRc<T> {
    fn new(value: T) -> Self {
        let inner = Box::new(MyRcInner {
            value,
            ref_count: MyCell::new(0),
        });

        MyRc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
        }
    }
}

impl<T> Clone for MyRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let inner = self.inner.as_ref();
            inner.ref_count.set(inner.ref_count.get() + 1);
        };

        MyRc { inner: self.inner }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let ref_count = unsafe { self.inner.as_ref().ref_count.get() };
        if ref_count == 1 {
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            unsafe {
                self.inner.as_ref().ref_count.set(ref_count - 1);
            }
        }
    }
}
