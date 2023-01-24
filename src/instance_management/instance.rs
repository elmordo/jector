use std::sync::Arc;

use crate::types::InstanceManager;


/// Always deliver same instance wrapped in the `Arc`
///
/// The instance is passed directly to the `new` method
/// The instance is not created "on demand" and this is the difference against the Singleton.
///
/// # Typical use cases
///
/// * constants
/// * configuration values
///
/// # Restrictions
///
/// Only unit value is passed as argument because any argument is pointless for this instance
/// manager
pub struct Instance<T> {
    /// Stored instance of T wrapped in the Arc
    instance: Arc<T>,
}


impl<T> Instance<T> {
    /// Create new instance of the `Instance` with given value
    pub fn new(instance: T) -> Self {
        Self { instance: Arc::new(instance) }
    }
}


impl<T> InstanceManager<Arc<T>> for Instance<T> {
    fn get_instance(&self, _: &()) -> Arc<T> {
        self.instance.clone()
    }
}


#[cfg(test)]
mod tests {
    use crate::instance_management::instance::Instance;
    use crate::InstanceManager;

    #[test]
    fn test_initialization() {
        let instance = Instance::new(42);
        assert_eq!(*instance.get_instance(&()), 42);
    }
}
