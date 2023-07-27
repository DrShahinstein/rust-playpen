use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    #[allow(dead_code)]
    name: String,
    parent: Option<Rc<RefCell<Person>>>,
}

impl Person {
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Person {
            name: name.to_string(),
            parent: None,
        }))
    }

    fn set_parent(&mut self, parent: Rc<RefCell<Person>>) {
        self.parent = Some(parent);
    }
}

pub fn run() {
    let lucas = Person::new("Lucas");
    let emily = Person::new("Emily");
    let oliver = Person::new("Oliver");
    let sophia = Person::new("Sophia");

    emily.borrow_mut().set_parent(Rc::clone(&lucas));
    oliver.borrow_mut().set_parent(Rc::clone(&lucas));
    sophia.borrow_mut().set_parent(Rc::clone(&emily));

    println!("Family Tree:");
    println!("{:?}", lucas);
    println!("{:?}", emily);
    println!("{:?}", oliver);
    println!("{:?}", sophia);
}

/*
In this program, Rc<T> and RefCell<T> work hand in hand to manage shared ownership and enable interior mutability,
respectively. Rc<T> allows for multiple parents to share ownership of the same Person instance,
ensuring memory is deallocated efficiently when the last reference is dropped.
This enables the representation of complex family relationships without duplicating data and
prevents memory leaks.

On the other hand, RefCell<T> provides a way to achieve interior mutability,
allowing the set_parent method to modify the Person instance even when it is shared through Rc<T>.
The runtime borrow checks of RefCell<T> ensure data integrity while enabling mutable access to the shared data,
thus avoiding data races and maintaining memory safety.
Together, Rc<T> and RefCell<T> offer a powerful combination for handling shared ownership with interior mutability,
making them valuable tools for scenarios where multiple parts of the program need to access and
modify the same data.
*/
