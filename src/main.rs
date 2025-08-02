use anyhow::Result;

mod python_env;
mod config;

use config::Config;

fn main() -> Result<()> {
    
    let config = Config::load("config.toml")?;
    println!("Loaded config: {}", config.app_name);
    
    let python_env = config.create_python_env()?;
    println!("Python venv path: {}", config.python.venv_path);
    
    python_env.setup()?;
    
    println!("Rust Python CLI setup");
    println!("Use `cargo test test_python_integration` to test Python integration");
    let output = python_env.run_script("hello.py", &["test"])?;
    println!("Python script output:\n{}", output);

    Ok(())
}
