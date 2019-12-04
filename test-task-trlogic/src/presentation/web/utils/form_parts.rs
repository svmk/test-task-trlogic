use failure::{Error, err_msg};
use warp::filters::multipart::Part as WarpFormPart;
use std::str::FromStr;
use crate::domain::model::temp_file_path::TempFilePath;
use crate::domain::model::mime_type::MimeType;
use futures_old::{Future, Stream};
use futures_old::future::ok as future_ok;
use futures_old::future::err as future_err;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub struct FormPart {
    name: String,
    content_type: Option<MimeType>,
    content: FormPartContent,
}

impl FormPart {
    pub fn from_warp_form_part(form_part: WarpFormPart) -> Box<dyn Future<Item=FormPart, Error=Error> + 'static + Send> {
        let name = form_part.name().to_string();
        let filename = form_part.filename().map(String::from);
        let content_type = form_part
            .content_type()
            .map(MimeType::from_str)
            .transpose();
        let content_type = match content_type {
            Ok(content_type) => content_type,
            Err(error) => {
                return Box::new(future_err(error));
            },
        };
        if let Some(filename) = filename {
            let tempfile = match TempFilePath::new() {
                Ok(tempfile) => tempfile,
                Err(error) => {
                    return Box::new(future_err(error));
                },
            };
            let file = File::create(tempfile.get_path()).map_err(err_msg);
            let mut file = match file {
                Ok(file) => file,
                Err(error) => {
                    return Box::new(future_err(error));
                },
            };
            let future = form_part
            .map_err(err_msg)
            .for_each(move |data| {
                let result = file.write_all(&data).map_err(err_msg);
                match result {
                    Ok(_) => {
                        return future_ok(());
                    },
                    Err(error) => {
                        return future_err(error);
                    }
                }
            })
            .and_then(move |_| {
                let result = FormPart::new_file(name, content_type, filename, tempfile);
                return future_ok(result);
            });
            return Box::new(future);
        } else {
            let future = form_part
                .concat2()
                .map_err(err_msg)
                .and_then(|content| {
                let content = String::from_utf8(content).map_err(err_msg);
                match content {
                    Ok(content) => {
                        let result = FormPart::new_text(name, content_type, content);
                        return future_ok(result);
                    },
                    Err(error) => {
                        return future_err(error);
                    },
                }
            });
            return Box::new(future);
        }
    }

    fn new_file(name: String, content_type: Option<MimeType>, filename: String, content: TempFilePath) -> FormPart {
        let content = FormPartFileContent {
            filename,
            content,
        };
        return FormPart {
            name,
            content_type,
            content: FormPartContent::File(content),
        };
    }

    fn new_text(name: String, content_type: Option<MimeType>, content: String) -> FormPart {
        let content = FormPartTextContent {
            content,
        };
        return FormPart {
            name,
            content_type,
            content: FormPartContent::Text(content),
        };
    }
}

#[derive(Debug)]
enum FormPartContent {
    File(FormPartFileContent),
    Text(FormPartTextContent)
}

#[derive(Debug)]
pub struct FormPartFileContent {
    filename: String,
    content: TempFilePath,
}

impl FormPartFileContent {
    pub fn get_filename(&self) -> &String {
        return &self.filename;
    }

    pub fn into_temp_file_path(self) -> TempFilePath {
        return self.content;
    }
}

#[derive(Debug)]
pub struct FormPartTextContent {
    content: String,
}

impl FormPartTextContent {
    pub fn get_content(&self) -> &String {
        return &self.content;
    }
}

impl FormPart {
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_content_type(&self) -> Option<&MimeType> {
        return self.content_type.as_ref();
    }

    pub fn into_text_content(self) -> Result<FormPartTextContent, Error> {
        match self.content {
            FormPartContent::Text(text) => {
                return Ok(text);
            },
            _ => {
                return Err(err_msg(format!("Requested field {} is not text", self.name)));
            },
        }
    }

    pub fn into_file_content(self) -> Result<FormPartFileContent, Error> {
        match self.content {
            FormPartContent::File(file) => {
                return Ok(file);
            },
            _ => {
                return Err(err_msg(format!("Requested field {} is not file", self.name)));
            },
        }
    }
}