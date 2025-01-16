use std::ffi::CStr;

pub static mut STRING_ADDR: Option<*mut u8> = None;
type MarshalPtrToStringAnsi = unsafe extern "fastcall" fn(*const u8) -> *const u8;

pub unsafe fn ptr_to_ansi(content: &CStr) -> *const u8 {
    if STRING_ADDR.is_none() {
        panic!("string_addr is not initialized.");
    }

    let func: MarshalPtrToStringAnsi = std::mem::transmute(STRING_ADDR.unwrap());
    func(content.to_bytes_with_nul().as_ptr())
}

pub unsafe fn read_il2cpp_str(addr: u64) -> String {
    let str_length: u32 = *(addr.wrapping_add(16) as *const u32);
    let str_ptr = addr.wrapping_add(20) as *const u8;
    let slice = std::slice::from_raw_parts(str_ptr as *const u8, (str_length * 2) as usize);
    String::from_utf16le(slice).unwrap()
}