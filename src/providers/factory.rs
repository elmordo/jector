use crate::{InstanceFactory};
use crate::providers::Provider;


/// Create unique value for each customer
///
/// For generic argument, see the Provider trait
pub struct Factory<C, V, P> {

    /// The instance factory
    factory: Box<dyn InstanceFactory<C, V, P>>
}

impl<C, V, P> Factory<C, V, P> {
    /// Make new instance of the Factory struct
    pub fn new(factory: Box<dyn InstanceFactory<C, V, P>>) -> Self {
        Factory{factory}
    }
}


impl<C, V, P> Provider<C, V, P> for Factory<C, V, P> {
    /// Each call create unique instance of the value
    fn get(&self, container: &C, params: &P) -> V {
        self.factory.new_instance(container, params)
    }
}
