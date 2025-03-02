use crate::utils::color::{blue, yellow};
use anyhow::{anyhow, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use toml::Value;
pub fn get_config_path() -> Result<PathBuf> {
    let config_path = dirs::config_dir()
        .ok_or_else(|| anyhow!("无法获取配置目录"))?
        .join("cursor-backup")
        .join("paths.toml");
    Ok(config_path)
}

pub fn load_path_config() -> Result<Value> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&config_path, include_str!("../config/default_paths.toml"))?;
    }

    let content = fs::read_to_string(config_path)?;
    let config: Value = toml::from_str(&content)?;
    Ok(config)
}

pub fn get_cursor_paths() -> Result<(PathBuf, PathBuf)> {
    let home_dir = get_home_dir()?;
    let config = load_path_config()?;

    let os_config = match env::consts::OS {
        "windows" => config.get("windows"),
        "macos" => config.get("macos"),
        "linux" => config.get("linux"),
        _ => return Err(anyhow!("不支持的操作系统")),
    }
    .ok_or_else(|| anyhow!("配置文件中缺少操作系统配置"))?;

    let storage_path_str = os_config
        .get("storage_path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("配置文件中缺少 storage_path"))?;

    let state_path_str = os_config
        .get("state_path")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("配置文件中缺少 state_path"))?;

    // Windows 环境变量替换
    let storage_path_str = if cfg!(windows) {
        storage_path_str.replace(
            "%APPDATA%",
            &home_dir.join("AppData").join("Roaming").to_string_lossy(),
        )
    } else {
        storage_path_str.to_string()
    };

    let state_path_str = if cfg!(windows) {
        state_path_str.replace(
            "%APPDATA%",
            &home_dir.join("AppData").join("Roaming").to_string_lossy(),
        )
    } else {
        state_path_str.to_string()
    };

    let storage_path = if cfg!(windows) {
        PathBuf::from(storage_path_str)
    } else {
        home_dir.join(storage_path_str)
    };

    let state_path = if cfg!(windows) {
        PathBuf::from(state_path_str)
    } else {
        home_dir.join(state_path_str)
    };

    Ok((storage_path, state_path))
}

pub fn kill_cursor_processes() -> Result<()> {
    match env::consts::OS {
        "windows" => {
            Command::new("taskkill")
                .args(["/F", "/IM", "Cursor.exe"])
                .output()?;
        }
        "macos" => {
            Command::new("pkill").arg("-9").arg("Cursor").output()?;
        }
        "linux" => {
            Command::new("pkill").arg("-9").arg("cursor").output()?;
        }
        _ => return Err(anyhow!("不支持的操作系统")),
    }

    Ok(())
}

// 获取用户主目录
fn get_home_dir() -> Result<PathBuf> {
    dirs::home_dir().ok_or_else(|| anyhow!("无法获取用户主目录"))
}

// 检查是否以管理员/root权限运行
pub fn check_admin_privileges() -> Result<bool> {
    println!("{}", blue("系统检测"));
    #[cfg(target_os = "windows")]
    {
        println!("{}", yellow("Windows 系统"));
        use std::process::Command;

        // 使用PowerShell检查当前进程是否以管理员身份运行
        let output = Command::new("powershell")
            .args(["-Command", "[Security.Principal.WindowsIdentity]::GetCurrent().Groups -contains 'S-1-5-32-544'"])
            .output()?;

        // 如果命令输出包含"True"，则表示以管理员身份运行
        let is_admin = String::from_utf8_lossy(&output.stdout)
            .trim()
            .eq_ignore_ascii_case("True");

        Ok(is_admin)
    }

    #[cfg(target_os = "macos")]
    {
        println!("{}", yellow("macOS 系统"));
        use std::process::Command;

        // 检查是否为root用户
        let output = Command::new("id").arg("-u").output()?;

        let uid_str = String::from_utf8_lossy(&output.stdout);
        let uid = uid_str.trim();
        let is_root = uid == "0";

        Ok(is_root)
    }

    #[cfg(target_os = "linux")]
    {
        println!("{}", yellow("Linux 系统"));
        use std::process::Command;

        // 检查是否为root用户
        let output = Command::new("id").arg("-u").output()?;

        let uid_str = String::from_utf8_lossy(&output.stdout);
        let uid = uid_str.trim();
        let is_root = uid == "0";

        Ok(is_root)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Ok(false) // 不支持的平台
    }
}

// 获取当前登录用户名
pub fn get_current_user() -> Result<String> {
    #[cfg(target_os = "windows")]
    {
        use std::env;

        let username = env::var("USERNAME")?;
        Ok(username)
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        use std::env;

        let username = env::var("USER")?;
        Ok(username)
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Err(anyhow!("不支持的操作系统"))
    }
}
