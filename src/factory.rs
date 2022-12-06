

pub trait InstanceFactory<ContainerType, ValueType> {
    /// Create new instance of service
    fn new_instance(&self, container: &ContainerType) -> ValueType;
}
