
pub mod stack;
pub mod queue;
pub mod hashtable;

#[cfg(test)]
mod datastructure_tests {
    use crate::stack;
    use crate::queue;
    use crate::hashtable;


    #[test]
    fn stack_works() {
        let mut stack = stack::Stack  {
            elements: vec![1, 2, 3, 4, 5, 6]
        };
        assert_eq!(stack.stack_length(), 6);
        stack.push(10);
        assert_eq!(stack.stack_length(), 7);
        let popped_val = stack.pop();
        assert_eq!(stack.stack_length(), 6);
        assert_eq!(popped_val, 10);

    }

    #[test]
    fn queue_works() {
        let mut queue = queue::Queue  {
            elements: vec![1]
        };
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);
        queue.enqueue(5);
        queue.enqueue(6);

        let dequeued_element = queue.dequeue();
        assert_eq!(dequeued_element, 1);
        let dequeued_element = queue.dequeue();
        assert_eq!(dequeued_element, 2);
    }

    #[test]
    fn hashtable_works() {
        let mut table = hashtable::create_hash_table();
        table.insert(10, 670);

        let retrieved = table.retrieve(10).unwrap();

        assert_eq!(*retrieved, 670);

    }
}
struct NewsPaper {
    articles: String,
}

impl Summarize for NewsPaper {
    fn summarize(&self) {
        println!("{}" , self.articles.clone());
    }
}
trait Summarize {
    fn summarize(&self);
}
fn main() {
    let mut stack = stack::Stack  {
        elements: vec![1, 2, 3, 4, 5, 6]
    };
    println!("Stack length {}", stack.stack_length());
    stack.push(10);

    println!("Stack length {}", stack.stack_length());
    stack.pop();

    println!("Stack length {}", stack.stack_length());

    let newspaper_1 = NewsPaper {
        articles: String::from("Sup")
    };

    newspaper_1.summarize();
}
