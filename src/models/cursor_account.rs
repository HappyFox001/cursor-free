use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CursorAccount {
    pub email: String,
    pub access_token: String,
    pub refresh_token: String,
    pub machine_id: String,
    pub device_id: String,
    pub mac_machine_id: String,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
    pub is_active: bool,
    pub login_status: LoginStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LoginStatus {
    Pending,
    LoggedIn,
    Failed,
}

impl CursorAccount {
    pub fn new(
        email: String,
        access_token: String,
        refresh_token: String,
        machine_id: String,
        device_id: String,
        mac_machine_id: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            email,
            access_token,
            refresh_token,
            machine_id,
            device_id,
            mac_machine_id,
            created_at: now,
            last_used_at: now,
            is_active: true,
            login_status: LoginStatus::LoggedIn,
        }
    }
}
