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
    pub fn new(storage: Storage) -> Self {
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
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
pub const RTE_CACHE_LINE_MIN_SIZE: u32 = 64;
pub const RTE_CACHE_LINE_SIZE: u32 = 64;
pub type phys_addr_t = u64;
pub type MARKER = [*mut ::std::os::raw::c_void; 0usize];
pub type MARKER8 = [u8; 0usize];
pub type MARKER64 = [u64; 0usize];
/// The atomic counter structure.
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_atomic16_t {
    ///< An internal counter value.
    pub cnt: i16,
}
#[test]
fn bindgen_test_layout_rte_atomic16_t() {
    assert_eq!(
        ::std::mem::size_of::<rte_atomic16_t>(),
        2usize,
        concat!("Size of: ", stringify!(rte_atomic16_t))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_atomic16_t>(),
        2usize,
        concat!("Alignment of ", stringify!(rte_atomic16_t))
    );
    fn test_field_cnt() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_atomic16_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cnt) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_atomic16_t),
                "::",
                stringify!(cnt)
            )
        );
    }
    test_field_cnt();
}
impl Clone for rte_atomic16_t {
    fn clone(&self) -> Self {
        *self
    }
}
/// The generic rte_mbuf, containing a packet mbuf.
#[repr(C)]
pub struct rte_mbuf {
    pub cacheline0: MARKER,
    ///< Virtual address of segment buffer.
    pub buf_addr: *mut ::std::os::raw::c_void,
    ///< Physical address of segment buffer.
    pub buf_physaddr: phys_addr_t,
    ///< Length of segment buffer.
    pub buf_len: u16,
    pub rearm_data: MARKER8,
    pub data_off: u16,
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_1,
    ///< Number of segments.
    pub nb_segs: u8,
    ///< Input port.
    pub port: u8,
    ///< Offload features.
    pub ol_flags: u64,
    pub rx_descriptor_fields1: MARKER,
    pub __bindgen_anon_2: rte_mbuf__bindgen_ty_2,
    ///< Total pkt len: sum of all segments.
    pub pkt_len: u32,
    ///< Amount of data in segment buffer.
    pub data_len: u16,
    /// VLAN TCI (CPU order), valid if PKT_RX_VLAN_STRIPPED is set.
    pub vlan_tci: u16,
    ///< hash information
    pub hash: rte_mbuf__bindgen_ty_3,
    ///< Sequence number. See also rte_reorder_insert()
    pub seqn: u32,
    /// Outer VLAN TCI (CPU order), valid if PKT_RX_QINQ_STRIPPED is set.
    pub vlan_tci_outer: u16,
    pub cacheline1: MARKER,
    pub __bindgen_anon_3: rte_mbuf__bindgen_ty_4,
    ///< Pool from which mbuf was allocated.
    pub pool: *mut rte_mempool,
    ///< Next segment of scattered packet.
    pub next: *mut rte_mbuf,
    pub __bindgen_anon_4: rte_mbuf__bindgen_ty_5,
    /// Size of the application private data. In case of an indirect
    /// mbuf, it stores the direct mbuf private data size.
    pub priv_size: u16,
    /// Timesync flags for use with IEEE1588.
    pub timesync: u16,
    pub __bindgen_padding_0: [u32; 7usize],
}
/// 16-bit Reference counter.
/// It should only be accessed using the following functions:
/// rte_mbuf_refcnt_update(), rte_mbuf_refcnt_read(), and
/// rte_mbuf_refcnt_set(). The functionality of these functions (atomic,
/// or non-atomic) is controlled by the CONFIG_RTE_MBUF_REFCNT_ATOMIC
/// config option.
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_1 {
    ///< Atomically accessed refcnt
    pub refcnt_atomic: __BindgenUnionField<rte_atomic16_t>,
    ///< Non-atomically accessed refcnt
    pub refcnt: __BindgenUnionField<u16>,
    pub bindgen_union_field: u16,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_1>(),
        2usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_1))
    );
    fn test_field_refcnt_atomic() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).refcnt_atomic) as usize -
                    ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_1),
                "::",
                stringify!(refcnt_atomic)
            )
        );
    }
    test_field_refcnt_atomic();
    fn test_field_refcnt() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).refcnt) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_1),
                "::",
                stringify!(refcnt)
            )
        );
    }
    test_field_refcnt();
}
impl Clone for rte_mbuf__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_2 {
    ///< L2/L3/L4 and tunnel information.
    pub packet_type: __BindgenUnionField<u32>,
    pub __bindgen_anon_1:
        __BindgenUnionField<rte_mbuf__bindgen_ty_2__bindgen_ty_1>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
    pub __bindgen_align: [u32; 0usize],
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_2__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_2__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_2__bindgen_ty_1)
        )
    );
}
impl Clone for rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
    #[inline]
    pub fn l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(8usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn tun_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(12usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_tun_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(12usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn inner_l2_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_inner_l2_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn inner_l3_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(20usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_inner_l3_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(20usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn inner_l4_type(&self) -> u32 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(24usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_inner_l4_type(&mut self, val: u32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        l2_type: u32,
        l3_type: u32,
        l4_type: u32,
        tun_type: u32,
        inner_l2_type: u32,
        inner_l3_type: u32,
        inner_l4_type: u32,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let l2_type: u32 = unsafe { ::std::mem::transmute(l2_type) };
            l2_type as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let l3_type: u32 = unsafe { ::std::mem::transmute(l3_type) };
            l3_type as u64
        });
        __bindgen_bitfield_unit.set(8usize, 4u8, {
            let l4_type: u32 = unsafe { ::std::mem::transmute(l4_type) };
            l4_type as u64
        });
        __bindgen_bitfield_unit.set(12usize, 4u8, {
            let tun_type: u32 = unsafe { ::std::mem::transmute(tun_type) };
            tun_type as u64
        });
        __bindgen_bitfield_unit.set(16usize, 4u8, {
            let inner_l2_type: u32 =
                unsafe { ::std::mem::transmute(inner_l2_type) };
            inner_l2_type as u64
        });
        __bindgen_bitfield_unit.set(20usize, 4u8, {
            let inner_l3_type: u32 =
                unsafe { ::std::mem::transmute(inner_l3_type) };
            inner_l3_type as u64
        });
        __bindgen_bitfield_unit.set(24usize, 4u8, {
            let inner_l4_type: u32 =
                unsafe { ::std::mem::transmute(inner_l4_type) };
            inner_l4_type as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_2))
    );
    fn test_field_packet_type() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_2>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).packet_type) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_2),
                "::",
                stringify!(packet_type)
            )
        );
    }
    test_field_packet_type();
}
impl Clone for rte_mbuf__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3 {
    ///< RSS hash result if RSS enabled
    pub rss: __BindgenUnionField<u32>,
    ///< Filter identifier if FDIR enabled
    pub fdir: __BindgenUnionField<rte_mbuf__bindgen_ty_3__bindgen_ty_1>,
    ///< Hierarchical scheduler
    pub sched: __BindgenUnionField<rte_mbuf__bindgen_ty_3__bindgen_ty_2>,
    ///< User defined tags. See rte_distributor_process()
    pub usr: __BindgenUnionField<u32>,
    pub bindgen_union_field: [u32; 2usize],
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1 {
    pub __bindgen_anon_1: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
    pub hi: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    pub __bindgen_anon_1: __BindgenUnionField<
        rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1,
    >,
    pub lo: __BindgenUnionField<u32>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
    pub hash: u16,
    pub id: u16,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1(
) {
    assert_eq ! (:: std :: mem :: size_of :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > () , 4usize , concat ! ("Size of: " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1)));
    assert_eq ! (:: std :: mem :: align_of :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > () , 2usize , concat ! ("Alignment of " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1)));
    fn test_field_hash() {
        assert_eq ! (unsafe { let uninit = :: std :: mem :: MaybeUninit :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > :: uninit () ; let ptr = uninit . as_ptr () ; :: std :: ptr :: addr_of ! ((* ptr) . hash) as usize - ptr as usize } , 0usize , concat ! ("Offset of field: " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1) , "::" , stringify ! (hash)));
    }
    test_field_hash();
    fn test_field_id() {
        assert_eq ! (unsafe { let uninit = :: std :: mem :: MaybeUninit :: < rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 > :: uninit () ; let ptr = uninit . as_ptr () ; :: std :: ptr :: addr_of ! ((* ptr) . id) as usize - ptr as usize } , 2usize , concat ! ("Offset of field: " , stringify ! (rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1) , "::" , stringify ! (id)));
    }
    test_field_id();
}
impl Clone
    for rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
{
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1>(
        ),
        4usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<
            rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
        >(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1)
        )
    );
    fn test_field_lo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lo) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(lo)
            )
        );
    }
    test_field_lo();
}
impl Clone for rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1)
        )
    );
    fn test_field_hi() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hi) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_1),
                "::",
                stringify!(hi)
            )
        );
    }
    test_field_hi();
}
impl Clone for rte_mbuf__bindgen_ty_3__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_2 {
    pub lo: u32,
    pub hi: u32,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2)
        )
    );
    fn test_field_lo() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    rte_mbuf__bindgen_ty_3__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lo) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2),
                "::",
                stringify!(lo)
            )
        );
    }
    test_field_lo();
    fn test_field_hi() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    rte_mbuf__bindgen_ty_3__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hi) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3__bindgen_ty_2),
                "::",
                stringify!(hi)
            )
        );
    }
    test_field_hi();
}
impl Clone for rte_mbuf__bindgen_ty_3__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_3>(),
        8usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_3))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_3>(),
        4usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_3))
    );
    fn test_field_rss() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rss) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3),
                "::",
                stringify!(rss)
            )
        );
    }
    test_field_rss();
    fn test_field_fdir() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fdir) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3),
                "::",
                stringify!(fdir)
            )
        );
    }
    test_field_fdir();
    fn test_field_sched() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sched) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3),
                "::",
                stringify!(sched)
            )
        );
    }
    test_field_sched();
    fn test_field_usr() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_3>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).usr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_3),
                "::",
                stringify!(usr)
            )
        );
    }
    test_field_usr();
}
impl Clone for rte_mbuf__bindgen_ty_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_4 {
    ///< Can be used for external metadata
    pub userdata: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    ///< Allow 8-byte userdata on 32-bit
    pub udata64: __BindgenUnionField<u64>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_4() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_4>(),
        8usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_4))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_4>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_4))
    );
    fn test_field_userdata() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).userdata) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_4),
                "::",
                stringify!(userdata)
            )
        );
    }
    test_field_userdata();
    fn test_field_udata64() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_4>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).udata64) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_4),
                "::",
                stringify!(udata64)
            )
        );
    }
    test_field_udata64();
}
impl Clone for rte_mbuf__bindgen_ty_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_5 {
    ///< combined for easy fetch
    pub tx_offload: __BindgenUnionField<u64>,
    pub __bindgen_anon_1:
        __BindgenUnionField<rte_mbuf__bindgen_ty_5__bindgen_ty_1>,
    pub bindgen_union_field: u64,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 7usize]>,
    pub __bindgen_align: [u64; 0usize],
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_5__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(rte_mbuf__bindgen_ty_5__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(rte_mbuf__bindgen_ty_5__bindgen_ty_1)
        )
    );
}
impl Clone for rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
    #[inline]
    pub fn l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u64)
        }
    }
    #[inline]
    pub fn set_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(7usize, 9u8) as u64)
        }
    }
    #[inline]
    pub fn set_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 9u8, val as u64)
        }
    }
    #[inline]
    pub fn l4_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(16usize, 8u8) as u64)
        }
    }
    #[inline]
    pub fn set_l4_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn tso_segsz(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(24usize, 16u8) as u64)
        }
    }
    #[inline]
    pub fn set_tso_segsz(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn outer_l3_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(40usize, 9u8) as u64)
        }
    }
    #[inline]
    pub fn set_outer_l3_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(40usize, 9u8, val as u64)
        }
    }
    #[inline]
    pub fn outer_l2_len(&self) -> u64 {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(49usize, 7u8) as u64)
        }
    }
    #[inline]
    pub fn set_outer_l2_len(&mut self, val: u64) {
        unsafe {
            let val: u64 = ::std::mem::transmute(val);
            self._bitfield_1.set(49usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        l2_len: u64,
        l3_len: u64,
        l4_len: u64,
        tso_segsz: u64,
        outer_l3_len: u64,
        outer_l2_len: u64,
    ) -> __BindgenBitfieldUnit<[u8; 7usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 7usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let l2_len: u64 = unsafe { ::std::mem::transmute(l2_len) };
            l2_len as u64
        });
        __bindgen_bitfield_unit.set(7usize, 9u8, {
            let l3_len: u64 = unsafe { ::std::mem::transmute(l3_len) };
            l3_len as u64
        });
        __bindgen_bitfield_unit.set(16usize, 8u8, {
            let l4_len: u64 = unsafe { ::std::mem::transmute(l4_len) };
            l4_len as u64
        });
        __bindgen_bitfield_unit.set(24usize, 16u8, {
            let tso_segsz: u64 = unsafe { ::std::mem::transmute(tso_segsz) };
            tso_segsz as u64
        });
        __bindgen_bitfield_unit.set(40usize, 9u8, {
            let outer_l3_len: u64 =
                unsafe { ::std::mem::transmute(outer_l3_len) };
            outer_l3_len as u64
        });
        __bindgen_bitfield_unit.set(49usize, 7u8, {
            let outer_l2_len: u64 =
                unsafe { ::std::mem::transmute(outer_l2_len) };
            outer_l2_len as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf__bindgen_ty_5() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf__bindgen_ty_5>(),
        8usize,
        concat!("Size of: ", stringify!(rte_mbuf__bindgen_ty_5))
    );
    assert_eq!(
        ::std::mem::align_of::<rte_mbuf__bindgen_ty_5>(),
        8usize,
        concat!("Alignment of ", stringify!(rte_mbuf__bindgen_ty_5))
    );
    fn test_field_tx_offload() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<rte_mbuf__bindgen_ty_5>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tx_offload) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf__bindgen_ty_5),
                "::",
                stringify!(tx_offload)
            )
        );
    }
    test_field_tx_offload();
}
impl Clone for rte_mbuf__bindgen_ty_5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_rte_mbuf() {
    assert_eq!(
        ::std::mem::size_of::<rte_mbuf>(),
        128usize,
        concat!("Size of: ", stringify!(rte_mbuf))
    );
    fn test_field_cacheline0() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cacheline0) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(cacheline0)
            )
        );
    }
    test_field_cacheline0();
    fn test_field_buf_addr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).buf_addr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(buf_addr)
            )
        );
    }
    test_field_buf_addr();
    fn test_field_buf_physaddr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).buf_physaddr) as usize -
                    ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(buf_physaddr)
            )
        );
    }
    test_field_buf_physaddr();
    fn test_field_buf_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).buf_len) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(buf_len)
            )
        );
    }
    test_field_buf_len();
    fn test_field_rearm_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rearm_data) as usize - ptr as usize
            },
            18usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(rearm_data)
            )
        );
    }
    test_field_rearm_data();
    fn test_field_data_off() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data_off) as usize - ptr as usize
            },
            18usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(data_off)
            )
        );
    }
    test_field_data_off();
    fn test_field_nb_segs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).nb_segs) as usize - ptr as usize
            },
            22usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(nb_segs)
            )
        );
    }
    test_field_nb_segs();
    fn test_field_port() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).port) as usize - ptr as usize
            },
            23usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(port)
            )
        );
    }
    test_field_port();
    fn test_field_ol_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ol_flags) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(ol_flags)
            )
        );
    }
    test_field_ol_flags();
    fn test_field_rx_descriptor_fields1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rx_descriptor_fields1) as usize -
                    ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(rx_descriptor_fields1)
            )
        );
    }
    test_field_rx_descriptor_fields1();
    fn test_field_pkt_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).pkt_len) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(pkt_len)
            )
        );
    }
    test_field_pkt_len();
    fn test_field_data_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data_len) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(data_len)
            )
        );
    }
    test_field_data_len();
    fn test_field_vlan_tci() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).vlan_tci) as usize - ptr as usize
            },
            42usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(vlan_tci)
            )
        );
    }
    test_field_vlan_tci();
    fn test_field_hash() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hash) as usize - ptr as usize
            },
            44usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(hash)
            )
        );
    }
    test_field_hash();
    fn test_field_seqn() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).seqn) as usize - ptr as usize
            },
            52usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(seqn)
            )
        );
    }
    test_field_seqn();
    fn test_field_vlan_tci_outer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).vlan_tci_outer) as usize -
                    ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(vlan_tci_outer)
            )
        );
    }
    test_field_vlan_tci_outer();
    fn test_field_cacheline1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cacheline1) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(cacheline1)
            )
        );
    }
    test_field_cacheline1();
    fn test_field_pool() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).pool) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(pool)
            )
        );
    }
    test_field_pool();
    fn test_field_next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(next)
            )
        );
    }
    test_field_next();
    fn test_field_priv_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).priv_size) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(priv_size)
            )
        );
    }
    test_field_priv_size();
    fn test_field_timesync() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<rte_mbuf>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).timesync) as usize - ptr as usize
            },
            98usize,
            concat!(
                "Offset of field: ",
                stringify!(rte_mbuf),
                "::",
                stringify!(timesync)
            )
        );
    }
    test_field_timesync();
}
impl Default for rte_mbuf {
    fn default() -> Self {
        unsafe {
            let mut s: Self = ::std::mem::uninitialized();
            ::std::ptr::write_bytes(&mut s, 0, 1);
            s
        }
    }
}
///< Pool from which mbuf was allocated.
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct rte_mempool {
    pub _address: u8,
}
impl Clone for rte_mempool {
    fn clone(&self) -> Self {
        *self
    }
}
