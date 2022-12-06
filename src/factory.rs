

pub trait InstanceFactory<ContainerType, ValueType, ParamsType> {
    /// Create new instance of service
    fn new_instance(&self, container: &ContainerType, params: &ParamsType) -> ValueType;
}
