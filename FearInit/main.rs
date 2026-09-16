/*
* Copyright (C) 2026 Denis Rudolf Kontek (Secure Your Soul)
* PROPRIETARY AND CONFIDENTIAL / SOURCE-AVAILABLE ONLY.
* See local LICENSE file for usage restrictions.
*/
use nix::mount::{MsFlags, mount};
use nix::sys::wait::{WaitPidFlag, waitpid};
use secure_api::{server_status, verify_programs};
use std::ffi::OsStr;
use nix::unistd::{fork, ForkResult, execvp};
use std::ffi::CString;
use std::process;
use signal_hook::iterator::Signals;

fn main() {
    // 1. Montowanie systemów plików
    mount_fs("proc", "/proc", "proc");
    mount_fs("sysfs", "/sys", "sysfs");
    mount_fs("devtmpfs", "/dev", "devtmpfs");
    
    server_status();
    verify_programs();

    // 2. Start usług
    if fork_and_exec("/sbin/wayland-compositor") {
        println!("Kompozytor wystartował pomyślnie.");
    } else {
        eprintln!("Błąd uruchomienia procesów tła.");
    }

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
    mount(
        Some(source),
        target,
        Some(fstype),
        MsFlags::MS_NOSUID | MsFlags::MS_NODEV | MsFlags::MS_NOEXEC,
        None::<&OsStr>,
    )
    .expect("Nie udało się zamontować systemu plików");
}

fn fork_and_exec(path: &str) -> bool {
    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            let c_path = match CString::new(path) {
                Ok(p) => p,
                Err(_) => process::exit(1),
            };
            let args = [c_path.clone()];
            match execvp(&c_path, &args) {
                Ok(_) => unreachable!(),  // exec nie wraca przy sukcesie
                Err(e) => {
                    eprintln!("execvp({}) failed: {}", path, e);
                    process::exit(1);
                }
            }
            process::exit(1);
        }
        Ok(ForkResult::Parent { child }) => {
            println!("Uruchomiono proces w tle o PID: {}", child);
            true
        }
        Err(e) => {
            eprintln!("Błąd podczas fork(): {}", e);
            false
        }
    }
}