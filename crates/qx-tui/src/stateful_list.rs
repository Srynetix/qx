use ratatui::widgets::ListState;

pub struct StatefulList {
    data: Vec<String>,
    state: ListState,
}

impl StatefulList {
    pub fn new(data: Vec<String>) -> Self {
        Self {
            data,
            state: ListState::default(),
        }
    }

    pub fn select_first_if_exists(&mut self) {
        if !self.data.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn select_next(&mut self) {
        if let Some(value) = self.state.selected() {
            let next_value = (value + 1) % self.data.len();
            self.state.select(Some(next_value));
        } else {
            self.select_first_if_exists()
        }
    }

    pub fn select_previous(&mut self) {
        if let Some(value) = self.state.selected() {
            let next_value = ((value as isize) - 1).rem_euclid(self.data.len() as isize) as usize;
            self.state.select(Some(next_value));
        } else {
            self.select_first_if_exists()
        }
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn iter(&self) -> impl Iterator<Item = String> + '_ {
        self.data.iter().map(|s| s.to_owned())
    }

    pub fn state_mut(&mut self) -> &mut ListState {
        &mut self.state
    }
}
