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

    pub fn push(&mut self) {
        if self.capacity() == 0 {
            if std::mem::size_of::<T>() == 0 {
                panic!("No zero sized types");
            }

            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate memory.");
            // SAFETY: Layout is hardcoded to be 4* size_of<T>
            let ptr = unsafe { alloc::alloc(layout) as *mut T };
        }
        todo!()
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
