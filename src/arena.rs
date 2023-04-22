use std::ffi::c_void;
use std::ptr;
pub(crate) struct Arena {
    n: usize,
    pub buf: Vec<u8>,
}

impl Arena {
    pub fn get_pointer(&self, offset: usize) -> *mut std::ffi::c_void {
        if offset == 0 {
            return ptr::null_mut();
        }
        &self.buf[offset as usize] as *const u8 as *mut c_void
    } 
}