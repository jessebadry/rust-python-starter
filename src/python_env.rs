use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::Path;
use std::process::{Command, Output};

use crate::config::PythonConfig;

// Extension trait for ergonomic error handling
trait AnyhowExt<T> {
    fn with_msg(self, msg: &str) -> Result<T>;
}

impl<T, E> AnyhowExt<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn with_msg(self, msg: &str) -> Result<T> {
        self.map_err(|e| anyhow::anyhow!("{}: {}", msg, e.into()))
    }
}

// Helper for command output validation
fn validate_output(output: Output, error_msg: &str) -> Result<Output> {
    if !output.status.success() {
        anyhow::bail!("{}: {}", error_msg, String::from_utf8_lossy(&output.stderr));
    }
    Ok(output)
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PythonEnv {
    venv_path: String,
    python_exe: String,
    pip_exe: String,
    python_alias: String,
}

impl PythonEnv {
    pub fn new() -> Result<Self> {
        let venv_path = "./venv".to_string();
        let (python_exe, pip_exe) = if cfg!(windows) {
            (
                format!("{}/Scripts/python.exe", venv_path),
                format!("{}/Scripts/pip.exe", venv_path),
            )
        } else {
            (
                format!("{}/bin/python", venv_path),
                format!("{}/bin/pip", venv_path),
            )
        };

        Ok(PythonEnv {
            venv_path,
            python_exe,
            pip_exe,
            python_alias: "python".to_string(),
        })
    }

    pub fn new_with_config(config: &PythonConfig) -> Result<Self> {
        let (python_exe, pip_exe) = if cfg!(windows) {
            (
                format!("{}/Scripts/{}", config.venv_path, config.python_exe),
                format!("{}/Scripts/pip.exe", config.venv_path),
            )
        } else {
            (
                format!("{}/bin/{}", config.venv_path, config.python_exe),
                format!("{}/bin/pip3", config.venv_path),
            )
        };

        Ok(PythonEnv {
            venv_path: config.venv_path.clone(),
            python_exe,
            pip_exe,
            python_alias: "python".to_string(),
        })
    }

    pub fn is_setup(&self) -> bool {
        Path::new(&self.venv_path).exists() && Path::new(&self.python_exe).exists()
    }

    pub fn setup(&self) -> Result<()> {
        self.setup_with_requirements("requirements.txt")
    }

    pub fn setup_with_requirements(&self, requirements_file: &str) -> Result<()> {
        if self.is_setup() {
            return Ok(());
        }

        println!("Setting up Python virtual environment...");

        // Create venv - try python3 first, then python
        let python_cmd = if Command::new("python3").arg("--version").output().is_ok() {
            "python3"
        } else {
            "python"
        };
        
        Command::new(python_cmd)
            .args(&["-m", "venv", &self.venv_path])
            .output()
            .with_msg("Failed to execute python command")
            .and_then(|output| validate_output(output, "Failed to create virtual environment"))?;

        // Install requirements if file exists
        if Path::new(requirements_file).exists() {
            let pip_exe = if cfg!(windows) {
                format!("{}/Scripts/pip.exe", self.venv_path)
            } else {
                format!("{}/bin/pip", self.venv_path)
            };

            Command::new(&pip_exe)
                .args(&["install", "-r", requirements_file])
                .output()
                .with_msg("Failed to execute pip install")
                .and_then(|output| validate_output(output, "Failed to install requirements"))?;
        }

        println!("Python environment ready!");

        Ok(())
    }

    pub fn run_script(&self, script_path: &str, args: &[&str]) -> Result<String> {
        if !self.is_setup() {
            anyhow::bail!("Python environment not set up. Call setup() first.");
        }
        let mut cmd_args = vec![script_path];
        cmd_args.extend_from_slice(args);

        // Set working directory based on build profile
        let working_dir = if cfg!(debug_assertions) {
            "./src/scripts"
        } else {
            "./scripts"
        };

        // Use absolute path for Python executable to ensure venv is used
        let current_dir = current_dir().context("Failed to get current directory")?;
        let abs_python_exe = current_dir.join(&self.python_exe);

        
        let output = Command::new(&abs_python_exe)
            .args(&cmd_args)
            .current_dir(working_dir)
            .output()
            .with_msg(&format!("Failed to execute Python script from directory: {}", working_dir))
            .and_then(|output| validate_output(output, "Python script failed"))?;

        String::from_utf8(output.stdout).with_msg("Failed to parse script output as UTF-8")
    }
}
