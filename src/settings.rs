use serde::{Deserialize, Serialize};
use std::fmt;
use toml::from_str;

use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub hub: HubSettings,
    pub solution_finder: SolutionFinderSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HubSettings {
    pub host: String,
    pub port: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolutionFinderSettings {
    pub path: Option<String>,
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "----------------------------------------")?;
        writeln!(f, "{}", self.hub)?;
        write!(f, "{}", self.solution_finder)?;
        writeln!(f, "----------------------------------------")
    }
}

impl fmt::Display for HubSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[hub]")?;
        writeln!(f, "  host: {}", self.host)?;
        writeln!(f, "  port: {}", self.port)
    }
}

impl fmt::Display for SolutionFinderSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[solution_finder]")?;
        writeln!(f, "  path: {}", self.path.clone().unwrap_or_default())
    }
}

impl Settings {
    pub fn read_file(path: &str) -> Result<Settings, String> {
        let mut file_content = String::new();
        let mut fr = File::open(path)
            .map(|f| BufReader::new(f))
            .map_err(|e| format!("{:#}", e))?;
        fr.read_to_string(&mut file_content)
            .map_err(|e| format!("{:#}", e))?;
        from_str::<Settings>(&file_content)
            .map(|v| v)
            .map_err(|e| format!("{:#}", e))
    }

    pub fn write_file(&self, path: &str) -> Result<(), String> {
        let mut file = File::create(path).map_err(|e| format!("{:#}", e))?;

        let toml = toml::to_string(&self).unwrap();
        file.write_all(toml.as_bytes())
            .map_err(|e| format!("{:#}", e))
    }
}
