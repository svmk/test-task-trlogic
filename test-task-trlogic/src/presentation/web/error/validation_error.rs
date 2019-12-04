#[derive(Debug)]
pub struct ValidationError {
    reason: String,
}

impl ValidationError {
    pub fn new(reason: impl Into<String>) -> ValidationError {
        return ValidationError {
            reason: reason.into(),
        }
    }

    pub fn get_reason(&self) -> &str {
        return &self.reason;
    }
}
