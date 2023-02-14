
// Stack
struct Stack<T> {
    elements: Vec<T>
}

impl<T> Stack<T> {
    fn pop(&mut self) -> T {
        self.elements.pop().unwrap()
    }

    fn push(&mut self, element: T) {
        self.elements.push(element);
    }

    fn stack_length(&self) -> usize {
        self.elements.len()
    }
}


fn main() {
    let mut stack = Stack  {
        elements: vec![1, 2, 3, 4, 5, 6]
    };
    println!("Stack length {}", stack.stack_length());

    stack.push(10);

    println!("Stack length {}", stack.stack_length());


    stack.pop();

    println!("Stack length {}", stack.stack_length());

}
