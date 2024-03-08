use std::{alloc::{self, handle_alloc_error, Layout}, fmt::Debug, ptr::{self, NonNull}};

pub struct Queue<T> {
    head: usize,
    len: usize,
    ptr: NonNull<T>,
    cap: usize
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "ZST not ready");

        Self { head: 0, len: 0, ptr: NonNull::dangling(), cap: 0 }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };

        let new_layout = Layout::array::<T>(new_cap).unwrap();

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout)
        };
        self.cap = new_cap;
    }

    pub fn enqueue(&mut self, elem: T) {
        if self.len == self.cap { self.grow(); }

        unsafe { ptr::write(self.ptr.as_ptr().add(self.len), elem); }

        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.head == self.len {
            None
        } else {
            let first = unsafe { ptr::read(self.ptr.as_ptr().add(self.head)) };
            self.head += 1;
            Some(first)
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.dequeue() { }
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl <T: Debug> Debug for Queue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Queue [")?;
        for i in self.head..self.len {
            write!(f, "{:?}", unsafe { ptr::read(self.ptr.as_ptr().add(i)) })?;
            if i != self.len-1 { write!(f, ",")?; }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[macro_export]
macro_rules! queue {
    () => {
        Queue::new()
    };
    ($($e:expr),+) => {
        {
            let mut temp = Queue::new();
            $(temp.enqueue($e);)+
            temp
        }
    }
}