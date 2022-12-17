use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::InstanceFactory;
use crate::providers::Provider;


/// Clone of single value is always returned.
pub struct Singleton<C, V, P> where V: Clone {
    /// The instance factory
    factory: Box<dyn InstanceFactory<C, V, P>>,

    /// Stored instance (Some) or None if instance was not created yet.
    value: RefCell<Option<Rc<V>>>,
}


impl<C, V, P> Singleton<C, V, P> where V: Clone {
    pub fn new(factory: Box<dyn InstanceFactory<C, V, P>>) -> Self {
        Singleton { factory, value: RefCell::new(None) }
    }

    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_none()
    }

    fn prepare_value(&self, container: &C, params: &P) {
        if self.is_empty() {
            self.set_value(self.create_instance(container, params));
        }
    }

    fn set_value(&self, value: V) {
        self.value.replace(Some(Rc::new(value)));
    }

    fn get_value(&self) -> V {
        // When value is not set (should never happened) -> panic!
        let binding = self.value.borrow();
        let val = binding.as_ref().unwrap();
        val.deref().clone()
    }

    fn create_instance(&self, container: &C, params: &P) -> V {
        self.factory.new_instance(container, params)
    }
}


impl<C, V, P> Provider<C, V, P> for Singleton<C, V, P> where V: Clone {
    fn get(&self, container: &C, params: &P) -> V {
        self.prepare_value(container, params);
        self.get_value()
    }
}
