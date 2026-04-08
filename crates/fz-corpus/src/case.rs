#[derive(Debug, Clone)]
pub struct CaseInput {
    pub data: Vec<u8>,
    pub description: String,
}

impl CaseInput {
    pub fn new(data: Vec<u8>, description: impl Into<String>) -> Self {
        Self {
            data,
            description: description.into(),
        }
    }
}
