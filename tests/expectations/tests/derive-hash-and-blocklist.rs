#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub struct BlocklistMe(u8);

/// Because this type contains a blocklisted type, it should not derive Hash.
#[repr(C)]
pub struct ShouldNotDeriveHash {
    pub a: BlocklistMe,
}
#[test]
fn bindgen_test_layout_ShouldNotDeriveHash() {
    assert_eq!(
        ::std::mem::size_of::<ShouldNotDeriveHash>(),
        1usize,
        concat!("Size of: ", stringify!(ShouldNotDeriveHash))
    );
    assert_eq!(
        ::std::mem::align_of::<ShouldNotDeriveHash>(),
        1usize,
        concat!("Alignment of ", stringify!(ShouldNotDeriveHash))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<ShouldNotDeriveHash>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ShouldNotDeriveHash),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
}
impl Default for ShouldNotDeriveHash {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
