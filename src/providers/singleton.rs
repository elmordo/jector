use crate::Factory;
use crate::providers::provider::Provider;

pub struct Singleton<ContainerType, ValueType> {
    factory: Box<dyn Factory<ContainerType, ValueType>>,
    value: Option<ValueType>
}


impl<ContainerType, ValueType> Singleton<ContainerType, ValueType> {
    pub fn new(factory: Box<dyn Factory<ContainerType, ValueType>>) -> Singleton<ContainerType, ValueType> {
        Singleton{factory, value: None}
    }

    fn create_instance(&self, container: &ContainerType) -> ValueType {
        self.factory.new_instance(container)
    }
}


impl<ContainerType, ValueType> Provider<ContainerType, ValueType> for Singleton<ContainerType, ValueType> {
    fn get(&self, container: &ContainerType) -> ValueType {
        todo!()
    }
}
