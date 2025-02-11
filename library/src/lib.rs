#![feature(str_from_utf16_endian)]
#![feature(let_chains)]
#![allow(non_snake_case)]

use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[allow(unused_imports)]
use crate::modules::{MhyContext, ModuleManager, SetUrl, RSAEncrypt, Censorship, RES};
use crate::marshal::STRING_ADDR;

mod util;
mod marshal;
mod interceptor;
mod modules;

pub const URL: &str = "http://127.0.0.1:619";

unsafe fn thread_func() {
    Console::AllocConsole().unwrap_or(());
    println!("FireflyHelper By Lethe");

    // Get base address
    let base_ga = loop {
        if let Some(base_ga) = util::try_get_base_address("GameAssembly.dll") {
            break base_ga;
        }
        thread::sleep(Duration::from_millis(500));
    };
    println!("GameAssembly: {base_ga}");

    // Init module manager
    let mut module_manager = MODULE_MANAGER.write().unwrap();

    // PtrToStringAnsi
    STRING_ADDR = Some((base_ga + 0x0D13CF40) as *mut u8);

    // Enable InternalSetUrl
    module_manager.enable(MhyContext::<SetUrl>::new(Some((base_ga + 0x06404EB0) as *mut u8)));

    // Enable MakeInitialUrl
    module_manager.enable(MhyContext::<SetUrl>::new(Some((base_ga + 0x06404B80) as *mut u8)));
    println!("SetUrl enabled!");

    // Enable GameCoreConfigLoader
    // module_manager.enable(MhyContext::<RES>::new(Some((base_ga + 0x0C742290) as *mut u8)));
    // println!("GameCoreConfigLoader enabled!");

    // Disable RSAEncrypt
    module_manager.enable(MhyContext::<RSAEncrypt>::new(Some((base_ga + 0x0D203B30) as *mut u8)));
    module_manager.enable(MhyContext::<RSAEncrypt>::new(Some((base_ga + 0x0CC903B0) as *mut u8)));
    println!("RSAEncrypt disabled!");

    // Disable SetElevationDitherAlphaValue | SetDistanceDitherAlphaValue
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x073C4F00) as *mut u8)));
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x073C51E0) as *mut u8)));

    // Disable SetDitherAlphaValue
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x073BB6D0) as *mut u8)));

    // Disable SetDitherAlphaValueWithAnimation
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x073BA620) as *mut u8)));
    println!("Censorship disabled!");

    println!("Successfully injected!");
}

lazy_static! {
    static ref MODULE_MANAGER: RwLock<ModuleManager> = RwLock::new(ModuleManager::default());
}

#[no_mangle]
unsafe extern "system" fn DllMain(_: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    if call_reason == DLL_PROCESS_ATTACH {
        thread::spawn(|| thread_func());
    }

    true
}
