use std::{alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout}, fmt::Debug, ptr::{self, NonNull}};

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize
}

#[macro_export]
macro_rules! vec {
    () => {
        Vec::new()
    };
    ($($e:expr),+) => {
        {
            let mut temp = Vec::new();
            $(temp.push($e);)+
            temp
        }
    }
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "not ready for ZST");

        Self {
            ptr: NonNull::dangling(),
            cap: 0,
            len: 0
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = self.cap * 2;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => handle_alloc_error(new_layout)
        };
        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap { self.grow(); }

        unsafe { ptr::write(self.ptr.as_ptr().add(self.len), elem) };

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0{
            None
        } else {
            self.len -= 1;

            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() { }
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl <T: Debug> Debug for Vec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len {
            write!(f, "{:?}", unsafe { ptr::read(self.ptr.as_ptr().add(i)) } )?;
            if i != self.len-1 { write!(f, ",")?; }
        }
        write!(f, "]")?;

        Ok(())
    }
}