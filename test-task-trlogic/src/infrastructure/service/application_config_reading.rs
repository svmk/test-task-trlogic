use crate::domain::model::application_config::ApplicationConfig;
use failure::{Error, err_msg};
use std::path::Path;
use std::fs::File;

pub fn read_application_config(file: &Path) -> Result<ApplicationConfig, Error> {
    let file = File::open(file).map_err(err_msg)?;
    let config: ApplicationConfig = serde_yaml::from_reader(file).map_err(err_msg)?;
    return Ok(config);
}