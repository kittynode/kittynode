use eyre::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub processor: String,
    pub memory: String,  // We'll show total RAM in GB
    pub storage: String, // Show total and available disk space
}

pub fn get_system_info() -> Result<SystemInfo> {
    let mut system = System::new_all();
    system.refresh_all();

    // Get processor details
    let cpu_name = system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let cpu_cores = system.physical_core_count().unwrap_or(1);
    let cpu_frequency = system
        .cpus()
        .first()
        .map(|cpu| cpu.frequency() as f64 / 1000.0) // Convert MHz to GHz
        .unwrap_or(0.0);
    let cpu_architecture = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "ARM"
    } else {
        "Unknown"
    };

    let processor = format!(
        "{} ({} cores, {:.2} GHz, {})",
        cpu_name, cpu_cores, cpu_frequency, cpu_architecture
    );

    // Fetch total RAM in bytes and convert to gigabytes
    let total_ram = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0); // Convert from bytes to GB
    let memory = format!("{:.2} GB RAM", total_ram);

    // Get total and available disk space
    let total_disk_space = system
        .disks()
        .iter()
        .map(|d| d.total_space() as f64 / 1_024_000_000.0) // Convert to GB
        .sum::<f64>();

    let available_disk_space = system
        .disks()
        .iter()
        .map(|d| d.available_space() as f64 / 1_024_000_000.0) // Convert to GB
        .sum::<f64>();

    let storage = format!(
        "{:.2} GB available / {:.2} GB total",
        available_disk_space, total_disk_space
    );

    Ok(SystemInfo {
        processor,
        memory,
        storage,
    })
}
