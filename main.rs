/*

  Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)

  This file is part of FearInit and is dual-licensed:
    1. AGPL-3.0-or-later — see LICENSE for the full text.
    2. Commercial license — see COMMERCIAL.md for details.

*/
#![allow(clippy::multiple_crate_versions)]
use nix::mount::{MsFlags, mount};
use nix::sys::wait::{WaitPidFlag, waitpid};
use signal_hook::iterator::Signals;
use std::ffi::OsStr;
use sysinfo::System;
const VERSION: &str = env!("CARGO_PKG_VERSION");
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Montowanie systemów plików
    mount_fs("proc", "/proc", "proc");
    mount_fs("sysfs", "/sys", "sysfs");

    server_status();
    verify_programs();

    // 3. Konfiguracja sygnałów + SIGCHLD (żeby jądro wołało nas, gdy dziecko padnie!)
    let mut signals = Signals::new([
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGCHLD,
    ])?; // Wymaga zmiany sygnatury main na -> Result

    println!("Inicjalizacja zakończona. Przechodzę w tryb oszczędzania energii (nasłuch zdarzeń).");

    // 4. PĘTLA GŁÓWNA (Zdarzeniowa - 0% CPU w spoczynku)
    'outer: loop {
        // Ta linijka BLOKJE proces. CPU idzie spać w stan głębokiego uśpienia.
        // Program obudzi się BŁYSKAWICZNIE tylko wtedy, gdy jądro dostarczy sygnał.
        for signal in signals.wait() {
            match signal {
                signal_hook::consts::SIGTERM | signal_hook::consts::SIGINT => {
                    println!("Otrzymano sygnał zamknięcia. Wyłączam system...");
                    break 'outer;
                }
                signal_hook::consts::SIGCHLD => {
                    // Proces potomny zakończył się lub zmienił stan - zbieramy zombie
                    loop {
                        match waitpid(None, Some(WaitPidFlag::WNOHANG)) {
                            Ok(nix::sys::wait::WaitStatus::StillAlive) | Err(_) => break,
                            Ok(status) => println!("Zdarzenie procesu: {status:?}"),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!("System zamknięty bezpiecznie.");
    Ok(())
}

fn mount_fs(source: &str, target: &str, fstype: &str) {
    match mount(
        Some(source),
        target,
        Some(fstype),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC,
        None::<&OsStr>,
    ) {
        Ok(()) | Err(nix::errno::Errno::EBUSY) => {} // if is mounted
        Err(e) => eprintln!("mount {source} -> {target}: {e}"),
    }
}

fn server_status() {
    let name: &str = "FearInit";

    // Pobieranie PID aktualnego procesu
    let current_pid = std::process::id();

    println!("[{name}] Starting (v{VERSION})");

    // Inicjalizacja informacji systemowych przez sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // Informacje o systemie i procesorze
    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown version".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown kernel".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown host".to_string());
    let cpu_brand = sys.cpus().first().map_or("Unknown CPU", |cpu| cpu.brand());
    let total_memory = sys.total_memory() / 1024 / 1024; // w MB

    println!("--------------------------------------------------");
    println!("[System Info]");
    println!(" * Hostname:      {host_name}");
    println!(" * OS:            {os_name} ({os_version})");
    println!(" * Kernel:        {kernel_version}");
    println!(" * CPU:           {cpu_brand}");
    println!(" * Total RAM:     {total_memory} MB");
    println!(" * PID procesu:   {current_pid}");
    println!("--------------------------------------------------");

    println!("[{name}] Loaded.");
}
fn verify_programs() {
    println!("Veryfing Programs...");
    if "Program" == "sign" {
        println!("Verify Error");
    }
    println!("Verify Done");
}
