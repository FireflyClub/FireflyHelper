use std::ptr;
use anyhow::Result;
use windows::Win32::System::Memory::{
    VirtualProtect, PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS,
};

use crate::modules::{MhyContext, MhyModule, ModuleType};

pub struct Censorship;
impl MhyModule for MhyContext<Censorship> {
    unsafe fn init(&mut self) -> Result<()> {
        if let Some(addr) = self.addr {
            hkaddr(addr);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Cannot find address."))
        }
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Censorship
    }
}

unsafe extern "win64" fn hkaddr(addr: *mut u8) {
    let mut old_protect = PAGE_PROTECTION_FLAGS(0);
    let _ = VirtualProtect(
        addr as *mut _,
        1,
        PAGE_EXECUTE_READWRITE,
        &mut old_protect,
    );
    ptr::write(
        addr as *mut u8,
        0xC3,
    );
}
