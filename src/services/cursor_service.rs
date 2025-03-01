use crate::models::CursorAccount;
use crate::services::cursor::CursorMachine;
use anyhow::Result;

pub struct CursorService {}

impl CursorService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn modify_machine_ids(&self) -> Result<CursorAccount> {
        let machine = CursorMachine::new();
        machine.modify_ids()
    }

    pub fn restore_backup(&self) -> Result<()> {
        let machine = CursorMachine::new();
        machine.restore_configs()
    }
}

impl Default for CursorService {
    fn default() -> Self {
        Self::new()
    }
}
