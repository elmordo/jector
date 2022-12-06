

pub trait Provider<ContainerType, ValueType> {
    fn get(&self, container: &ContainerType) -> &ValueType;

    fn get_mut(&mut self, container: &ContainerType) -> &mut ValueType;
}
