pub mod platform;

pub use platform::{
    check_admin_privileges, get_current_user, get_cursor_paths, kill_cursor_processes,
};
