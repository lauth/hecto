pub struct FileType {
    name: String,
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

