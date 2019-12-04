use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name = "test-task-trlogic", about = "An test task for TrLogic.")]
pub struct ApplicationConsoleArguments {
    #[structopt(parse(from_os_str))]
    config: PathBuf,
}

impl ApplicationConsoleArguments {
    pub fn get_config_path(&self) -> &PathBuf {
        return &self.config;
    }
}