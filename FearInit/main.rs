use nix::mount::{MsFlags, mount};
use nix::sys::wait::{WaitPidFlag, waitpid};
use secure_api::{server_status, verify_programs};
use std::ffi::OsStr;
fn main() {
    server_status();
    // 1. MONTAŻ (Fundament)
    // Bez tego nic nie działa: proc (dla ps/top), sys (dla sprzętu), dev (dla urządzeń)
    mount_fs("proc", "/proc", "proc");
    mount_fs("sysfs", "/sys", "sysfs");
    mount_fs("devtmpfs", "/dev", "devtmpfs");

    // 2. BEZPIECZEŃSTWO (Hardening)
    // Tu wstawiasz swoje Landlock/Seccomp
    verify_programs();

    // 3. START USŁUG (Delegowanie)
    // Nie robisz tu wszystkiego, tylko odpalasz menedżer usług lub kompozytor
    if fork_and_exec("/usr/bin/wayland-compositor") {
        println!("Kompozytor wystartował.");
    }

    // 4. PĘTLA GŁÓWNA (Zombiaki i Sygnały)
    // To jest jedyna rzecz, która trzyma system przy życiu
    loop {
        // Zbieranie zombie: PID 1 MUSI to robić, inaczej kernel się zapcha
        match waitpid(None, Some(WaitPidFlag::WNOHANG)) {
            Ok(pid) => println!("Proces {:?} zakończył się.", pid),
            Err(_) => std::thread::sleep(std::time::Duration::from_secs(1)),
        }

        // Tutaj obsługa sygnałów: jeśli dostaniesz SIGTERM,
        // musisz zamknąć desktop i wyłączyć system (reboot/poweroff)
    }
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

fn fork_and_exec(_path: &str) -> bool {
    // Tutaj używasz nix::unistd::fork() i exec()
    // To uruchamia Twój desktop w tle
    true
}
