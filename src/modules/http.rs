use std::ffi::CString;
use anyhow::Result;
use ilhook::x64::Registers;

use crate::modules::{MhyContext, MhyModule, ModuleType};
use crate::marshal;

pub struct Http;

impl MhyModule for MhyContext<Http> {
    unsafe fn init(&mut self) -> Result<()> {
        if let Some(addr) = self.addr {
            self.interceptor.attach(
                addr as usize,
                hkaddr,
            )
        } else {
            Err(anyhow::anyhow!("Cannot find Mhypbase address."))
        }
    }

    unsafe fn de_init(&mut self) -> Result<()> {
        Ok(())
    }

    fn get_module_type(&self) -> super::ModuleType {
        ModuleType::Http
    }
}

unsafe extern "win64" fn hkaddr(reg: *mut Registers, _: usize) {
    let url = marshal::read_csharp_string((*reg).rdx);
    
    if url.to_lowercase().contains("watermark") {
        return;
    }

    let mut new_url = String::from("http://127.0.0.1:619");
    url.split('/').skip(3).for_each(|s| {
        new_url.push_str("/");
        new_url.push_str(s);
    });

    println!("\"{url}\" => {new_url}");

    (*reg).rdx =
        marshal::ptr_to_string_ansi(CString::new(new_url.as_str()).unwrap().as_c_str()) as u64;
}
