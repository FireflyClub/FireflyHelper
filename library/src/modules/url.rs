use std::ffi::CString;
use anyhow::Result;
use ilhook::x64::Registers;

use crate::URL;
use crate::modules::{MhyContext, MhyModule, ModuleType};
use crate::marshal::{ptr_to_ansi, read_il2cpp_str};

pub struct SetUrl;
impl MhyModule for MhyContext<SetUrl> {
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
        ModuleType::SetUrl
    }
}

unsafe extern "win64" fn hkaddr(reg: *mut Registers, _: usize) {
    let url = read_il2cpp_str((*reg).rcx);

    let mut new_url = match url.as_str() {
        s if ((s.contains("mihoyo.com")
            || s.contains("hoyoverse.com")
            || s.contains("starrails.com")
            || s.contains(".bhsr.com"))
            && !(s.contains("autopatchcn") || s.contains("autopatchos"))) =>
        {
            URL.to_string()
        }
        s => {
            println!("Leaving request as-is: {s}");
            return;
        }
    };

    url.split('/').skip(3).for_each(|s| {
        new_url.push_str("/");
        new_url.push_str(s);
    });

    println!("\"{url}\" => {new_url}");
    (*reg).rcx =
        ptr_to_ansi(CString::new(new_url.as_str()).unwrap().as_c_str()) as u64;
}
