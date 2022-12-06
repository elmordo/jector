use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use crate::InstanceFactory;
use crate::providers::provider::Provider;

pub struct Singleton<ContainerType, ValueType> where ValueType: Clone {
    factory: Box<dyn InstanceFactory<ContainerType, ValueType>>,
    value: RefCell<Option<Rc<ValueType>>>,
}


impl<ContainerType, ValueType> Singleton<ContainerType, ValueType> where ValueType: Clone {
    pub fn new(factory: Box<dyn InstanceFactory<ContainerType, ValueType>>) -> Self {
        Singleton { factory, value: RefCell::new(None) }
    }

    pub fn is_empty(&self) -> bool {
        self.value.borrow().is_none()
    }

    fn prepare_value(&self, container: &ContainerType) {
        if self.is_empty() {
            self.set_value(self.create_instance(container));
        }
    }

    fn set_value(&self, value: ValueType) {
        self.value.replace(Some(Rc::new(value)));
    }

    fn get_value(&self) -> ValueType {
        // When value is not set (should never happened) -> panic!
        let binding = self.value.borrow();
        let val = binding.as_ref().unwrap();
        val.deref().clone()
    }

    fn create_instance(&self, container: &ContainerType) -> ValueType {
        self.factory.new_instance(container)
    }
}


impl<ContainerType, ValueType> Provider<ContainerType, ValueType> for Singleton<ContainerType, ValueType> where ValueType: Clone {
    fn get(&self, container: &ContainerType) -> ValueType {
        self.prepare_value(container);
        self.get_value()
    }
}
