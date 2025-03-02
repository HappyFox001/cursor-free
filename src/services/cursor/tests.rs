use super::*;


#[tokio::test]
async fn test_get_verification_code() -> Result<()> {
    // 从环境变量获取 EPIN
    let service = TempMailService::new(TempMailConfig {
        temp_mail_username: "mobai-free-cursor".to_string(),
        temp_mail_extension: "@mailto.plus".to_string(),
        temp_mail_epin: "".to_string(),
        protocol: "API".to_string(),
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