pub(crate) fn with_metadata<T: ?Sized>(addr: *const u8, mut meta: *const T) -> *const T {
    let meta_ptr = (&raw mut meta).cast::<*const u8>();
    unsafe {
        meta_ptr.write(addr);
    }
    meta
}

pub(crate) fn with_metadata_mut<T: ?Sized>(addr: *mut u8, meta: *const T) -> *mut T {
    with_metadata(addr, meta) as _
}
