#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// This is a constant that has a docstring
///
/// And expected to be found in generated bindings code too.
pub const FOO: ::std::os::raw::c_int = 1;
/// This is a constant that has a docstring
///
/// And expected to be found in generated bindings code too.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Bar {
    pub baz: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(
        ::std::mem::size_of::<Bar>(),
        4usize,
        concat!("Size of: ", stringify!(Bar))
    );
    assert_eq!(
        ::std::mem::align_of::<Bar>(),
        4usize,
        concat!("Alignment of ", stringify!(Bar))
    );
    fn test_field_baz() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Bar>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).baz) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Bar),
                "::",
                stringify!(baz)
            )
        );
    }
    test_field_baz();
}
