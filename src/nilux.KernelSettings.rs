
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub struct KernelSettings {
    sysctl_conf: String,
    grub_config: String,
}

impl KernelSettings {
    pub fn new() -> Self {
        KernelSettings {
            sysctl_conf: String::from("/etc/sysctl.conf"),
            grub_config: String::from("/etc/default/grub"),
        }
    }

    pub fn apply_optimizations(&self) -> io::Result<()> {
        self.optimize_sysctl()?;
        self.optimize_grub()?;
        Ok(())
    }

    fn optimize_sysctl(&self) -> io::Result<()> {
        let mut file = File::create(&self.sysctl_conf)?;
        writeln!(file, "# Nilux Kernel Optimizations")?;
        writeln!(file, "vm.swappiness = 10")?;
        writeln!(file, "vm.vfs_cache_pressure = 50")?;
        writeln!(file, "net.ipv4.tcp_fastopen = 3")?;
        writeln!(file, "net.core.default_qdisc = fq")?;
        writeln!(file, "net.ipv4.tcp_congestion_control = bbr")?;
        Ok(())
    }

    fn optimize_grub(&self) -> io::Result<()> {
        let content = fs::read_to_string(&self.grub_config)?;
        let mut lines: Vec<String> = content.lines().map(String::from).collect();

        for line in &mut lines {
            if line.starts_with("GRUB_CMDLINE_LINUX_DEFAULT") {
                *line = r#"GRUB_CMDLINE_LINUX_DEFAULT="quiet splash intel_pstate=enable mitigations=off""#.to_string();
            }
        }

        let mut file = File::create(&self.grub_config)?;
        for line in lines {
            writeln!(file, "{}", line)?;
        }

        // Update GRUB
        std::process::Command::new("update-grub")
            .status()
            .expect("Failed to update GRUB");

        Ok(())
    }
}

pub fn setup_kernel_settings() -> io::Result<()> {
    let settings = KernelSettings::new();
    settings.apply_optimizations()?;
    println!("Kernel settings optimized for Nilux OS.");
    Ok(())
}
