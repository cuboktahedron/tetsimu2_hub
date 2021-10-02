use anyhow::Result;
use env_logger;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tetsimu2_hub::hub::hub_server::HubServer;
use tetsimu2_hub::settings::HubSettings;
use tetsimu2_hub::settings::Settings;
use tetsimu2_hub::settings::SolutionFinderSettings;

fn start_server(settings: Settings) -> Result<()> {
    HubServer::listen(Arc::new(settings))
}

const CONFIG_FILE: &str = "config.toml";

fn main() {
    env_logger::init();
    println!("Tetsimu2 Hub 0.0.1");

    println!("Load {}", CONFIG_FILE);
    let config = match load_config(CONFIG_FILE) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    println!(
        "Starting websocket server. Listening on {}:{}",
        config.hub.host, config.hub.port
    );
    println!("Press Ctrl + c to Exit");

    if let Err(e) = start_server(config) {
        eprintln!("{:?}", e);
    }
}

fn load_config(file_path: &str) -> Result<Settings, String> {
    if !Path::new(file_path).exists() {
        println!("'{}' is not found.", file_path);
        println!("You need to configure settings.\n");

        return init_config(file_path);
    }

    Settings::read_file(CONFIG_FILE)
}

fn init_config(path: &str) -> Result<Settings, String> {
    let host = input_host()?;
    let port = input_port()?;
    let sf_path = input_sf_path()?;
    println!("");

    let settings = Settings {
        hub: HubSettings { host, port },
        solution_finder: SolutionFinderSettings { path: sf_path },
    };

    if let Err(e) = settings.write_file(&format!("{}", path)) {
        return Err(e);
    }

    println!("Cofiguration completed");

    Ok(settings)
}

fn input_host() -> Result<String, String> {
    print!("Hub hostname or ip(default: localhost): ");
    stdout().flush().ok();

    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    let host = String::from(input.trim());
    if host.is_empty() {
        return Ok(String::from("localhost"));
    } else {
        return Ok(host);
    }
}

fn input_port() -> Result<u32, String> {
    print!("Hub port(default: 3012): ");
    stdout().flush().ok();

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let port = String::from(input.trim());
        if port.is_empty() {
            return Ok(3012);
        }
        if let Ok(x) = port.parse() {
            return Ok(x);
        }
    }
}

fn input_sf_path() -> Result<Option<String>, String> {
    print!("Solution finder root directory(optional): ");
    stdout().flush().ok();

    let mut input = String::new();
    stdin().read_line(&mut input).map_err(|e| e.to_string())?;
    let sf_path = String::from(input.trim());
    if sf_path.is_empty() {
        return Ok(None);
    } else {
        return Ok(Some(sf_path));
    }
}
