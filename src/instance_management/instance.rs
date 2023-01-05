use std::sync::Arc;

use crate::types::InstanceManager;

pub struct Instance<T> {
    value: Arc<T>,
}


impl<T> Instance<T> {
    pub fn new(value: T) -> Self {
        Self { value: Arc::new(value) }
    }
}


impl<T> InstanceManager<Arc<T>> for Instance<T> {
    fn get_instance(&self, _: &()) -> Arc<T> {
        self.value.clone()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_hovno() {}
}
