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
    // TODO
    STRING_ADDR = Some((base_ga + 0x0D04CAB0) as *mut u8);

    // Enable InternalSetUrl
    // 56 57 48 83 EC 28 48 89 D6 48 89 CF 80 3D ? ? ? ? ? 74 ? 48 89 F9 FF 15 ? ? ? ? 84 C0 74 ? 48 89 F9 48 89 F2 FF 15 ? ? ? ? 85 C0 75 ? 48 83 C4 28 5F 5E C3 B9 12
    module_manager.enable(MhyContext::<SetUrl>::new(Some((base_ga + 0x04E64B70) as *mut u8)));

    // Enable MakeInitialUrl
    // 55 41 56 56 57 53 48 83 EC 70 48 8D 6C 24 ? 48 C7 45 ? ? ? ? ? 48 89 D6 48 89 CF 80 3D ? ? ? ? ? 74
    module_manager.enable(MhyContext::<SetUrl>::new(Some((base_ga + 0x04E64810) as *mut u8)));
    println!("SetUrl enabled!");

    // Enable GameCoreConfigLoader
    // module_manager.enable(MhyContext::<RES>::new(Some((base_ga + 0x0C742290) as *mut u8)));
    // println!("GameCoreConfigLoader enabled!");

    // Disable RSAEncrypt
    // 41 57 41 56 41 55 41 54 56 57 55 53 48 83 EC 78 49 89 D4 48 89 CE
    module_manager.enable(MhyContext::<RSAEncrypt>::new(Some((base_ga + 0x0C9A7FA0) as *mut u8)));
    module_manager.enable(MhyContext::<RSAEncrypt>::new(Some((base_ga + 0x0D0FDE20) as *mut u8)));
    println!("RSAEncrypt disabled!");

    // Disable SetElevationDitherAlphaValue | SetDistanceDitherAlphaValue
    // 56 48 83 EC 30 0F 29 74 24 ? 0F 28 F1 48 89 CE 80 3D ? ? ? ? ? 75 ? 80 7E ? ? 74 ? 0F 57
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x067242A0) as *mut u8)));
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x067245A0) as *mut u8)));

    // Disable SetDitherAlphaValue
    // 56 57 48 83 EC 38 0F 29 74 24 ? 44 89 C6 0F 28 F1 48 89 CF 80 3D ? ? ? ? ? 75 ? 80
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x0671A480) as *mut u8)));

    // Disable SetDitherAlphaValueWithAnimation
    // 56 57 55 53 48 83 EC 78 44 0F 29 44 24 ? 0F 29 7C 24 ? 0F 29 74 24 ? 44 0F 28 C3
    module_manager.enable(MhyContext::<Censorship>::new(Some((base_ga + 0x06719380) as *mut u8)));
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
