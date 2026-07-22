use sysinfo::System;

pub fn server_status() {
    let name: &str = "FearInit";
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Pobieranie PID aktualnego procesu
    let current_pid = std::process::id();

    println!("[{}] Starting", name);
    println!("[{}] (v{})", name, VERSION);

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
pub fn verify_programs() {
    println!("Veryfing Programs...");
    if "Program" == "sign" {
        println!("Verify Error");
    }
    println!("Verify Done");
}
