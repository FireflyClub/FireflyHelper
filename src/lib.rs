#![feature(str_from_utf16_endian)]
#![feature(let_chains)]
#![allow(non_snake_case)]

mod util;
mod interceptor;
mod modules;

use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use lazy_static::lazy_static;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
use winapi::um::processthreadsapi::{GetCurrentThread, TerminateThread};
use crate::modules::{MhyContext, ModuleManager, Mhypbase, SdkUtil};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
extern "cdecl" fn Initialize() -> bool {
    thread::sleep(Duration::from_secs(2));
    let thread = unsafe { GetCurrentThread() };
    unsafe { TerminateThread(thread, 0) };
    false
}

unsafe fn thread_func() {
    // Get base address
    let base_ga = loop {
        if let Some(base_ga) = util::try_get_base_address("GameAssembly.dll") {
            break base_ga;
        }
        thread::sleep(Duration::from_millis(500));
    };
    let base_up = loop {
        if let Some(base_up) = util::try_get_base_address("UnityPlayer.dll") {
            break base_up;
        }
        thread::sleep(Duration::from_millis(500));
    };

    Console::AllocConsole().unwrap_or(());

    println!("FireflyRP By Lethe");
    util::detect_version();
    println!("GameAssembly: {base_ga} | UnityPlayer: {base_up}");

    // Init module manager
    let mut module_manager = MODULE_MANAGER.write().unwrap();

    // Disable MhyPBase
    println!("Disabling Mhypbase...");
    let anti_cheat_init_addr = util::pattern_scan(base_up, "55 41 56 56 57 53 48 81 EC 00 01 00 00 48 8D AC 24 80 00 00 00 C7 45 7C 00 00 00 00", 0);
    module_manager.enable(MhyContext::<Mhypbase>::new(anti_cheat_init_addr));
    println!("AntiCheatInit: {:?}", anti_cheat_init_addr);
    let black_list_dlls_addr = util::pattern_scan(base_up, "55 41 57 41 56 41 55 41 54 56 57 53 48 81 EC B8 02 00 00", 0);
    module_manager.enable(MhyContext::<Mhypbase>::new(black_list_dlls_addr));
    println!("Blacklistdlls: {:?}", black_list_dlls_addr);

    // Disable RSAEncryption
    println!("Disabling RSAEncryption...");
    // TODO: Devide pattern num by version
    let mihoyo_sdk_util_addr = util::pattern_scan(base_ga, "48 89 ? ? ? 48 89 ? ? ? 57 48 81 ? ? ? ? ? 80 ? ? ? ? ? ? 48 8B EA 48 8B F1 75 ? B9 ? ? ? ?", 0);
    module_manager.enable(MhyContext::<SdkUtil>::new(mihoyo_sdk_util_addr));
    println!("MihoyoSdkUtil: {:?}", mihoyo_sdk_util_addr);

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
