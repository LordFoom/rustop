use std::{cmp::Ordering, time::Instant};

use anyhow::Result;
use crossterm::event::KeyCode;
use ratatui::widgets::TableState;
use users::UsersCache;

use crate::{
    model::{ProcessInfo, SortBy},
    processes::get_process_info,
};

pub struct App {
    pub processes: Vec<ProcessInfo>,
    pub sort_by: Option<SortBy>,
    pub user_cache: UsersCache,
    pub refresh_count: u8,
    pub last_refresh: Instant,
    pub table_state: TableState,
    pub should_quit: bool,
    ///This will place the selection at the top when sorting changes
    pub should_go_to_top: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            sort_by: None,
            user_cache: UsersCache::new(),
            refresh_count: 0,
            last_refresh: Instant::now(),
            table_state: {
                let mut state = TableState::default();
                state.select(Some(0));
                state
            },
            should_quit: false,
            should_go_to_top: false,
        }
    }

    pub fn select(&mut self, i: usize) {
        let idx = if i >= self.processes.len() {
            self.processes.len()
        } else {
            i
        };
        self.table_state.select(Some(idx));
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Char('c') | KeyCode::Char('C') => self.handle_sort(SortBy::Cpu),
            KeyCode::Char('m') | KeyCode::Char('M') => self.handle_sort(SortBy::Memory),
            KeyCode::Char('p') | KeyCode::Char('P') => self.handle_sort(SortBy::Pid),
            KeyCode::Char('n') | KeyCode::Char('N') => self.handle_sort(SortBy::Name),
            KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => self.next_process(),
            KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => self.previous_process(),
            _ => {}
        }
    }

    ///Set the sort by
    fn handle_sort(&mut self, sort: SortBy) {
        self.sort_by = Some(sort);
        self.should_go_to_top = true;
    }

    pub fn next_process(&mut self) {
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

    pub fn previous_process(&mut self) {
        let i = match self.table_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.processes.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.table_state.select(Some(i));
    }

    pub fn update_processes(&mut self) -> Result<()> {
        if self.last_refresh.elapsed().as_millis() >= 250 {
            self.processes = get_process_info(&mut self.user_cache)?;
            match self.sort_by {
                Some(SortBy::Cpu) => self.processes.sort_by(|a, b| {
                    a.cpu_percent
                        .partial_cmp(&b.cpu_percent)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }),
                Some(SortBy::Memory) => self.processes.sort_by(|a, b| {
                    a.memory_kb
                        .partial_cmp(&b.memory_kb)
                        .unwrap_or(Ordering::Equal)
                }),
                Some(SortBy::Pid) => self
                    .processes
                    .sort_by(|a, b| a.pid.partial_cmp(&b.pid).unwrap_or(Ordering::Equal)),
                Some(SortBy::Name) => self
                    .processes
                    .sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap_or(Ordering::Equal)),
                None => {}
            }
            if self.refresh_count % 100 == 0 {
                self.user_cache = UsersCache::new();
                self.refresh_count = 0;
            } else {
                self.refresh_count += 1;
            }
            self.last_refresh = Instant::now();
            if self.should_go_to_top {
                self.select(0);
                self.should_go_to_top = false;
            }
        }

        Ok(())
    }
}
