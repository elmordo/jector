use std::sync::Arc;
use crate::InstanceManager;

pub struct Prototype<T> where T: Clone {
    instance: Arc<T>
}

impl<T> Prototype<T> where T: Clone {
    pub fn new(instance: T) {
        Prototype::new(Arc::new(instance))
    }
}


impl<T> InstanceManager<T, ()> for Prototype<T> where T: Clone {
    fn get_instance(&self, args: &()) -> T {
        self.instance.clone()
    }
}
