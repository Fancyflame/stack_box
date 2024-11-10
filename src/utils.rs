pub(crate) fn with_metadata<T: ?Sized>(addr: *mut (), mut meta: *const T) -> *mut T {
    let meta_ptr = (&raw mut meta).cast::<*mut ()>();
    unsafe {
        meta_ptr.write(addr);
    }
    meta as *mut T
}
