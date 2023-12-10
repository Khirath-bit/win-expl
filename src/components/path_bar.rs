#[derive(Default)]
pub struct PathBar {
    pub view: nwg::TextBox,
}

impl PathBar {
    pub fn get_path(&self) -> Result<String, String> {
        if self.view.text().is_empty() {
            self.view.set_text("C:");
            return Err(String::from("No path found"));
        }

        Ok(String::from(self.view.text().replace(' ', "").trim()) + "\\")
    }

    pub fn move_into_directory(&self, dir_path: String){
        self.view.clear();
        self.view.set_text(&dir_path);
    }

    pub fn depth(&self) -> usize {
        self.view.text().matches('\\').count()
    }

    pub fn move_one_up(&self) {
        let path_parts: Vec<String> = self.view.text().split('\\').map(|s| s.to_string()).collect();
        if let Some(first) = path_parts.first() {
            let result = path_parts.iter().skip(1).take(path_parts.len() - 2).fold(first.clone(), |acc, part| acc + "\\" + part);
            self.view.set_text(&result);
        }
    }
}