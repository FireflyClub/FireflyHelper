use core::iter::once;
use std::slice;
use std::ffi::OsStr;
use std::io::Cursor;

use std::os::windows::ffi::OsStrExt;
use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetModuleFileNameW};
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

pub unsafe fn __get_module_file_name(module_name: &str) -> String {
    let w_module_name = __wide_str(module_name);
    match GetModuleHandleW(PCWSTR::from_raw(w_module_name.as_ptr())) {
        Ok(hmodule) => {
            const LEN: usize = 128;
            let mut buf: [u16; LEN] = [0; LEN];
            GetModuleFileNameW(hmodule, &mut buf);
            let mut vec = buf.to_vec();
            vec.retain(|&x| x > 0);
            let filename = String::from_utf16(&vec).unwrap();
            return filename;
        },
        Err(_) => String::from("")
    }
}

pub unsafe fn detect_version() {
    let file = __get_module_file_name("UnityPlayer.dll");
    let vec = std::fs::read(&file).unwrap();
    let content = String::from_utf8_lossy(&vec);
    let mut version: String = String::from("");
    if content.contains("OSBETAWin") || content.contains("CNBETAWin") {
        let index = content.find("BETAWin").unwrap();
        version = content[index-2..index+13].to_string();
    } else if content.contains(&*"CNPRODWin") || content.contains(&*"OSPRODWin") {
        let index = content.find("RELWin").unwrap();
        version = content[index-2..index+12].to_string();
    }
    println!("Detected version: {}", version);
}

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
