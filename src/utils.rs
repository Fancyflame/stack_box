use core::ptr::null;

pub struct MetadataApplicator<T: ?Sized> {
    metadata_pointer: *const T,
}

impl<T: ?Sized> MetadataApplicator<T> {
    pub fn new(metadata_pointer: *const T) -> Self {
        let with_diff_addr: *mut () = (metadata_pointer as *mut ()).wrapping_byte_add(1);

        let patched: *mut T = unsafe { with_metadata_unchecked(with_diff_addr, metadata_pointer) };

        if patched as *mut () != with_diff_addr {
            panic!("cannot figure out memory layout of the unsize pointer");
        }

        Self { metadata_pointer }
    }

    pub fn new_sized() -> Self
    where
        T: Sized,
    {
        Self {
            metadata_pointer: null(),
        }
    }

    pub fn apply_metadata(&self, addr: *mut ()) -> *mut T {
        unsafe { with_metadata_unchecked(addr, self.metadata_pointer) }
    }
}

unsafe fn with_metadata_unchecked<T: ?Sized>(addr: *mut (), mut meta: *const T) -> *mut T {
    // assume address part of a fat pointer is the leading usize-sized bytes
    let meta_ptr: *mut *mut () = (&raw mut meta).cast();
    meta_ptr.write(addr);
    meta as _
}
