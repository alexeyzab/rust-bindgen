/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy)]
pub struct _bindgen_ty_1 {
    pub _bitfield_1: u32,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_bindgen_ty_1>() , 4usize);
    assert_eq!(::std::mem::align_of::<_bindgen_ty_1>() , 4usize);
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
impl _bindgen_ty_1 {
    #[inline]
    pub fn pad3(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 & (16777215usize as u32))
                                       >> 0u32) as u32)
        }
    }
    #[inline]
    pub fn set_pad3(&mut self, val: ::std::os::raw::c_uint) {
        self._bitfield_1 &= !(16777215usize as u32);
        self._bitfield_1 |=
            ((val as u32 as u32) << 0u32) & (16777215usize as u32);
    }
    #[inline]
    pub fn type_(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(((self._bitfield_1 &
                                        (4278190080usize as u32)) >> 24u32) as
                                      u32)
        }
    }
    #[inline]
    pub fn set_type(&mut self, val: ::std::os::raw::c_uint) {
        self._bitfield_1 &= !(4278190080usize as u32);
        self._bitfield_1 |=
            ((val as u32 as u32) << 24u32) & (4278190080usize as u32);
    }
}
pub use _bindgen_ty_1 as mach_msg_type_descriptor_t;
