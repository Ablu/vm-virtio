/* automatically generated by rust-bindgen 0.63.0 */

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
pub const VRING_DESC_F_NEXT: u32 = 1;
pub const VRING_DESC_F_WRITE: u32 = 2;
pub const VRING_DESC_F_INDIRECT: u32 = 4;
pub const VRING_PACKED_DESC_F_AVAIL: u32 = 7;
pub const VRING_PACKED_DESC_F_USED: u32 = 15;
pub const VRING_USED_F_NO_NOTIFY: u32 = 1;
pub const VRING_AVAIL_F_NO_INTERRUPT: u32 = 1;
pub const VRING_PACKED_EVENT_FLAG_ENABLE: u32 = 0;
pub const VRING_PACKED_EVENT_FLAG_DISABLE: u32 = 1;
pub const VRING_PACKED_EVENT_FLAG_DESC: u32 = 2;
pub const VRING_PACKED_EVENT_F_WRAP_CTR: u32 = 15;
pub const VIRTIO_RING_F_INDIRECT_DESC: u32 = 28;
pub const VIRTIO_RING_F_EVENT_IDX: u32 = 29;
pub const VRING_AVAIL_ALIGN_SIZE: u32 = 2;
pub const VRING_USED_ALIGN_SIZE: u32 = 4;
pub const VRING_DESC_ALIGN_SIZE: u32 = 16;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __u32 = ::std::os::raw::c_uint;
pub type __u64 = ::std::os::raw::c_ulonglong;
pub type __le16 = __u16;
pub type __le32 = __u32;
pub type __le64 = __u64;
pub type __virtio16 = __u16;
pub type __virtio32 = __u32;
pub type __virtio64 = __u64;
#[doc = " struct vring_desc - Virtio ring descriptors,\n 16 bytes long. These can chain together via @next.\n\n @addr: buffer address (guest-physical)\n @len: buffer length\n @flags: descriptor flags\n @next: index of the next descriptor in the chain,\n        if the VRING_DESC_F_NEXT flag is set. We chain unused\n        descriptors via this, too."]
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_desc {
    pub addr: __virtio64,
    pub len: __virtio32,
    pub flags: __virtio16,
    pub next: __virtio16,
}
#[test]
fn bindgen_test_layout_vring_desc() {
    const UNINIT: ::std::mem::MaybeUninit<vring_desc> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring_desc>(),
        16usize,
        concat!("Size of: ", stringify!(vring_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_desc>(),
        8usize,
        concat!("Alignment of ", stringify!(vring_desc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).next) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_desc),
            "::",
            stringify!(next)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct vring_avail {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: __IncompleteArrayField<__virtio16>,
}
#[test]
fn bindgen_test_layout_vring_avail() {
    const UNINIT: ::std::mem::MaybeUninit<vring_avail> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring_avail>(),
        4usize,
        concat!("Size of: ", stringify!(vring_avail))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_avail>(),
        2usize,
        concat!("Alignment of ", stringify!(vring_avail))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_avail),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).idx) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_avail),
            "::",
            stringify!(idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ring) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_avail),
            "::",
            stringify!(ring)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_used_elem {
    pub id: __virtio32,
    pub len: __virtio32,
}
#[test]
fn bindgen_test_layout_vring_used_elem() {
    const UNINIT: ::std::mem::MaybeUninit<vring_used_elem> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring_used_elem>(),
        8usize,
        concat!("Size of: ", stringify!(vring_used_elem))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_used_elem>(),
        4usize,
        concat!("Alignment of ", stringify!(vring_used_elem))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used_elem),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used_elem),
            "::",
            stringify!(len)
        )
    );
}
pub type vring_used_elem_t = vring_used_elem;
#[repr(C)]
#[derive(Debug, Default)]
pub struct vring_used {
    pub flags: __virtio16,
    pub idx: __virtio16,
    pub ring: __IncompleteArrayField<vring_used_elem_t>,
}
#[test]
fn bindgen_test_layout_vring_used() {
    const UNINIT: ::std::mem::MaybeUninit<vring_used> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring_used>(),
        4usize,
        concat!("Size of: ", stringify!(vring_used))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_used>(),
        4usize,
        concat!("Alignment of ", stringify!(vring_used))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used),
            "::",
            stringify!(flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).idx) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used),
            "::",
            stringify!(idx)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ring) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_used),
            "::",
            stringify!(ring)
        )
    );
}
#[doc = " struct vring_desc - Virtio ring descriptors,\n 16 bytes long. These can chain together via @next.\n\n @addr: buffer address (guest-physical)\n @len: buffer length\n @flags: descriptor flags\n @next: index of the next descriptor in the chain,\n        if the VRING_DESC_F_NEXT flag is set. We chain unused\n        descriptors via this, too."]
pub type vring_desc_t = vring_desc;
pub type vring_avail_t = vring_avail;
pub type vring_used_t = vring_used;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct vring {
    pub num: ::std::os::raw::c_uint,
    pub desc: *mut vring_desc_t,
    pub avail: *mut vring_avail_t,
    pub used: *mut vring_used_t,
}
#[test]
fn bindgen_test_layout_vring() {
    const UNINIT: ::std::mem::MaybeUninit<vring> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring>(),
        32usize,
        concat!("Size of: ", stringify!(vring))
    );
    assert_eq!(
        ::std::mem::align_of::<vring>(),
        8usize,
        concat!("Alignment of ", stringify!(vring))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).num) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(num)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).desc) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(desc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).avail) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(avail)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).used) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vring),
            "::",
            stringify!(used)
        )
    );
}
impl Default for vring {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_packed_desc_event {
    pub off_wrap: __le16,
    pub flags: __le16,
}
#[test]
fn bindgen_test_layout_vring_packed_desc_event() {
    const UNINIT: ::std::mem::MaybeUninit<vring_packed_desc_event> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring_packed_desc_event>(),
        4usize,
        concat!("Size of: ", stringify!(vring_packed_desc_event))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_packed_desc_event>(),
        2usize,
        concat!("Alignment of ", stringify!(vring_packed_desc_event))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).off_wrap) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc_event),
            "::",
            stringify!(off_wrap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc_event),
            "::",
            stringify!(flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct vring_packed_desc {
    pub addr: __le64,
    pub len: __le32,
    pub id: __le16,
    pub flags: __le16,
}
#[test]
fn bindgen_test_layout_vring_packed_desc() {
    const UNINIT: ::std::mem::MaybeUninit<vring_packed_desc> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<vring_packed_desc>(),
        16usize,
        concat!("Size of: ", stringify!(vring_packed_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<vring_packed_desc>(),
        8usize,
        concat!("Alignment of ", stringify!(vring_packed_desc))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).addr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).len) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(len)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(vring_packed_desc),
            "::",
            stringify!(flags)
        )
    );
}
