use stack_box::{auto_box::AutoBox, FitContainer};

type AutoU32Box<T> = AutoBox<T, FitContainer!(u32)>;

fn main() {
    assert!(AutoU32Box::is_stack(&AutoU32Box::new(0u32)));
    assert!(AutoU32Box::is_stack(&AutoU32Box::new(0u8)));
    assert!(AutoU32Box::is_heap(&AutoU32Box::new([0u32; 10])));
}
