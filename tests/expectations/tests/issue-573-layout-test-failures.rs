#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Outer {
    pub i: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AutoIdVector {
    pub ar: Outer,
}
#[test]
fn bindgen_test_layout_AutoIdVector() {
    assert_eq!(
        ::std::mem::size_of::<AutoIdVector>(),
        1usize,
        concat!("Size of: ", stringify!(AutoIdVector))
    );
    assert_eq!(
        ::std::mem::align_of::<AutoIdVector>(),
        1usize,
        concat!("Alignment of ", stringify!(AutoIdVector))
    );
    fn test_field_ar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<AutoIdVector>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(AutoIdVector),
                "::",
                stringify!(ar)
            )
        );
    }
    test_field_ar();
}
#[test]
fn __bindgen_test_layout_Outer_open0_int_close0_instantiation() {
    assert_eq!(
        ::std::mem::size_of::<Outer>(),
        1usize,
        concat!("Size of template specialization: ", stringify!(Outer))
    );
    assert_eq!(
        ::std::mem::align_of::<Outer>(),
        1usize,
        concat!("Alignment of template specialization: ", stringify!(Outer))
    );
}
