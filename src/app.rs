use crossterm::event::KeyCode;
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
            tablejjjjjjj_state: TableState::default(),
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
            KeyCode::Up | KeyCode::Char('j') | KeyCode::Char('J') => self.previous_process(),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => self.next_process(),
            _ => {}
        }
    }

    fn next_process(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i >= self.processes.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    fn previous_process(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i <= 0 {
                    self.processes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }
}
