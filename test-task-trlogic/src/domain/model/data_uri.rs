use crate::domain::model::mime_type::MimeType;

pub struct DataUri {
    mime_type: MimeType,
    data: Vec<u8>,
}

impl DataUri {
    pub fn new(mime_type: MimeType, data: Vec<u8>) -> DataUri {
        DataUri {
            mime_type,
            data,
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        return &self.data;
    }

    pub fn get_mime_type(&self) -> &MimeType {
        return &self.mime_type;
    }
}