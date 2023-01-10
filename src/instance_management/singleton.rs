use std::cell::RefCell;
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

    pub fn is_empty(&self) -> bool {
        self.instance.borrow().is_none()
    }

    pub fn clear(&mut self) {
        *self.instance.borrow_mut() = None;
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
    fn get_instance(&self, _: &()) -> Arc<T> {
        self.prepare_instance();
        self.instance.as_ref().borrow().as_ref().unwrap().clone()
    }
}


#[cfg(test)]
mod tests {
    use std::ptr::addr_of;
    use crate::instance_management::singleton::Singleton;
    use crate::InstanceManager;

    fn make_singleton() -> Singleton<()> {
        Singleton::new(Box::new(move |_| ()))
    }

    #[test]
    fn is_empty_on_init() {
        let s = make_singleton();
        assert!(s.is_empty())
    }

    #[test]
    fn is_not_empty_after_get_instance() {
        let s = make_singleton();
        s.get_instance(&());
        assert!(!s.is_empty());
    }

    #[test]
    fn return_same_instances() {
        let s = make_singleton();
        let v1 = s.get_instance(&());
        let v2 = s.get_instance(&());

        assert_eq!(*v1, *v2);
    }

    #[test]
    fn clear_remove_value() {
        let mut s = make_singleton();
        s.clear();
        assert!(s.is_empty());
    }

    #[test]
    fn new_instance_created_after_clear() {
        let mut s = make_singleton();
        let v1 = s.get_instance(&());
        s.clear();
        let v2 = s.get_instance(&());
        assert_ne!(addr_of!(*v1), addr_of!(*v2));
    }
}
