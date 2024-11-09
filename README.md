# stack_box

Put your tiny stuff on stack!

If you are sure your stuff has or uses less than an exact size, but the compiler doesn't see
and force you to put them on heap, this crate is for you.

```rust
use std::fmt::Debug;

use stack_box::{stackbox_coerce, FitStackBox};

// alias to a `StackBox` that could exactly contains a `[u32; 3]`
type U32Box<T> = FitStackBox!(T, [u32; 3]);

fn main() {
    let u16_box = U32Box::new(16u16); // ok
    let u32_box = U32Box::new([32u32; 3]); // ok

    // U32Box::new(0u64); // panic at compile time
    // U32Box::new([0u32; 4]); // panic at compile time
    assert!(U32Box::new_runtime_checked(0u64).is_err()); // align too large
    assert!(U32Box::new_runtime_checked([0u32; 4]).is_err()); // size too large

    let arr: [U32Box<dyn Debug>; 2] = [coerce!(u16_box), coerce!(u32_box)];

    for x in arr {
        dbg!(&*x);
    }
}
```

The expansion of `coerce` macro is also safe rust code.