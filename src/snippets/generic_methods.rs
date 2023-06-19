// https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn print(&self)
    where
        T: std::fmt::Debug,
    {
        for item in &self.items {
            println!("{:?}", item);
        }
    }
}

pub fn run() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    stack.print();

    if let Some(item) = stack.pop() {
        println!("Popped item: {:?}", item);
    }

    println!("{:?}", stack.items);

    println!("Is stack empty? {}", stack.is_empty());
}
