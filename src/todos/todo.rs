use std::fmt::Display;

pub struct Todo {
    pub idx: usize,
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn edit_todo(&mut self, new_text: String) {
        self.text = new_text;
    }

    pub fn toggle_complete(&mut self) {
        self.completed = !self.completed;
    }
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let completed_text = if self.completed {
            "completed"
        } else {
            "not completed"
        };

        return write!(f, "[{}]: {} ({})", self.idx, self.text, completed_text);
    }
}
