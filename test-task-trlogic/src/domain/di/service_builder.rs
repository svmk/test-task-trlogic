use crate::domain::di::service::Service;
use failure::Error;
use std::sync::{Arc, Mutex};
use std::clone::Clone;

pub struct ServiceBuilder<App, S>(Arc<ServiceBuilderInner<App, S>>);

struct ServiceBuilderInner<App, S> {
    factory: Box<dyn Fn(&App) -> Result<S, Error>>,
    service: Mutex<Option<Service<S>>>,
}

impl <App, S> ServiceBuilder<App, S> where S: 'static {
    pub fn new(factory: impl Fn(&App) -> Result<S, Error> + 'static) -> ServiceBuilder<App, S> {
        let inner = Arc::new(ServiceBuilderInner {
            factory: Box::new(factory),
            service: Mutex::new(None),
        });
        return ServiceBuilder(inner);
    }

    // TODO: Избавиться от application
    pub fn get(&self, application: &App) -> Result<Service<S>, Error> {
        let mut service_container = self
            .0
            .service
            .lock()
            .expect("Unable to lock mutes for service factory");
        if let Some(service) = &*service_container {
            return Ok(service.clone());
        } else {
            let service = (self.0.factory)(application)?;
            let service = Service::new(service);
            *service_container = Some(service.clone());
            return Ok(service);
        }
    }
}

impl <App, S>Clone for ServiceBuilder<App, S> {
    fn clone(&self) -> Self {
        return ServiceBuilder(self.0.clone());
    }
}