#[derive(Debug, Clone)]
pub struct Unknown {
    pub command_name: String,
}

impl Unknown {
    pub fn new(key: impl Into<String>) -> Unknown {
        Unknown {
            command_name: key.into(),
        }
    }
}
