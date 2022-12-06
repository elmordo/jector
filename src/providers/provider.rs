

pub trait Provider<ContainerType, ValueType, ParamsType> {
    fn get(&self, container: &ContainerType, params: &ParamsType) -> ValueType;
}
