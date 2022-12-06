use crate::{InstanceFactory, Provider};

pub struct Factory<ContainerType, ValueType> {
    factory: Box<dyn InstanceFactory<ContainerType, ValueType>>
}

impl<ContainerType, ValueType> Factory<ContainerType, ValueType> {
    pub fn new(factory: Box<dyn InstanceFactory<ContainerType, ValueType>>) -> Self {
        Factory{factory}
    }
}


impl<ContainerType, ValueType> Provider<ContainerType, ValueType> for Factory<ContainerType, ValueType> {
    fn get(&self, container: &ContainerType) -> ValueType {
        self.factory.new_instance(container)
    }
}
