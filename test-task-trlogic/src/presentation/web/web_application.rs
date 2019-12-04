use crate::presentation::web::routing::Routing;
use crate::domain::di::service::Service;
use crate::domain::model::application_config::WebServer;
use crate::system::runtime::Runtime;
use warp::serve;

#[derive(new)]
pub struct WebApplication {
    web_server_config: WebServer,
    routing: Service<Routing>,
}

impl WebApplication {
    pub fn start(&'static self, runtime: &mut Runtime) {
        let routing = self.routing.create_routing();
        let server_address = self.web_server_config.get_bind_address().clone();
        let (_, future) = serve(routing).bind_ephemeral(server_address);
        runtime.spawn(future);
    }
}