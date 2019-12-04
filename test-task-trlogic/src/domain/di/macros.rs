#[macro_export]
macro_rules! service_ref {
    ($service:expr => &dyn $trait_reference: path) => {
        {
            let service = $service.get_rc();
            let trait_reference = service.as_ref() as &dyn $trait_reference;
            let trait_reference: ::std::raw::TraitObject = unsafe {
                ::std::mem::transmute(trait_reference)
            };
            let trait_reference: &dyn $trait_reference = unsafe {
                ::std::mem::transmute(trait_reference)
            };
            crate::domain::di::service_ref::ServiceRef::new(service, trait_reference)
        }
    };
}