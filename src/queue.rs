struct Queue<T, const N: usize> {
    data: [T; N],
    start: usize, // start always point to the first element (exept when queue is empty, then start == end)
    end: usize,   // end always point to the next empty slot
    len: usize,   // len is the number of elements in the queue
}
enum QueueError {
    Full,
    Empty,
}

impl<T: Default + Copy, const N: usize> Queue<T, N> {
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
