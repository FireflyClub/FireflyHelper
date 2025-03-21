use crate::modules::{MhyContext, MhyModule, ModuleType};
use anyhow::Result;
use ilhook::x64::Registers;

pub struct RSAEncrypt;
impl MhyModule for MhyContext<RSAEncrypt> {
    unsafe fn init(&mut self) -> Result<()> {
        if let Some(addr) = self.addr {
            self.interceptor.replace(
                addr as usize,
                hkaddr,
            )
        } else {
            Err(anyhow::anyhow!("Cannot find address."))
        }
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::RSAEncrypt
    }
}

pub unsafe extern "win64" fn hkaddr(
    reg: *mut Registers, _: usize, _:usize
) -> usize {
    let content_ptr = (*reg).rdx;
    content_ptr as usize
}