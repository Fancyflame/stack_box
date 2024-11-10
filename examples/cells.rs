use std::cell::RefCell;

use stack_box::FitStackBox;

type CellBox<T> = FitStackBox!(RefCell<T>, RefCell<u32>);

fn main() {
    let cb = CellBox::new(RefCell::new(10u32)); // internal mutability
    let r1 = cb.borrow();
    let r2 = cb.borrow();
    drop((r1, r2));
    *cb.borrow_mut() = 99;
}
