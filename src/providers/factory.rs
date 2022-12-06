use crate::{InstanceFactory, Provider};

pub struct Factory<ContainerType, ValueType, ParamType> {

    factory: Box<dyn InstanceFactory<ContainerType, ValueType, ParamType>>
}

impl<ContainerType, ValueType, ParamType> Factory<ContainerType, ValueType, ParamType> {
    pub fn new(factory: Box<dyn InstanceFactory<ContainerType, ValueType, ParamType>>) -> Self {
        Factory{factory}
    }
}


impl<ContainerType, ValueType, ParamType> Provider<ContainerType, ValueType, ParamType> for Factory<ContainerType, ValueType, ParamType> {
    fn get(&self, container: &ContainerType, params: &ParamType) -> ValueType {
        self.factory.new_instance(container, params)
    }
}
