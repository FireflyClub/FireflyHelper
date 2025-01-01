#![feature(str_from_utf16_endian)]
#![feature(let_chains)]
#![allow(non_snake_case)]

mod util;
mod marshal;
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
use crate::modules::{MhyContext, ModuleManager, Http, Mhypbase, SdkUtil, DisableCensorship};

pub static mut STRING_ADDR: Option<*mut u8> = None;

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

    println!("FireflyHelper By Lethe");
    util::detect_version();
    println!("GameAssembly: {base_ga} | UnityPlayer: {base_up}");

    // Init module manager
    let mut module_manager = MODULE_MANAGER.write().unwrap();

    // Init pattern scanner
    // 48 C7 C2 ? ? ? ? 48 FF C2 80 3C 11 ? 75 ? E9 ? ? ? ?
    STRING_ADDR = Some((base_ga + 0x00BCA4D0) as *mut u8);

    // Enable WebReqInitUrl
    println!("Enabling WebReqInitUrl...");
    // 48 89 4C 24 ?? 53 56 57 41 56 41 57 48 83 EC ?? 48 C7 44 24 ?? ?? ?? ?? ?? 48 8B FA 48 8B D9 80 3D ?? ?? ?? ?? ?? 75 ?? B9 ?? ?? ?? ?? E8 ?? ?? ?? ?? C6 05 ?? ?? ?? ?? ?? 48 8B CB
    module_manager.enable(MhyContext::<Http>::new(Some((base_ga + 0x08445EA0) as *mut u8)));
    println!("WebReqInitUrl enabled!");

    // Disable Mhypbase
    println!("Disabling Mhypbase...");
    let anti_cheat_init_addr = util::pattern_scan(base_up, "55 41 56 56 57 53 48 81 EC 00 01 00 00 48 8D AC 24 80 00 00 00 C7 45 7C 00 00 00 00", 0);
    module_manager.enable(MhyContext::<Mhypbase>::new(anti_cheat_init_addr));
    println!("AntiCheatInit disabled!");
    let black_list_dlls_addr = util::pattern_scan(base_up, "55 41 57 41 56 41 55 41 54 56 57 53 48 81 EC B8 02 00 00", 0);
    module_manager.enable(MhyContext::<Mhypbase>::new(black_list_dlls_addr));
    println!("Blacklistdlls disabled!");

    // Disable MhySdkUtil
    println!("Disabling MhySdkUtil...");
    // 48 89 6C 24 ? 48 89 74 24 ? 57 48 81 EC ? ? ? ? 80 3D ? ? ? ? ? 48 8B EA 48 8B F1 75 11 B9 ? ? ? ? E8 ? ? ? ? C6 05 ? ? ? ? ? 48 8B 0D ? ? ? ? 48 89 9C 24 ? ? ? ? E8 ? ? ? ? 48 8B C8 48 8B F8 E8 ? ? ? ? 48 85 FF 0F 84 ? ? ? ? 80 3D ? ? ? ? ? 75 11
    module_manager.enable(MhyContext::<SdkUtil>::new(Some((base_ga + 0x0732F210) as *mut u8)));
    module_manager.enable(MhyContext::<SdkUtil>::new(Some((base_ga + 0x07927D50) as *mut u8)));
    println!("MhySdkUtil disabled!");

    // Disable DisableCensorship
    println!("Disabling DisableCensorship...");
    // 40 53 48 83 EC 30 80 3D ? ? ? ? ? 48 8B D9 0F 29 74 24 ? 0F 28 F1 75 44 80 79 33 00 74 33 F3 0F 10 0D ? ? ? ? 0F 57 C0 F3 0F 5F C6 F3 0F 5D C8 F3 0F 11 49 ? 41 B8 ? ? ? ? F3 0F 59 49 ? E8 ? ? ? ? 84 C0 75 07 C7 43 ? ? ? ? ? 0F 28 74 24
    module_manager.enable(MhyContext::<DisableCensorship>::new(Some((base_ga + 0x01189D00) as *mut u8)));
    module_manager.enable(MhyContext::<DisableCensorship>::new(Some((base_ga + 0x0118A050) as *mut u8)));

    // 48 89 5C 24 ? 57 48 83 EC 30 80 3D ? ? ? ? ? 41 8B F8 0F 29 74 24 ? 48 8B D9 0F 28 F1 75 2E 80 79 33 00 74 18 F3 0F 10 0D
    module_manager.enable(MhyContext::<DisableCensorship>::new(Some((base_ga + 0x01189FC0) as *mut u8)));

    // 48 89 5C 24 ? 57 48 83 EC 70 80 3D ? ? ? ? ? 48 8B D9 0F 29 74 24 ? 0F 28 F2 0F 29 7C 24
    module_manager.enable(MhyContext::<DisableCensorship>::new(Some((base_ga + 0x01189D90) as *mut u8)));
    println!("DisableCensorship disabled!");

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
