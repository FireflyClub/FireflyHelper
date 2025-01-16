use core::iter::once;
use std::slice;
use std::ffi::OsStr;
use std::io::Cursor;

use std::os::windows::ffi::OsStrExt;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::core::PCWSTR;

use winapi::um::winnt::IMAGE_DOS_HEADER;
use winapi::um::winnt::IMAGE_NT_HEADERS;

use patternscan::scan;

pub fn __wide_str(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub unsafe fn try_get_base_address(module_name: &str) -> Option<usize> {
    let w_module_name = __wide_str(module_name);
    match GetModuleHandleW(PCWSTR::from_raw(w_module_name.as_ptr())) {
        Ok(module) => Some(module.0 as usize),
        Err(_) => None
    }
}

#[allow(dead_code)]
pub unsafe fn pattern_scan(base: usize, pattern: &str, num: usize) -> Option<*mut u8> {
    let module_handle_ptr: *const _ = base as *const _;
    let mod_base = base as *const u8;
    let dos_header = unsafe { &*(mod_base as *const IMAGE_DOS_HEADER) };
    let nt_headers = unsafe { &*((mod_base.offset(dos_header.e_lfanew as isize)) as *const IMAGE_NT_HEADERS) };
    let size_of_image = nt_headers.OptionalHeader.SizeOfImage as usize;
    let memory_slice: &[u8] = unsafe { slice::from_raw_parts(module_handle_ptr, size_of_image) };
    let mut cursor = Cursor::new(memory_slice);

    let loc = scan(&mut cursor, pattern).unwrap();
    match loc.iter().nth(num) {
        None => None,
        Some(loc) => Some((module_handle_ptr.wrapping_add(*loc)) as *mut u8),
    }
}
