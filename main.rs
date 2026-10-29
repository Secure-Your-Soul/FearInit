/*

  Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)

  This file is part of FearInit and is dual-licensed:
    1. AGPL-3.0-or-later — see LICENSE for the full text.
    2. Commercial license — see COMMERCIAL.md for details.

*/
#![allow(clippy::multiple_crate_versions)]
use nix::fcntl::{OFlag, open};
use nix::mount::{MsFlags, mount};
use nix::sys::stat::{Mode, SFlag, makedev, mknod};
use nix::sys::wait::{WaitPidFlag, waitpid};
use nix::unistd::{close, dup2};
use signal_hook::iterator::Signals;
use std::ffi::OsStr;
use sysinfo::System;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Kernel mount /dev (devtmpfs) with MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC
    // Creating /dev/console
    let _ = mknod(
        "/dev/console",
        SFlag::S_IFCHR,
        Mode::from_bits_truncate(0o600),
        makedev(5, 1),
    );

    // Connect console to stdin (0), stdout (1), stderr (2)
    if let Ok(fd) = open("/dev/console", OFlag::O_RDWR, Mode::empty()) {
        let _ = dup2(fd, 0);
        let _ = dup2(fd, 1);
        let _ = dup2(fd, 2);
        let _ = close(fd);
    }

    // Mount /proc and /sys with MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC
    mount_fs("proc", "/proc", "proc");
    mount_fs("sysfs", "/sys", "sysfs");

    server_status();
    verify_programs();

    // Signals Configuration + SIGCHLD (żeby jądro wołało nas, gdy dziecko padnie!)
    let mut signals = Signals::new([
        signal_hook::consts::SIGTERM,
        signal_hook::consts::SIGINT,
        signal_hook::consts::SIGCHLD,
    ])?;

    // Main loop (0% CPU in rest)
    'outer: loop {
        // Ta linijka BLOKUJE proces. CPU idzie spać w stan głębokiego uśpienia, a program obudzi się tylko wtedy, gdy jądro dostarczy sygnał.
        for signal in signals.wait() {
            match signal {
                signal_hook::consts::SIGTERM | signal_hook::consts::SIGINT => {
                    println!("Otrzymano sygnał zamknięcia. Wyłączam system...");
                    break 'outer;
                }
                signal_hook::consts::SIGCHLD => {
                    // Taking zombie
                    loop {
                        match waitpid(None, Some(WaitPidFlag::WNOHANG)) {
                            Ok(nix::sys::wait::WaitStatus::StillAlive) | Err(_) => break,
                            Ok(status) => println!("Process Event: {status:?}"),
                        }
                    }
                }
                _ => {}
            }
        }
    }

    println!("System closed safety.");
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

    let current_pid = std::process::id();

    println!("[{name}] Starting (v{VERSION})");

    let mut sys = System::new_all();
    sys.refresh_all();

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
