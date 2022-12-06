use crate::{InstanceFactory, Provider};

pub struct Factory<C, V, P> {

    factory: Box<dyn InstanceFactory<C, V, P>>
}

impl<C, V, P> Factory<C, V, P> {
    pub fn new(factory: Box<dyn InstanceFactory<C, V, P>>) -> Self {
        Factory{factory}
    }
}


impl<C, V, P> Provider<C, V, P> for Factory<C, V, P> {
    fn get(&self, container: &C, params: &P) -> V {
        self.factory.new_instance(container, params)
    }
}
