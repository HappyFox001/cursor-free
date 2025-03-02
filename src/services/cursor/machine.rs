use crate::models::CursorAccount;
use crate::utils::color::{green, red, yellow};
use crate::utils::{
    check_admin_privileges, get_current_user, get_cursor_paths, kill_cursor_processes,
};
use anyhow::{bail, Result};
use chrono::Utc;
use rand::{thread_rng, Rng};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

pub struct CursorMachine;

impl CursorMachine {
    pub fn new() -> Self {
        Self
    }

    pub fn modify_ids(&self) -> Result<CursorAccount> {
        if !check_admin_privileges()? {
            bail!("需要管理员权限才能修改机器码");
        }

        let username = get_current_user()?;
        println!("{} {}", yellow("当前用户: "), green(username.as_str()));

        println!("正在关闭 Cursor...");
        if let Err(e) = kill_cursor_processes() {
            println!("{} {}", red("警告: "), red(e.to_string().as_str()));
            println!("{}", yellow("请确保手动关闭 Cursor 后继续"));
        }

        // 备份原始配置
        self.backup_configs()?;

        let machine_id = self.generate_id();
        let device_id = self.generate_id();
        let mac_machine_id = self.generate_id();

        if let Err(e) = self.update_config_files(&machine_id, &device_id, &mac_machine_id) {
            println!("修改配置失败，正在尝试恢复备份...");
            if let Err(restore_err) = self.restore_configs() {
                bail!(
                    "修改失败且恢复备份也失败！原始错误: {}, 恢复错误: {}",
                    e,
                    restore_err
                );
            }
            bail!("修改失败，已恢复到原始配置: {}", e);
        }

        let account = CursorAccount::new(
            username,
            String::new(),
            String::new(),
            machine_id,
            device_id,
            mac_machine_id,
        );

        Ok(account)
    }

    pub fn backup_configs(&self) -> Result<()> {
        println!("正在备份原始配置...");
        let (storage_path, user_data_path) = get_cursor_paths()?;
        let backup_dir = self.get_backup_dir()?;

        // 创建备份目录
        fs::create_dir_all(&backup_dir)?;

        // 备份时间戳
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();

        // 备份 storage.json
        if storage_path.exists() {
            let backup_path = backup_dir.join(format!("storage_{}.json", timestamp));
            fs::copy(&storage_path, &backup_path)?;
        }

        // 备份 state.json
        if user_data_path.exists() {
            let backup_path = backup_dir.join(format!("state_{}.json", timestamp));
            fs::copy(&user_data_path, &backup_path)?;
        }

        println!("{}", green("配置备份完成"));
        Ok(())
    }

    pub fn restore_configs(&self) -> Result<()> {
        println!("正在恢复最近的备份...");
        let backup_dir = self.get_backup_dir()?;
        let (storage_path, user_data_path) = get_cursor_paths()?;

        // 获取最新的备份文件
        let backup_files: Vec<_> = fs::read_dir(&backup_dir)?
            .filter_map(|entry| entry.ok())
            .collect();

        // 恢复 storage.json
        if let Some(latest_storage) = backup_files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("storage_"))
            .max_by_key(|f| f.file_name())
        {
            fs::copy(latest_storage.path(), &storage_path)?;
        }

        // 恢复 state.json
        if let Some(latest_state) = backup_files
            .iter()
            .filter(|f| f.file_name().to_string_lossy().starts_with("state_"))
            .max_by_key(|f| f.file_name())
        {
            fs::copy(latest_state.path(), &user_data_path)?;
        }

        println!("{}", green("配置已恢复到最近的备份"));
        Ok(())
    }

    fn get_backup_dir(&self) -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("无法获取用户主目录"))?;
        Ok(home_dir.join(".cursor_backup"))
    }

    fn update_config_files(
        &self,
        machine_id: &str,
        device_id: &str,
        mac_machine_id: &str,
    ) -> Result<()> {
        let (storage_path, user_data_path) = get_cursor_paths()?;

        for path in [&storage_path, &user_data_path] {
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }
        }

        // 更新 storage.json
        let mut storage_config: serde_json::Value = if storage_path.exists() {
            let content = fs::read_to_string(&storage_path)?;
            serde_json::from_str(&content)?
        } else {
            json!({})
        };

        storage_config["telemetryMachineId"] = json!(machine_id);
        storage_config["telemetryDevDeviceId"] = json!(device_id);
        storage_config["telemetryMacMachineId"] = json!(mac_machine_id);

        fs::write(
            &storage_path,
            serde_json::to_string_pretty(&storage_config)?,
        )?;

        // 更新 state.json
        let mut state_config: serde_json::Value = if user_data_path.exists() {
            let content = fs::read_to_string(&user_data_path)?;
            serde_json::from_str(&content)?
        } else {
            json!({})
        };

        if let Some(obj) = state_config.as_object_mut() {
            obj.insert("machineId".to_string(), json!(machine_id));
            obj.insert("deviceId".to_string(), json!(device_id));
            obj.insert("macMachineId".to_string(), json!(mac_machine_id));
        }

        fs::write(
            &user_data_path,
            serde_json::to_string_pretty(&state_config)?,
        )?;

        Ok(())
    }

    fn generate_id(&self) -> String {
        let mut rng = thread_rng();
        let id: u64 = rng.gen();
        format!("{:016x}", id)
    }
}
