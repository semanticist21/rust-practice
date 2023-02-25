pub struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { data: vec![] }
    }

    pub fn enqueue(&mut self, item: T) {
        self.data.insert(0, item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}