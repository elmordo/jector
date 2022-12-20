use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::InstanceFactory;
use crate::providers::Provider;


/// Clone of single value is always returned.
/// The value is usually shared (Rc) so each customer gets same data.
///
/// For generic argument, see the Provider trait
pub struct Singleton<C, V, P> where V: Clone {
    /// The instance factory
    factory: Box<dyn InstanceFactory<C, V, P>>,

    /// Stored instance (Some) or None if instance was not created yet.
    value: RefCell<Option<Rc<V>>>,
}


impl<C, V, P> Singleton<C, V, P> where V: Clone {
    /// Create new singleton provider instance
    pub fn new(factory: Box<dyn InstanceFactory<C, V, P>>) -> Self {
        Singleton { factory, value: RefCell::new(None) }
    }

    /// Create new instance wrapped in the Box
    pub fn boxed(factory: Box<dyn InstanceFactory<C, V, P>>) -> Box<Self> {
        Box::new(Self::new(factory))
    }

    /// Return true if value was not created yet
    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_none()
    }

    /// If value is empty, create new value
    fn prepare_value(&self, container: &C, params: &P) {
        if self.is_empty() {
            self.set_value(self.create_instance(container, params));
        }
    }

    /// Set new value
    fn set_value(&self, value: V) {
        self.value.replace(Some(Rc::new(value)));
    }

    /// Get value from the container
    fn get_value(&self) -> V {
        // When value is not set (should never happened) -> panic!
        let binding = self.value.borrow();
        let val = binding.as_ref().unwrap();
        val.deref().clone()
    }

    /// Create new instance of the value
    fn create_instance(&self, container: &C, params: &P) -> V {
        self.factory.new_instance(container, params)
    }
}


impl<C, V, P> Provider<C, V, P> for Singleton<C, V, P> where V: Clone {
    /// Each call clone prepared value.
    /// If value is not prepared, create new value, store it in the Singleton instance and clone it.
    fn get(&self, container: &C, params: &P) -> V {
        self.prepare_value(container, params);
        self.get_value()
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::InstanceFactory;
    use crate::providers::{Provider, Singleton};

    #[derive(Default)]
    struct MyInstanceFactory {
        val: RefCell<i32>
    }

    impl InstanceFactory<(), i32, ()> for MyInstanceFactory {
        /// The first call returns 1, the second returns 2, ...
        fn new_instance(&self, _: &(), _: &()) -> i32 {
            let mut ref_val = self.val.borrow_mut();
            *ref_val += 1;
            *ref_val
        }
    }

    #[test]
    fn every_call_return_same_value() {
        let provider = Singleton::new(Box::new(MyInstanceFactory::default()));
        let val_1 = provider.get(&(), &());
        let val_2 = provider.get(&(), &());

        assert_eq!(val_1, 1);
        assert_eq!(val_2, 1);
    }
}
