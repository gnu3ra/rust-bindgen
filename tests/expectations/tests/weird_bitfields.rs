#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum nsStyleSVGOpacitySource {
    eStyleSVGOpacitySource_Normal = 0,
    eStyleSVGOpacitySource_ContextFillOpacity = 1,
    eStyleSVGOpacitySource_ContextStrokeOpacity = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Weird {
    pub mStrokeDasharrayLength: ::std::os::raw::c_uint,
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub mClipRule: ::std::os::raw::c_uchar,
    pub mColorInterpolation: ::std::os::raw::c_uchar,
    pub mColorInterpolationFilters: ::std::os::raw::c_uchar,
    pub mFillRule: ::std::os::raw::c_uchar,
    pub mImageRendering: ::std::os::raw::c_uchar,
    pub mPaintOrder: ::std::os::raw::c_uchar,
    pub mShapeRendering: ::std::os::raw::c_uchar,
    pub mStrokeLinecap: ::std::os::raw::c_uchar,
    pub mStrokeLinejoin: ::std::os::raw::c_uchar,
    pub mTextAnchor: ::std::os::raw::c_uchar,
    pub mTextRendering: ::std::os::raw::c_uchar,
    pub _bitfield_align_2: [u8; 0],
    pub _bitfield_2: __BindgenBitfieldUnit<[u8; 2usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_Weird() {
    assert_eq!(
        ::std::mem::size_of::<Weird>(),
        24usize,
        concat!("Size of: ", stringify!(Weird))
    );
    assert_eq!(
        ::std::mem::align_of::<Weird>(),
        4usize,
        concat!("Alignment of ", stringify!(Weird))
    );
    fn test_field_mStrokeDasharrayLength() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mStrokeDasharrayLength) as usize -
                    ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mStrokeDasharrayLength)
            )
        );
    }
    test_field_mStrokeDasharrayLength();
    fn test_field_mClipRule() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mClipRule) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mClipRule)
            )
        );
    }
    test_field_mClipRule();
    fn test_field_mColorInterpolation() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mColorInterpolation) as usize -
                    ptr as usize
            },
            9usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mColorInterpolation)
            )
        );
    }
    test_field_mColorInterpolation();
    fn test_field_mColorInterpolationFilters() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mColorInterpolationFilters) as usize -
                    ptr as usize
            },
            10usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mColorInterpolationFilters)
            )
        );
    }
    test_field_mColorInterpolationFilters();
    fn test_field_mFillRule() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mFillRule) as usize - ptr as usize
            },
            11usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mFillRule)
            )
        );
    }
    test_field_mFillRule();
    fn test_field_mImageRendering() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mImageRendering) as usize -
                    ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mImageRendering)
            )
        );
    }
    test_field_mImageRendering();
    fn test_field_mPaintOrder() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mPaintOrder) as usize - ptr as usize
            },
            13usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mPaintOrder)
            )
        );
    }
    test_field_mPaintOrder();
    fn test_field_mShapeRendering() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mShapeRendering) as usize -
                    ptr as usize
            },
            14usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mShapeRendering)
            )
        );
    }
    test_field_mShapeRendering();
    fn test_field_mStrokeLinecap() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mStrokeLinecap) as usize -
                    ptr as usize
            },
            15usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mStrokeLinecap)
            )
        );
    }
    test_field_mStrokeLinecap();
    fn test_field_mStrokeLinejoin() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mStrokeLinejoin) as usize -
                    ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mStrokeLinejoin)
            )
        );
    }
    test_field_mStrokeLinejoin();
    fn test_field_mTextAnchor() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mTextAnchor) as usize - ptr as usize
            },
            17usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mTextAnchor)
            )
        );
    }
    test_field_mTextAnchor();
    fn test_field_mTextRendering() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Weird>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).mTextRendering) as usize -
                    ptr as usize
            },
            18usize,
            concat!(
                "Offset of field: ",
                stringify!(Weird),
                "::",
                stringify!(mTextRendering)
            )
        );
    }
    test_field_mTextRendering();
}
impl Default for Weird {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
impl Weird {
    #[inline]
    pub fn bitTest(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 16u8) as u32)
        }
    }
    #[inline]
    pub fn set_bitTest(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn bitTest2(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 15u8) as u32)
        }
    }
    #[inline]
    pub fn set_bitTest2(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 15u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        bitTest: ::std::os::raw::c_uint,
        bitTest2: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 16u8, {
            let bitTest: u32 = unsafe { ::std::mem::transmute(bitTest) };
            bitTest as u64
        });
        __bindgen_bitfield_unit.set(16usize, 15u8, {
            let bitTest2: u32 = unsafe { ::std::mem::transmute(bitTest2) };
            bitTest2 as u64
        });
        __bindgen_bitfield_unit
    }
    #[inline]
    pub fn mFillOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(0usize, 3u8) as u32)
        }
    }
    #[inline]
    pub fn set_mFillOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(0usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn mStrokeOpacitySource(&self) -> nsStyleSVGOpacitySource {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(3usize, 3u8) as u32)
        }
    }
    #[inline]
    pub fn set_mStrokeOpacitySource(&mut self, val: nsStyleSVGOpacitySource) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_2.set(3usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn mStrokeDasharrayFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(6usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_mStrokeDasharrayFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn mStrokeDashoffsetFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(7usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_mStrokeDashoffsetFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn mStrokeWidthFromObject(&self) -> bool {
        unsafe {
            ::std::mem::transmute(self._bitfield_2.get(8usize, 1u8) as u8)
        }
    }
    #[inline]
    pub fn set_mStrokeWidthFromObject(&mut self, val: bool) {
        unsafe {
            let val: u8 = ::std::mem::transmute(val);
            self._bitfield_2.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_2(
        mFillOpacitySource: nsStyleSVGOpacitySource,
        mStrokeOpacitySource: nsStyleSVGOpacitySource,
        mStrokeDasharrayFromObject: bool,
        mStrokeDashoffsetFromObject: bool,
        mStrokeWidthFromObject: bool,
    ) -> __BindgenBitfieldUnit<[u8; 2usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 3u8, {
            let mFillOpacitySource: u32 =
                unsafe { ::std::mem::transmute(mFillOpacitySource) };
            mFillOpacitySource as u64
        });
        __bindgen_bitfield_unit.set(3usize, 3u8, {
            let mStrokeOpacitySource: u32 =
                unsafe { ::std::mem::transmute(mStrokeOpacitySource) };
            mStrokeOpacitySource as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let mStrokeDasharrayFromObject: u8 =
                unsafe { ::std::mem::transmute(mStrokeDasharrayFromObject) };
            mStrokeDasharrayFromObject as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let mStrokeDashoffsetFromObject: u8 =
                unsafe { ::std::mem::transmute(mStrokeDashoffsetFromObject) };
            mStrokeDashoffsetFromObject as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let mStrokeWidthFromObject: u8 =
                unsafe { ::std::mem::transmute(mStrokeWidthFromObject) };
            mStrokeWidthFromObject as u64
        });
        __bindgen_bitfield_unit
    }
}
