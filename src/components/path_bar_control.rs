use std::cell::RefCell;

use crate::ring_buffer::RingBuffer;

#[derive(Default)]
pub struct PathBarControl {
    pub view: nwg::TextBox,
    last_page: RefCell<RingBuffer<String>>,
}

impl PathBarControl {
    pub fn get_path(&self) -> Result<String, String> {
        if self.view.text().is_empty() {
            self.view.set_text("C:");
            return Err(String::from("No path found"));
        }

        Ok(String::from(self.view.text().trim()) + "\\")
    }

    pub fn move_into_directory(&self, dir_path: String) {
        self.last_page.borrow_mut().push(self.view.text());
        self.view.clear();
        self.view.set_text(&dir_path);
    }

    pub fn depth(&self) -> usize {
        self.view.text().matches('\\').count()
    }

    pub fn any_last_page(&self) -> bool {
        !self.last_page.borrow_mut().all_read()
    }

    #[allow(unused_assignments)]
    pub fn move_one_back(&self) {
        let mut val = self.last_page.borrow_mut().pop();
        if val.is_none() {
            return;
        }
        self.view.set_text(&val.unwrap());
        val = Some(self.view.text());
    }

    pub fn move_one_up(&self) {
        let path_parts: Vec<String> = self
            .view
            .text()
            .split('\\')
            .map(|s| s.to_string())
            .collect();
        if let Some(first) = path_parts.first() {
            let result = path_parts
                .iter()
                .skip(1)
                .take(path_parts.len() - 2)
                .fold(first.clone(), |acc, part| acc + "\\" + part);
            self.view.set_text(&result);
            self.last_page.borrow_mut().push(self.view.text());
        }
    }
}
