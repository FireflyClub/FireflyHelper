use crate::{marshal, modules::{MhyContext, MhyModule, ModuleType}};
use anyhow::Result;
use ilhook::x64::Registers;

pub struct RES;
impl MhyModule for MhyContext<RES> {
    unsafe fn init(&mut self) -> Result<()> {
        if let Some(addr) = self.addr {
            self.interceptor.attach(
                addr as usize,
                hkaddr,
            )
        } else {
            Err(anyhow::anyhow!("Cannot find address."))
        }
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::RES
    }
}

pub unsafe extern "win64" fn hkaddr(reg: *mut Registers, _: usize) {
    let path = marshal::read_il2cpp_str((*reg).rcx);
    println!("Catch Path: {}", path);
}