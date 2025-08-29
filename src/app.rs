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

impl App {
    fn new() -> Self {
        Self {
            processes: Vec::new(),
            sort_by: None,
            user_cache: UsersCache::new(),
            refresh_count: 0,
            last_refresh: Instant::now(),
            table_state: TableState::default(),
            should_quit: false,
        }
    }

    fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('c') | KeyCode::Char('C') => self.sort_by = Some(SortBy::Cpu),
            KeyCode::Char('m') | KeyCode::Char('M') => self.sort_by = Some(SortBy::Memory),
            KeyCode::Char('p') | KeyCode::Char('P') => self.sort_by = Some(SortBy::Pid),
            KeyCode::Char('n') | KeyCode::Char('N') => self.sort_by = Some(SortBy::Name),
            KeyCode::Up => self.previous_process(),
            KeyCode::Down => self.next_process(),
            _ => {}
        }
    }
}
