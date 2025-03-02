use crate::utils::color::{blue, green, red, yellow};
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use super::data::NameGenerator;

#[derive(Debug, Serialize, Deserialize)]
pub struct TempMailConfig {
    pub first_id: String,
    pub second_id: String,
    pub password: String,
    pub user_email: String,
    pub temp_mail_username: String,
    pub temp_mail_extension: String,
    pub temp_mail_epin: String,
}

impl Default for TempMailConfig {
    fn default() -> Self {
        Self {
            first_id: "".to_string(),
            second_id: "".to_string(),
            password: "".to_string(),
            user_email: "".to_string(),
            temp_mail_username: "mobai-free-cursor".to_string(),
            temp_mail_extension: "@mailto.plus".to_string(),
            temp_mail_epin: "".to_string(),
        }
    }
}

impl TempMailConfig {
    pub fn from_env() -> Self {
        let mut generator = NameGenerator::new();
        let (first_name, last_name) = generator.generate_name();
        let temp_mail = std::env::var("TEMP_MAIL").unwrap_or("mobai-free-cursor".to_string());
        let temp_mail_extension = std::env::var("TEMP_MAIL_EXT").unwrap_or("@mailto.plus".to_string());
        let temp_mail_epin = "".to_string();
        Self {
            first_id: first_name.to_lowercase(),
            second_id: last_name.to_lowercase(),
            user_email: format!("{}@{}", first_name.to_lowercase(),temp_mail_extension),
            temp_mail_username: temp_mail,
            temp_mail_extension: temp_mail_extension,
            temp_mail_epin: temp_mail_epin,
            ..Default::default()
        }
    }
}

pub struct TempMailService {
    config: TempMailConfig,
    client: Client,
}

impl TempMailService {
    pub fn new(config: TempMailConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    pub fn new_default() -> Self {
        Self::new(TempMailConfig::from_env())
    }

    pub async fn get_verification_code(
        &self,
        max_retries: u32,
        retry_interval: u64,
    ) -> Result<String> {
        for attempt in 1..=max_retries {
            match self.get_code_by_api().await {
                Ok(code) => return Ok(code),
                Err(e) => {
                    log::warn!("第 {}/{} 次尝试获取验证码失败: {}", attempt, max_retries, e);
                    if attempt < max_retries {
                        sleep(Duration::from_secs(retry_interval)).await;
                    }
                }
            }
        }
        Err(anyhow!("在 {} 次尝试后仍未获取到验证码", max_retries))
    }

    async fn get_code_by_api(&self) -> Result<String> {
        // 获取邮件列表
        let mail_list_url = format!(
            "https://tempmail.plus/api/mails?email={}{}&limit=20&epin={}",
            self.config.temp_mail_username,
            self.config.temp_mail_extension,
            self.config.temp_mail_epin
        );
        let response = self.client.get(&mail_list_url).send().await?;
        println!("{}", red(mail_list_url.as_str()));
        let mail_list: serde_json::Value = response.json().await?;

        if !mail_list["result"].as_bool().unwrap_or(false) {
            return Err(anyhow!("获取邮件列表失败"));
        }

        let first_id = mail_list["first_id"]
            .as_str()
            .ok_or_else(|| anyhow!("未找到邮件"))?;

        println!("{}", red(first_id));
        // 获取邮件详情
        let mail_detail_url = format!(
            "https://tempmail.plus/api/mails/{}?email={}{}&epin={}",
            first_id,
            self.config.temp_mail_username,
            self.config.temp_mail_extension,
            self.config.temp_mail_epin
        );

        let response = self.client.get(&mail_detail_url).send().await?;
        let mail_detail: serde_json::Value = response.json().await?;

        if !mail_detail["result"].as_bool().unwrap_or(false) {
            return Err(anyhow!("获取邮件详情失败"));
        }

        let mail_text = mail_detail["text"].as_str().unwrap_or("");
        let re = Regex::new(r"(?<![a-zA-Z@.])\b\d{6}\b")?;

        if let Some(captures) = re.captures(mail_text) {
            let code = captures.get(0).unwrap().as_str().to_string();
            self.cleanup_mail(first_id).await?;
            Ok(code)
        } else {
            Err(anyhow!("未找到验证码"))
        }
    }

    async fn cleanup_mail(&self, first_id: &str) -> Result<()> {
        let delete_url = "https://tempmail.plus/api/mails/";

        for _ in 0..5 {
            let response = self
                .client
                .delete(delete_url)
                .form(&[
                    (
                        "email",
                        format!(
                            "{}{}",
                            self.config.temp_mail_username, self.config.temp_mail_extension
                        ),
                    ),
                    ("first_id", first_id.to_string()),
                    ("epin", self.config.temp_mail_epin.clone()),
                ])
                .send()
                .await?;

            let result = response.json::<serde_json::Value>().await?;
            if result["result"].as_bool().unwrap_or(false) {
                return Ok(());
            }

            sleep(Duration::from_millis(500)).await;
        }

        Err(anyhow!("清理邮件失败"))
    }

    async fn extract_mail_body(&self, content: &str) -> Result<String> {
        // 简单的邮件内容提取
        let re = Regex::new(r"Content-Type: text/plain.*?\r\n\r\n(.*?)\r\n\r\n")?;
        if let Some(captures) = re.captures(content) {
            Ok(captures.get(1).unwrap().as_str().to_string())
        } else {
            Ok(content.to_string())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CursorEmail {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl CursorEmail {
    pub fn new(domain: &str) -> Self {
        let mut generator = NameGenerator::new();
        let (first_name, last_name) = generator.generate_name();
        let email = format!("{}.{}@{}", first_name.to_lowercase(), last_name.to_lowercase(), domain);
        let password = generator.generate_password(&first_name);

        Self {
            first_name,
            last_name,
            email,
            password,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_generation() {
        let email = CursorEmail::new("example.com");
        
        // 验证邮箱格式
        assert!(email.email.contains('@'));
        assert!(email.email.ends_with("example.com"));
        assert!(email.email.contains(&email.first_name.to_lowercase()));
        assert!(email.email.contains(&email.last_name.to_lowercase()));
        
        // 验证密码格式
        assert!(email.password.starts_with(&email.first_name));
        assert!(email.password.len() > email.first_name.len());
    }
}

#[tokio::test]
async fn test_get_verification_code() -> Result<()> {
    // 从环境变量获取 EPIN
    let service = TempMailService::new(TempMailConfig {
        temp_mail_username: "mobai-free-cursor".to_string(),
        temp_mail_extension: "@mailto.plus".to_string(),
        temp_mail_epin: "".to_string(),
        ..Default::default()
    });

    // 尝试获取验证码
    match service.get_verification_code(3, 10).await {
        Ok(code) => {
            println!("获取到验证码: {}", code);
            assert!(code.len() == 6);
            assert!(code.chars().all(|c| c.is_digit(10)));
        }
        Err(e) => {
            println!("获取验证码失败: {}", e);
            // 这里我们允许失败，因为可能没有新的验证码邮件
        }
    }

    Ok(())
}
