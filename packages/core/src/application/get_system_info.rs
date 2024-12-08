use crate::domain::system_info::{DiskInfo, MemoryInfo, ProcessorInfo, StorageInfo, SystemInfo};
use eyre::Result;
use sysinfo::{Disks, System};

pub fn get_system_info() -> Result<SystemInfo> {
    let mut system = System::new_all();
    system.refresh_all();

    let processor = get_processor_info(&system)?;
    let memory = get_memory_info(&system);
    let storage = get_storage_info()?;

    Ok(SystemInfo {
        processor,
        memory,
        storage,
    })
}

fn get_processor_info(system: &System) -> Result<ProcessorInfo> {
    let cpu = system
        .cpus()
        .first()
        .ok_or_else(|| eyre::eyre!("No CPU found"))?;

    Ok(ProcessorInfo {
        name: if cpu.brand().is_empty() {
            "Unknown CPU".to_string()
        } else {
            cpu.brand().to_string()
        },
        cores: system.physical_core_count().unwrap_or(1) as u32,
        frequency_ghz: cpu.frequency() as f64 / 1000.0,
        architecture: std::env::consts::ARCH.to_string(),
    })
}

fn get_memory_info(system: &System) -> MemoryInfo {
    MemoryInfo {
        total_bytes: system.total_memory(),
    }
}

fn get_storage_info() -> Result<StorageInfo> {
    let disks = Disks::new_with_refreshed_list();
    const MIN_DISK_SIZE: u64 = 10 * 1024 * 1024 * 1024; // 10 GiB

    let mut seen_devices = std::collections::HashSet::new();
    let disk_infos: Vec<DiskInfo> = disks
        .list()
        .iter()
        .filter_map(|disk| {
            // Skip virtual and special filesystems
            let fs_type = disk.file_system().to_str()?;
            if [
                "devpts",
                "tmpfs",
                "devtmpfs",
                "squashfs",
                "overlay",
                "hugetlbfs",
                "mqueue",
            ]
            .contains(&fs_type)
            {
                return None;
            }

            // Skip small filesystems and those with 0 total space
            if disk.total_space() < MIN_DISK_SIZE || disk.total_space() == 0 {
                return None;
            }

            // On macOS, avoid duplicate APFS volumes
            let device_name = disk.name().to_str()?;
            if !seen_devices.insert(device_name.to_string()) {
                return None;
            }

            Some(DiskInfo {
                name: device_name.to_string(),
                mount_point: disk.mount_point().to_str()?.to_string(),
                total_bytes: disk.total_space(),
                available_bytes: disk.available_space(),
                disk_type: fs_type.to_string(),
            })
        })
        .collect();

    if disk_infos.is_empty() {
        return Err(eyre::eyre!("No valid disks found"));
    }

    Ok(StorageInfo { disks: disk_infos })
}
