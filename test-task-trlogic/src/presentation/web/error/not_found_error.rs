#[derive(Debug)]
pub struct NotFoundError {
    reason: String,
}

impl NotFoundError {
//    pub fn new(reason: impl Into<String>) -> NotFoundError {
//        return NotFoundError {
//            reason: reason.into(),
//        }
//    }

    pub fn get_reason(&self) -> &str {
        return &self.reason;
    }
}
