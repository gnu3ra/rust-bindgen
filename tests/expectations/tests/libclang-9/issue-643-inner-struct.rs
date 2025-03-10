#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct rte_ring {
    pub memzone: *mut rte_memzone,
    pub prod: rte_ring_prod,
    pub cons: rte_ring_cons,
    pub ring: __IncompleteArrayField<*mut ::std::os::raw::c_void>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_ring_prod {
    pub watermark: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_rte_ring_prod() {
    assert_eq!(
        ::std::mem::size_of::<rte_ring_prod>(),
        4usize,
        concat!("Size of: ", stringify!(rte_ring_prod))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ring_prod>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_ring_prod))
    );
    fn test_field_watermark() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_ring_prod>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).watermark) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ring_prod),
                "::",
                stringify!(watermark)
            )
        );
    }
    test_field_watermark();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_ring_cons {
    pub sc_dequeue: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_rte_ring_cons() {
    assert_eq!(
        ::std::mem::size_of::<rte_ring_cons>(),
        4usize,
        concat!("Size of: ", stringify!(rte_ring_cons))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ring_cons>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_ring_cons))
    );
    fn test_field_sc_dequeue() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_ring_cons>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sc_dequeue) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ring_cons),
                "::",
                stringify!(sc_dequeue)
            )
        );
    }
    test_field_sc_dequeue();
}
#[test]
fn bindgen_test_layout_rte_ring() {
    assert_eq!(
        ::std::mem::size_of::<rte_ring>(),
        16usize,
        concat!("Size of: ", stringify!(rte_ring))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_ring>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_ring))
    );
    fn test_field_memzone() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_ring>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).memzone) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ring),
                "::",
                stringify!(memzone)
            )
        );
    }
    test_field_memzone();
    fn test_field_prod() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_ring>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).prod) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ring),
                "::",
                stringify!(prod)
            )
        );
    }
    test_field_prod();
    fn test_field_cons() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_ring>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cons) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ring),
                "::",
                stringify!(cons)
            )
        );
    }
    test_field_cons();
    fn test_field_ring() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_ring>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ring) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_ring),
                "::",
                stringify!(ring)
            )
        );
    }
    test_field_ring();
}
impl Default for rte_ring {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct rte_memzone {
    pub _address: u8,
}
