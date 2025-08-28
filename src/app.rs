use ratatui::widgets::TableState;
use users::UsersCache;

use crate::model::{ProcessInfo, SortBy};

pub struct App {
    processes: Vec<ProcessInfo>,
    sort_by: Option<SortBy>,
    user_cache: UsersCache,
    refresh_count: u8,
    last_refresh: Instant,
    table_state: TableState,
    should_quit: bool,
}
