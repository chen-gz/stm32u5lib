#![allow(dead_code)]


pub struct Queue<T, const N: usize> {
    pub data: [T; N],
    pub start: usize, // start always point to the first element (exept when queue is empty, then start == end)
    pub end: usize,   // end always point to the next empty slot
    pub len: usize,   // len is the number of elements in the queue
}
pub enum QueueError {
    Full,
    Empty,
}

impl<T: Default + Copy + core::cmp::PartialOrd, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Queue {
            data: [Default::default(); N],
            start: 0,
            end: 0,
            len: 0,
        }
    }
    pub fn push(&mut self, value: T) -> Result<(), QueueError> {
        if self.len == N {
            return Err(QueueError::Full);
        }
        self.data[self.end] = value;
        self.end += 1;
        if self.end == N {
            self.end = 0;
        }
        Ok(())
    }
    pub fn push_sort(&mut self, value: T) -> Result<(), QueueError> {
        // the smallest value will be at the start
        if self.len == N {
            return Err(QueueError::Full);
        }
        // move all elemnts that are bigger than value to the right
        let mut i = self.len;
        while i > 0 && self.data[(self.start + i - 1) % N] > value {
            self.data[(self.start + i) % N] = self.data[(self.start + i - 1) % N];
            i -= 1;
        }
        // insert the value
        self.data[(self.start + i) % N] = value;
        self.end += 1;
        if self.end == N {
            self.end = 0;
        }
        self.len += 1;
        Ok(())
    }
    pub fn pop(&mut self) -> Result<T, QueueError> {
        if self.len == 0 {
            return Err(QueueError::Empty);
        }
        let val = self.data[self.start];
        self.start += 1;
        if self.start == N {
            self.start = 0;
        }
        self.len -= 1;
        Ok(val)
    }
    /// Push a value to the queue, if the queue is full, the oldest value will be overwritten
    /// and the function will return an error
    pub fn push_force (&mut self, value: T) -> Result<(), QueueError> {
        let mut ret = Ok(());
        if self.len == N {
            self.start += 1;
            if self.start == N {
                self.start = 0;
            }
            ret = Err(QueueError::Full);
        }
        self.data[self.end] = value;
        self.end += 1;
        if self.end == N {
            self.end = 0;
        }
        self.len += 1;
        return ret;
    }
}
