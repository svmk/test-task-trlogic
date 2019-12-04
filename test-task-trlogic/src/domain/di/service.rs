use std::sync::{Arc};
use std::ops::Deref;
use std::borrow::Borrow;
use std::clone::Clone;

#[derive(Debug)]
pub struct Service<T> {
    service: Arc<T>,
}

impl <T>Service<T> where T: 'static {
    pub fn new(service: T) -> Service<T> {
        return Service {
            service: Arc::new(service),
        }
    }

    pub fn get_rc(&self) -> Arc<T> {
        return self.service.clone();
    }

//    pub fn as_ref<F>(&self, factory: impl FnOnce(&T) -> F) -> ServiceRef<F> where F: Any {
//        let service_ref = factory(self.service.as_ref());
//        let service: Box<dyn Any> = Box::new(self.service.clone());
//        return ServiceRef::new(service, Box::new(service_ref));
//    }
}

impl <T>Deref for Service<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return self.service.as_ref();
    }
}

impl <T>Borrow<T> for Service<T> {
    fn borrow(&self) -> &T {
        return self.deref();
    }
}

impl <T>Clone for Service<T> {
    fn clone(&self) -> Self {
        return Service {
            service: self.service.clone(),
        }
    }
}