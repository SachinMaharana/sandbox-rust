use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    let answer = Cell::new(42);
    assert_eq!(answer.get(), 42);
    answer.set(73);
    assert_eq!(answer.get(), 73);

    let greeting = RefCell::new("hello".to_string());

    assert_eq!(*greeting.borrow(), "hello");
    assert_eq!(greeting.borrow().len(), 5);

    *greeting.borrow_mut() = "hola".to_string();

    assert_eq!(*greeting.borrow(), "hola");
    {
        let mut gr = greeting.borrow_mut();
        *gr = "kol".to_string();
    }
    assert_eq!(*greeting.borrow(), "kol");

    let s = "hello_dolly".to_string();
    let rs1 = Rc::new(s);
    let rs2 = rs1.clone();

    println!("len {}, {}", rs1.len(), rs2.len());

}
