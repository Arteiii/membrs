use std::fs::{self, File};
use std::io::{self, Write};
#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
use std::os::windows::fs::symlink_file as symlink; // For Windows

use std::path::Path;

// Define the NginxConf struct
pub struct NginxConf {
    domain: String,
    frontend_port: u16,
    backend_port: u16,
    sites_available: String,
    sites_enabled: String,
}

impl NginxConf {
    pub fn new(domain: &str, frontend_port: u16, backend_port: u16) -> Self {
        Self {
            domain: domain.to_string(),
            frontend_port,
            backend_port,
            sites_available: "/etc/nginx/sites-available".to_string(),
            sites_enabled: "/etc/nginx/sites-enabled".to_string(),
        }
    }

    pub fn generate_config(&self) -> String {
        let template = include_str!("nginx_template.conf");
        template
            .replace("{domain}", &self.domain)
            .replace("{frontend_port}", &self.frontend_port.to_string())
            .replace("{backend_port}", &self.backend_port.to_string())
    }

    // Method to save the configuration to a file
    pub fn save_config(&self) -> io::Result<()> {
        let nginx_conf_file = format!("{}/{}", self.sites_available, self.domain);
        let config_content = self.generate_config();

        // Write the configuration file
        let mut file = File::create(&nginx_conf_file)?;
        file.write_all(config_content.as_bytes())?;

        // Create a symlink in sites-enabled
        let symlink_path = format!("{}/{}", self.sites_enabled, self.domain);
        if Path::new(&symlink_path).exists() {
            fs::remove_file(&symlink_path)?;
        }
        symlink(&nginx_conf_file, &symlink_path)?;

        // Test and reload Nginx
        std::process::Command::new("nginx")
            .arg("-t")
            .status()
            .expect("Failed to test Nginx configuration");
        std::process::Command::new("systemctl")
            .arg("reload")
            .arg("nginx")
            .status()
            .expect("Failed to reload Nginx");

        println!(
            "Nginx configuration for {} has been set up and enabled.",
            self.domain
        );

        Ok(())
    }
}
