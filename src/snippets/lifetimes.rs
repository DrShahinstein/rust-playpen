struct Stack<'a, T> {
    items: &'a mut Vec<T>, // Lifetime 'a specifies the scope of the borrowed vector reference
}

impl<'a, T> Stack<'a, T> {
    fn new(items: &'a mut Vec<T>) -> Self {
        Stack { items }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

pub fn run() {
    let mut stack_data = vec![1, 2, 3, 4, 5];
    let mut stack = Stack::new(&mut stack_data); // Create Stack instance with a mutable reference to stack_data

    stack.push(6);
    stack.push(7);
    stack.push(8);

    while let Some(item) = stack.pop() {
        println!("Popped item: {}", item);
    }
}

/*
Output:
Popped item: 8
Popped item: 7
Popped item: 6
Popped item: 5
Popped item: 4
Popped item: 3
Popped item: 2
Popped item: 1
*/

/*
Notes
- The 'a lifetime parameter in the Stack struct specifies the scope of the borrowed vector reference items. It ensures that the reference lives at least as long as the Stack instance.
- In the ::new method, the parameter items is annotated with the same lifetime 'a to indicate that the borrowed reference and the Stack instance have the same lifetime.
- The push and pop methods modify the underlying vector through the mutable reference held by the Stack instance.
- In the main function, we create a mutable vector stack_data and pass a mutable reference to it when creating the Stack instance. This allows us to borrow and modify the vector through the Stack.
 */
