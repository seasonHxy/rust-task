use std::cell::RefCell;
use std::rc::Rc;

trait Add {
    fn add(&self, other: &dyn Add) -> Rc<RefCell<dyn Add>>;
    fn get_value(&self) -> i32;
}

struct MyType {
    value: i32,
}

impl Add for MyType {
    fn add(&self, other: &dyn Add) -> Rc<RefCell<dyn Add>> {
        let sum = self.value + other.get_value();
        Rc::new(RefCell::new(MyType { value: sum }))
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let a: Rc<RefCell<dyn Add>> = Rc::new(RefCell::new(MyType { value: 15 }));
    let b: Rc<RefCell<dyn Add>> = Rc::new(RefCell::new(MyType { value: 10 }));

    let result = a.borrow().add(&*b.borrow());
    println!("Result: {}", result.borrow().get_value());
}