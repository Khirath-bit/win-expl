#[derive(Clone)]
pub struct RingBuffer<T> where T: Clone {
    buffer: Vec<Option<T>>,
    size: usize,
    write_index: usize,
    read_index: usize,
}

impl<T: std::clone::Clone> RingBuffer<T> {
    pub fn new(size: usize) -> Self {
        let buffer: Vec<Option<T>> = vec![None; size];
        RingBuffer {
            buffer,
            size,
            write_index: 0,
            read_index: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        self.write_index = (self.write_index + 1) % self.size;
        if self.write_index == self.read_index {
            self.read_index = (self.read_index + 1) % self.size;
        }
        self.buffer[self.write_index] = Some(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read_index == self.write_index {
            None 
        } else {
            let item = self.buffer[self.write_index].take();
            self.write_index = (self.write_index + self.size - 1) % self.size;
            item
        }
    }

    pub fn all_read(&self) -> bool {
        self.read_index == self.write_index
    }
}

impl Default for RingBuffer<String> {
    fn default() -> Self {
        RingBuffer::new(10)
    }
}