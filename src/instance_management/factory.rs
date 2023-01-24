use crate::InstanceManager;


/// Always create new instance using provided instance factory
pub struct Factory<T, Args> {
    /// The instance factory
    instance_factory: Box<dyn Fn(&Args) -> T>,
}


impl<T, Args> Factory<T, Args> {
    /// Create new `Factory` instance and initialize it with the instance factory
    pub fn new(instance_factory: Box<dyn Fn(&Args) -> T>) -> Self {
        Factory{instance_factory}
    }
}


impl<T, Args> InstanceManager<T, Args> for Factory<T, Args> {
    fn get_instance(&self, args: &Args) -> T {
        (self.instance_factory)(args)
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::ops::Deref;
    use crate::instance_management::factory::Factory;
    use crate::InstanceManager;

    fn make_factory() -> Factory<i32, (i32,)> {
        let counter = RefCell::new(0);
        Factory::new(Box::new(move |offset| {
            let base: i32 = counter.borrow().deref().clone();
            *counter.borrow_mut() = base + 1;
            base + offset.0
        }))
    }

    #[test]
    fn factory_work() {
        let factory = make_factory();
        let val_1 = factory.get_instance(&(0,));
        let val_2 = factory.get_instance(&(1,));

        assert_eq!(val_1, 0);
        assert_eq!(val_2, 2);
    }
}
