use std::collections::HashMap;
use anyhow::Result;

use crate::interceptor::Interceptor;

mod mhypbase;
pub use mhypbase::Mhypbase;
mod sdkutil;
pub use sdkutil::SdkUtil;

#[derive(Default)]
pub struct ModuleManager {
    modules: HashMap<ModuleType, Box<dyn MhyModule>>,
}
unsafe impl Sync for ModuleManager {}
unsafe impl Send for ModuleManager {}

impl ModuleManager {
    pub unsafe fn enable(&mut self, module: impl MhyModule + 'static) {
        let mut boxed_module = Box::new(module);
        boxed_module.init().unwrap();
        self.modules
            .insert(boxed_module.get_module_type(), boxed_module);
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum ModuleType {
    Mhypbase, SdkUtil
}

pub trait MhyModule {
    unsafe fn init(&mut self) -> Result<()>;
    #[allow(dead_code)] unsafe fn de_init(&mut self) -> Result<()>;
    fn get_module_type(&self) -> ModuleType;
}

pub struct MhyContext<T> {
    pub addr: Option<*mut u8>,
    pub interceptor: Interceptor,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> MhyContext<T> {
    pub const fn new(addr: Option<*mut u8>) -> Self {
        Self {
            addr,
            interceptor: Interceptor::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}
