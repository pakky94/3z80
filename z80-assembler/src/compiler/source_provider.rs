#[derive(Clone)]
pub struct SourceHeader {
    pub filename: String,
}

pub trait SourceProvider {
    fn file_list(&self) -> Vec<SourceHeader>;
    fn source(&self, filename: &str) -> String;
}

pub struct InMemorySourceProvider {
    pub files: Vec<(SourceHeader, String)>,
}

impl SourceProvider for InMemorySourceProvider {
    fn file_list(&self) -> Vec<SourceHeader> {
        self.files
            .iter()
            .map(|(h, _)| h.clone())
            .collect::<Vec<_>>()
    }

    fn source(&self, filename: &str) -> String {
        self.files
            .iter()
            .find(|(h, c)| h.filename == filename)
            .map(|(_, c)| c.clone())
            .unwrap()
    }
}
