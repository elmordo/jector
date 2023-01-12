use crate::InstanceManager;

pub struct Factory<T, Args> {
    instance_factory: Box<dyn Fn(&Args) -> T>,
}


impl<T, Args> Factory<T, Args> {
    pub fn new(instance_factory: Box<dyn Fn(&Args) -> T>) -> Self {
        Factory{instance_factory}
    }
}


impl<T, Args> InstanceManager<T, Args> for Factory<T, Args> {
    fn get_instance(&self, args: &Args) -> T {
        (self.instance_factory)(args)
    }
}
