use crate::types::InstanceManager;

pub struct Value<T> {
    value: T
}


impl<T> Value<T> {
    pub fn new(value: T) -> Self {
        Self{value}
    }
}


impl<T> InstanceManager<&T> for Value<T> {
    fn get_instance(&self, args: &()) -> &T {
        &self.value
    }
}
