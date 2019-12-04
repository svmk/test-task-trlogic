use std::sync::Arc;
use std::ops::Deref;
use std::any::Any;
use std::marker::{Sync, Send};
use std::clone::Clone;

pub struct ServiceRef<T>(Arc<ServiceRefInner<T>>);

struct ServiceRefInner<T> {
    // Used for calling destructor.
    _service: Box<dyn Any + Sync>,
    reference: T,
}

impl <T>ServiceRef<T> {
    pub fn new<S>(service: Arc<S>, reference: T) -> ServiceRef<T>
        where S: Sync,
              S: Send,
              S: 'static
    {
        let service: Box<dyn Any + Sync> = Box::new(service);
        let service_ref = ServiceRefInner {
            _service: service,
            reference,
        };
        let service_ref = ServiceRef(Arc::new(service_ref));
        return service_ref;
    }
}

impl <T>Deref for ServiceRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0.reference;
    }
}

/// TODO: Implement drop in separated thread.
unsafe impl <T>Sync for ServiceRef<T> {}
unsafe impl <T>Send for ServiceRef<T> {}

impl <T>Clone for ServiceRef<T> {
    fn clone(&self) -> Self {
        return ServiceRef(self.0.clone());
    }
}