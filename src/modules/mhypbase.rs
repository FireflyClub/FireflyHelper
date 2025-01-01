use crate::modules::{MhyContext, MhyModule, ModuleType};
use anyhow::Result;
use ilhook::x64::Registers;

pub struct Mhypbase;

impl MhyModule for MhyContext<Mhypbase> {
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

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Mhypbase
    }
}

unsafe extern "win64" fn hkaddr(_reg: *mut Registers, _: usize, _:usize) ->usize{
    
    0
}