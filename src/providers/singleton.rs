use std::cell::RefCell;
use std::ops::Deref;
use crate::Factory;
use crate::providers::provider::Provider;

pub struct Singleton<ContainerType, ValueType> {
    factory: Box<dyn Factory<ContainerType, ValueType>>,
    value: RefCell<Option<ValueType>>,
}


impl<ContainerType, ValueType> Singleton<ContainerType, ValueType> {
    pub fn new(factory: Box<dyn Factory<ContainerType, ValueType>>) -> Singleton<ContainerType, ValueType> {
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
        self.value.replace(Some(value));
    }

    fn get_value(&self) -> &ValueType {
        // When value is not set (should never happened) -> panic!
        let x = self.value.borrow();
        todo!()
    }

    fn get_value_mut(&self) -> &mut ValueType {
        // When value is not set (should never happened) -> panic!
        todo!()
    }

    fn create_instance(&self, container: &ContainerType) -> ValueType {
        self.factory.new_instance(container)
    }
}


impl<ContainerType, ValueType> Provider<ContainerType, ValueType> for Singleton<ContainerType, ValueType> {
    fn get(&self, container: &ContainerType) -> &ValueType {
        self.prepare_value(container);
        self.get_value()
    }

    fn get_mut(&mut self, container: &ContainerType) -> &mut ValueType {
        self.prepare_value(container);
        self.get_value_mut()
    }
}
