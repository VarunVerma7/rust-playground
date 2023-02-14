pub struct Queue<T> {
    pub elements: Vec<T>
}

impl<T> Queue<T> {
    pub fn enqueue(&mut self, element: T) {
        self.elements.insert(0, element);
    }

    pub fn dequeue(&mut self) -> T {
        self.elements.pop().unwrap()
    }
}