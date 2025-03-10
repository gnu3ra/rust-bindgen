#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub union UnionWithDtor {
    pub mFoo: ::std::os::raw::c_int,
    pub mBar: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_UnionWithDtor() {
    assert_eq!(
        ::std::mem::size_of::<UnionWithDtor>(),
        8usize,
        concat!("Size of: ", stringify!(UnionWithDtor))
    );
    assert_eq!(
        ::std::mem::align_of::<UnionWithDtor>(),
        8usize,
        concat!("Alignment of ", stringify!(UnionWithDtor))
    );
    fn test_field_mFoo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UnionWithDtor>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mFoo) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(UnionWithDtor),
                "::",
                stringify!(mFoo)
            )
        );
    }
    test_field_mFoo();
    fn test_field_mBar() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<UnionWithDtor>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mBar) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(UnionWithDtor),
                "::",
                stringify!(mBar)
            )
        );
    }
    test_field_mBar();
}
extern "C" {
    #[link_name = "\u{1}_ZN13UnionWithDtorD1Ev"]
    pub fn UnionWithDtor_UnionWithDtor_destructor(this: *mut UnionWithDtor);
}
impl Default for UnionWithDtor {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl UnionWithDtor {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        UnionWithDtor_UnionWithDtor_destructor(self)
    }
}
