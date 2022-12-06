

pub trait Provider<ContainerType, ValueType> {
    fn get(&self, container: &ContainerType) -> ValueType;
}
