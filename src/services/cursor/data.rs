use lazy_static::lazy_static;
use rand::prelude::*;

lazy_static! {
    static ref FIRST_NAMES: Vec<String> = load_names("first_names");
    static ref LAST_NAMES: Vec<String> = load_names("last_names");
}

fn load_names(name_type: &str) -> Vec<String> {
    let content = include_str!("../data.txt");
    content
        .lines()
        .filter(|line| line.starts_with(name_type))
        .map(|line| line.split('=').nth(1).unwrap_or("").trim().to_string())
        .collect()
}

pub struct NameGenerator {
    rng: ThreadRng,
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
        }
    }

    pub fn generate_name(&mut self) -> (String, String) {
        let first_name = self.generate_random_name(&FIRST_NAMES);
        let last_name = self.generate_random_name(&LAST_NAMES);
        (first_name, last_name)
    }

    fn generate_random_name(&mut self, names: &[String]) -> String {
        let name = names.choose(&mut self.rng)
            .unwrap_or(&"Default".to_string())
            .to_string();
        
        // 添加随机数字
        let number: u32 = self.rng.gen_range(100..999);
        format!("{}{}", name, number)
    }

    pub fn generate_password(&mut self, first_name: &str) -> String {
        let number: u32 = self.rng.gen_range(100000..999999);
        format!("{}{}", first_name, number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_generation() {
        let mut generator = NameGenerator::new();
        let (first_name, last_name) = generator.generate_name();
        
        // 验证名字格式
        assert!(!first_name.is_empty());
        assert!(!last_name.is_empty());
        assert!(first_name.chars().any(|c| c.is_digit(10)));
        assert!(last_name.chars().any(|c| c.is_digit(10)));
    }

    #[test]
    fn test_password_generation() {
        let mut generator = NameGenerator::new();
        let (first_name, _) = generator.generate_name();
        let password = generator.generate_password(&first_name);
        
        // 验证密码格式
        assert!(password.starts_with(&first_name));
        assert!(password.chars().skip(first_name.len()).all(|c| c.is_digit(10)));
        assert!(password.len() > first_name.len());
    }
} 