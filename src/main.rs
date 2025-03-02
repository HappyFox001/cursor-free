use anyhow::Result;
use clap::Parser;
use env_logger::Env;
use std::env;
use std::fs;
use std::process::Command;
mod cli;
mod models;
mod services;
mod utils;
use cli::Cli;
use services::CursorService;
use utils::check_admin_privileges;
use utils::color::{blue, green, red, yellow};
use utils::platform::get_config_path;

async fn show_menu(cursor_service: &CursorService) -> Result<bool> {
    println!("\n{}", blue("【选择操作】"));
    println!("1. 修改机器码（自动备份）");
    println!("2. 恢复最近的备份");
    println!("3. 修改 Cursor 路径配置");
    println!("4. 退出程序");
    println!("\n请选择操作 [1-4]: ");

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;

    match choice.trim() {
        "1" => {
            println!("\n{}", blue("▶ 开始处理..."));

            match cursor_service.modify_machine_ids() {
                Ok(account) => {
                    println!("\n{}", green("✅ 机器码修改成功！"));

                    println!("\n{}", blue("【机器码信息】"));
                    println!("机器码: {}", account.machine_id);
                    println!("设备码: {}", account.device_id);
                    println!("Mac机器码: {}", account.mac_machine_id);

                    println!("\n{}", green("所有操作已完成！"));
                    println!("{}", yellow("请重启 Cursor 以使更改生效。"));

                    println!("\n是否立即启动 Cursor？[y/N]: ");
                    let mut launch_answer = String::new();
                    std::io::stdin().read_line(&mut launch_answer)?;

                    if launch_answer.trim().eq_ignore_ascii_case("y") {
                        println!("\n{}", blue("正在启动 Cursor..."));

                        let launch_result = match env::consts::OS {
                            "macos" => Command::new("open")
                                .arg("/Applications/Cursor.app")
                                .status(),
                            "windows" => Command::new("cmd")
                                .args(["/c", "start", "", "Cursor.exe"])
                                .status(),
                            "linux" => Command::new("cursor").status(),
                            _ => Err(std::io::Error::new(
                                std::io::ErrorKind::Other,
                                "不支持的操作系统",
                            )),
                        };

                        match launch_result {
                            Ok(_) => println!("{}", green("Cursor 已启动！")),
                            Err(e) => println!("{}", red(&format!("无法启动 Cursor: {}", e))),
                        }
                    }
                }
                Err(e) => {
                    eprintln!("\n{}", red(&format!("❌ 机器码修改失败：{}", e)));
                    eprintln!("{}", yellow("请确保以管理员权限运行本程序。"));
                    std::process::exit(1);
                }
            }
            Ok(true)
        }
        "2" => {
            println!("\n{}", blue("▶ 开始恢复备份..."));
            if let Err(e) = cursor_service.restore_backup() {
                eprintln!("\n{}", red(&format!("❌ 恢复备份失败：{}", e)));
                std::process::exit(1);
            }
            println!("{}", green("✅ 备份恢复成功！"));
            println!("{}", yellow("请重启 Cursor 以使更改生效。"));
            Ok(true)
        }
        "3" => {
            println!("\n{}", blue("▶ 修改 Cursor 路径配置"));

            let config_path = get_config_path()?;

            if !config_path.exists() {
                println!("{}", yellow("配置文件不存在，将创建默认配置文件。"));
                utils::platform::load_path_config()?;
            }

            println!("\n当前配置文件位置: {}", config_path.display());

            let content = fs::read_to_string(&config_path)?;
            println!("\n当前配置内容:\n{}", content);

            println!("\n{}", blue("请选择操作："));
            println!("1. 使用默认编辑器打开配置文件");
            println!("2. 返回主菜单");
            println!("\n请选择 [1-2]: ");

            let mut edit_choice = String::new();
            std::io::stdin().read_line(&mut edit_choice)?;

            match edit_choice.trim() {
                "1" => {
                    println!("\n{}", blue("正在打开配置文件..."));

                    let open_result = match env::consts::OS {
                        "windows" => Command::new("notepad").arg(&config_path).status(),
                        "macos" => Command::new("open").arg("-t").arg(&config_path).status(),
                        "linux" => {
                            if Command::new("xdg-open").arg(&config_path).status().is_ok() {
                                Command::new("true").status()
                            } else {
                                Command::new("nano").arg(&config_path).status()
                            }
                        }
                        _ => Err(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "不支持的操作系统",
                        )),
                    };

                    match open_result {
                        Ok(_) => {
                            println!("{}", green("✅ 配置文件已打开！"));
                            println!("{}", yellow("修改配置文件后，需要重启程序才能生效。"));
                        }
                        Err(e) => println!("{}", red(&format!("无法打开配置文件: {}", e))),
                    }
                }
                "2" => {
                    println!("\n{}", blue("返回主菜单..."));
                    return Ok(true);
                }
                _ => println!("\n{}", red("无效选项，请重新选择。")),
            }
            Ok(true)
        }
        "4" | _ => {
            println!("\n{}", blue("感谢使用，再见！"));
            Ok(false)
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dotenv::dotenv().ok();

    let _cli = Cli::parse();

    println!("{}", blue("========================================="));
    println!("{}", blue("         Cursor 机器码修改工具           "));
    println!("{}", blue("========================================="));
    println!();

    // 检查权限
    if let Ok(false) = check_admin_privileges() {
        println!("{}", yellow("⚠️ 警告：当前程序不是以管理员权限运行的！"));
        println!("{}", yellow("修改 Cursor 机器码需要管理员权限。"));
        println!();

        if env::consts::OS == "windows" {
            println!("请右键点击本程序，选择「以管理员身份运行」。");
        } else if env::consts::OS == "macos" || env::consts::OS == "linux" {
            println!("请使用 sudo 命令运行本程序：");
            println!("sudo ./cursor-automation");
        }

        println!("\n是否继续？(可能会失败) [y/N]: ");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        if !answer.trim().eq_ignore_ascii_case("y") {
            return Ok(());
        }
    }

    let cursor_service = CursorService::new();

    // 菜单循环
    while show_menu(&cursor_service).await? {}

    Ok(())
}
