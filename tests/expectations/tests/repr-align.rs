#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(feature = "nightly")]

#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
    pub c: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_a() {
    assert_eq!(
        ::std::mem::size_of::<a>(),
        8usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        8usize,
        concat!("Alignment of ", stringify!(a))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<a>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(a), "::", stringify!(b))
        );
    }
    test_field_b();
    fn test_field_c() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<a>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize
            },
            4usize,
            concat!("Offset of field: ", stringify!(a), "::", stringify!(c))
        );
    }
    test_field_c();
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Default, Copy, Clone)]
pub struct b {
    pub b: ::std::os::raw::c_int,
    pub c: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_b() {
    assert_eq!(
        ::std::mem::size_of::<b>(),
        8usize,
        concat!("Size of: ", stringify!(b))
    );
    assert_eq!(
        ::std::mem::align_of::<b>(),
        8usize,
        concat!("Alignment of ", stringify!(b))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<b>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(b), "::", stringify!(b))
        );
    }
    test_field_b();
    fn test_field_c() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<b>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c) as usize - ptr as usize
            },
            4usize,
            concat!("Offset of field: ", stringify!(b), "::", stringify!(c))
        );
    }
    test_field_c();
}
