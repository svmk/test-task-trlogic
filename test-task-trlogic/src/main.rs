#![feature(raw)]
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate warp;
#[macro_use]
extern crate async_trait;
mod application;
mod domain;
mod presentation;
mod infrastructure;
mod system;

use once_cell::sync::OnceCell;
use structopt::StructOpt;
use crate::infrastructure::service::application_config_reading::read_application_config;
use crate::presentation::console::model::application_console_arguments::ApplicationConsoleArguments;
use crate::presentation::web::web_application::WebApplication;
use crate::system::app_container::AppContainer;
use crate::system::web_app_container::WebAppContainer;
use crate::system::runtime::Runtime;

fn main() {
    let console_params = ApplicationConsoleArguments::from_args();
    let config = read_application_config(console_params.get_config_path())
        .expect("Unable to read application config");
    let application_container = AppContainer::new(config)
        .expect("Unable to create application container");
    let web_application_container = WebAppContainer::from_application(&application_container)
        .expect("Unable to create web application container");
    let routing_service = web_application_container
        .routing
        .get(&web_application_container)
        .expect("Unable to initialize routing service");
    let web_application = WebApplication::new(
        web_application_container.app_config.get_web_server().clone(),
        routing_service,
    );
    static WEB_APPLICATION: OnceCell<WebApplication> = OnceCell::new();
    let _ = WEB_APPLICATION.set(web_application);
    let mut runtime = Runtime::new().expect("Unable to initialize runtime");
    WEB_APPLICATION.get().unwrap().start(&mut runtime);
    runtime.execute().expect("Error while executing runtime");
}
