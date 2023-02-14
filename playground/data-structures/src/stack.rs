
pub struct Stack<T> {
    pub elements: Vec<T>
}

impl<T> Stack<T> {
    pub fn pop(&mut self) -> T {
        self.elements.pop().unwrap()
    }

    pub fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    pub fn stack_length(&self) -> usize {
        self.elements.len()
    }
}
