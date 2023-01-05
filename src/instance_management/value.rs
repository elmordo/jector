use crate::types::InstanceManager;

pub struct Value<T> {
    value: T
}


impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self{value}
    }
}


impl<'self_, 'args, T> InstanceManager<'self_, 'args, &'self_ T> for Value<T> {
    fn get_instance(&self, _: &()) -> &T {
        &self.value
    }
}
