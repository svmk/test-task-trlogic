use std::collections::HashMap;

#[derive(Debug)]
pub struct HtmlResponse {
    params: HashMap<String, String>,
    content: String,
}

impl HtmlResponse {
    pub fn new(content: impl Into<String>) -> HtmlResponse {
        return HtmlResponse {
            params: HashMap::new(),
            content: content.into(),
        }
    }

    pub fn get_params(&self) -> &HashMap<String, String> {
        return &self.params;
    }

    pub fn get_content(&self) -> &String {
        return &self.content;
    }
}

