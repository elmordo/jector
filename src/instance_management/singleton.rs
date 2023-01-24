use std::cell::RefCell;
use std::sync::Arc;
use crate::{InstanceManager};


/// Always deliver same instance wrapped in the `Arc`
///
/// The instance is created by a factory callback on call of the `get_instance()` method.
/// Each other call of the `get_instance()` returns the instance created by the first call.
///
/// # Typical use cases
///
/// * database adapters
/// * loggers
///
/// # Restrictions
///
/// Only unit value is passed as argument because any argument is pointless for this instance
/// manager
pub struct Singleton<T> {
    /// The stored instance (or None if instance was not created yet)
    instance: Arc<RefCell<Option<Arc<T>>>>,

    /// The instance's factory fn
    factory: Box<dyn Fn(()) -> T>,
}


impl<T> Singleton<T> {
    /// Create new `Singleton` instance manager
    pub fn new(factory: Box<dyn Fn(()) -> T>) -> Self {
        Self{instance: Arc::new(RefCell::new(None)), factory}
    }

    /// Return true if no instance was created yet, false otherwise
    pub fn is_empty(&self) -> bool {
        self.instance.borrow().is_none()
    }

    /// Clear stored instance
    pub fn clear(&mut self) {
        *self.instance.borrow_mut() = None;
    }

    /// Create new instance if there is no one. Do nothing otherwise
    fn prepare_instance(&self) {
        if self.is_empty() {
            let mut instance = self.instance.borrow_mut();
            *instance = Some(self.make_instance())
        }
    }

    /// Call the factory method and return the instance wrapped in the `Arc`
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
    use std::cell::RefCell;
    use crate::instance_management::singleton::Singleton;
    use crate::InstanceManager;

    fn make_singleton() -> Singleton<i32> {
        let x = RefCell::new(0);
        Singleton::new(Box::new(move |_| {
            *x.borrow_mut() += 1;
            x.borrow().clone()
        }))
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
        assert_ne!(v1, v2);
    }
}
