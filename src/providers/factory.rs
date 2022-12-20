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

    /// Create new instance wrapped in the Box
    pub fn boxed(factory: Box<dyn InstanceFactory<C, V, P>>) -> Box<Self> {
        Box::new(Self::new(factory))
    }
}


impl<C, V, P> Provider<C, V, P> for Factory<C, V, P> {
    /// Each call create unique instance of the value
    fn get(&self, container: &C, params: &P) -> V {
        self.factory.new_instance(container, params)
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::InstanceFactory;
    use crate::providers::{Factory, Provider};

    #[derive(Default)]
    struct MyInstanceFactory {
        val: RefCell<i32>
    }

    impl InstanceFactory<(), i32, ()> for MyInstanceFactory {
        /// The first call returns 1, the second returns 2, ...
        fn new_instance(&self, _: &(), _: &()) -> i32 {
            let mut ref_val = self.val.borrow_mut();
            *ref_val += 1;
            *ref_val
        }
    }

    #[test]
    fn each_call_create_unique_instance() {
        let provider = Factory::new(Box::new(MyInstanceFactory::default()));
        let val_1 = provider.get(&(), &());
        let val_2 = provider.get(&(), &());

        assert_eq!(val_1, 1);
        assert_eq!(val_2, 2);
    }
}
