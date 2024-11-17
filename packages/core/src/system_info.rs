use eyre::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub processor: String,
    pub memory: String,
    pub storage: String,
}

pub fn get_system_info() -> Result<SystemInfo> {
    let mut system = System::new_all();
    system.refresh_all();

    // Get processor details
    let cpu_name = system
        .cpus()
        .first()
        .map(|cpu| {
            if cpu.brand().is_empty() {
                "Unknown CPU name".to_string()
            } else {
                cpu.brand().to_string()
            }
        })
        .ok_or_else(|| eyre::eyre!("Failed to retrieve CPU name"))?;

    let cpu_cores = system.physical_core_count().unwrap_or(1);

    let cpu_frequency = system
        .cpus()
        .first()
        .map(|cpu| cpu.frequency() as f64 / 1000.0)
        .ok_or_else(|| eyre::eyre!("Failed to retrieve CPU frequency"))?;

    let cpu_architecture = if std::env::consts::ARCH.is_empty() {
        "Unknown architecture".to_string()
    } else {
        std::env::consts::ARCH.to_string()
    };

    let processor = format!(
        "{} ({} cores, {:.2} GHz, {})",
        cpu_name, cpu_cores, cpu_frequency, cpu_architecture
    );

    // Fetch total RAM in bytes and convert to gigabytes
    let total_ram = system.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);

    let memory = format!("{:.2} GB RAM", total_ram);

    // Get total and available disk space and convert to gigabytes
    let disks = Disks::new_with_refreshed_list();

    let total_disk_space = disks
        .iter()
        .map(|d| d.total_space() as f64 / 1_024_000_000.0)
        .sum::<f64>();

    let available_disk_space = disks
        .iter()
        .map(|d| d.available_space() as f64 / 1_024_000_000.0)
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
