use std::alloc;
use std::ptr::NonNull;

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push(&mut self, item: T) {
        if self.capacity() == 0 {
            assert_ne!(std::mem::size_of::<T>(), 0, "No zero sized types");

            let layout: alloc::Layout =
                alloc::Layout::array::<T>(4).expect("Could not allocate memory.");

            // SAFETY: Layout is hardcoded to be 4* size_of<T> and size_of<T> is greater than zero
            let ptr: *mut T = unsafe { alloc::alloc(layout) as *mut T };
            let ptr: NonNull<T> = NonNull::new(ptr).expect("Could not allocate memory");

            // SAFETY: ptr is non-null and we have just allocated space in memory for this one item (and 3 more)
            unsafe { ptr.as_ptr().write(item) };

            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset: usize = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot find memory location");
            assert_eq!(offset < isize::MAX as usize, "Wrapped isize");
            // Offset cannot wrap around and pointer is pointing to valid memory
            // And writing to an offset at self.len is valid
            unsafe { self.ptr.as_ptr().add(self.len).write(item) } //do we lose ownership of self.len?
            self.len += 1;
        } else {
            todo!();
        }
    }
}

impl<T> Default for MyVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();

        // vec.push(1usize);
        // vec.push(1);
        // vec.push(2);

        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);
        println!("")
    }
}
