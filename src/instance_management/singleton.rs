use std::cell::RefCell;
use std::ops::Deref;
use std::sync::Arc;
use crate::{InstanceManager};


pub struct Singleton<T> {
    instance: Arc<RefCell<Option<Arc<T>>>>,

    factory: Box<dyn Fn(()) -> T>,
}


impl<T> Singleton<T> {
    pub fn new(factory: Box<dyn Fn(()) -> T>) -> Self {
        Self{instance: Arc::new(RefCell::new(None)), factory}
    }

    fn prepare_instance(&self) {
        let mut instance = self.instance.borrow_mut();
        if instance.is_none() {
            *instance = Some(self.make_instance())
        }
    }

    fn make_instance(&self) -> Arc<T> {
        Arc::new((self.factory)(()))
    }
}


impl<T> InstanceManager<Arc<T>> for Singleton<T> {
    fn get_instance(&self, args: &()) -> Arc<T> {
        self.prepare_instance();
        self.instance.as_ref().borrow().as_ref().unwrap().clone()
    }
}
