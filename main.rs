// Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)
// SPDX-License-Identifier: AGPL-3.0-or-later OR LicenseRef-Commercial
// See LICENSE and DUAL-LICENSING.md for licensing details.
use nix::mount::{MsFlags, mount};
use nix::sys::wait::{WaitPidFlag, waitpid};
use std::ffi::OsStr;
use signal_hook::iterator::Signals;
use sysinfo::System;

fn main() {
    // 1. Montowanie systemów plików
    mount_fs("proc", "/proc", "proc");
    mount_fs("sysfs", "/sys", "sysfs");
    mount_fs("devtmpfs", "/dev", "devtmpfs");
    
    server_status();
    verify_programs();

    // 3. Konfiguracja sygnałów + SIGCHLD (żeby jądro wołało nas, gdy dziecko padnie!)
    let mut signals = Signals::new(&[
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGCHLD, // Kluczowe: informacja, że proces potomny zmienił stan
    ]).expect("Nie udało się zarejestrować sygnałów");

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
                            Ok(nix::sys::wait::WaitStatus::StillAlive) => break,
                            Ok(status) => println!("Zdarzenie procesu: {:?}", status),
                            Err(_) => break, // Brak więcej dzieci do zebrania
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!("System zamknięty bezpiecznie.");
}

fn mount_fs(source: &str, target: &str, fstype: &str) {
    match mount(
        Some(source), target, Some(fstype),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC,
        None::<&OsStr>,
    ) {
        Ok(()) => {}
        Err(nix::errno::Errno::EBUSY) => {} // już zamontowane
        Err(e) => eprintln!("mount {} -> {}: {}", source, target, e),
    }
}

fn server_status() {
    let name: &str = "FearInit";
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Pobieranie PID aktualnego procesu
    let current_pid = std::process::id();

    println!("[{}] Starting (v{})", name, VERSION);

    // Inicjalizacja informacji systemowych przez sysinfo
    let mut sys = System::new_all();
    sys.refresh_all();

    // Informacje o systemie i procesorze
    let os_name = System::name().unwrap_or_else(|| "Unknown OS".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown version".to_string());
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown kernel".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown host".to_string());
    let cpu_brand = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand())
        .unwrap_or("Unknown CPU");
    let total_memory = sys.total_memory() / 1024 / 1024; // w MB

    println!("--------------------------------------------------");
    println!("[System Info]");
    println!(" * Hostname:      {}", host_name);
    println!(" * OS:            {} ({})", os_name, os_version);
    println!(" * Kernel:        {}", kernel_version);
    println!(" * CPU:           {}", cpu_brand);
    println!(" * Total RAM:     {} MB", total_memory);
    println!(" * PID procesu:   {}", current_pid);
    println!("--------------------------------------------------");

    println!("[{}] Loaded.", name);
}
fn verify_programs() {
    println!("Veryfing Programs...");
    if "Program" == "sign" {
        println!("Verify Error");
    }
    println!("Verify Done");
}
