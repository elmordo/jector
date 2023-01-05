/// Function pointer used as factory for instances delivered by providers
pub type InstanceFactory<T, Args = ()> = dyn Fn(Args) -> T;


/// Common interface for instance managers
pub trait InstanceManager<T, Args = ()> {
    /// Get instance for the instance manager
    fn get_instance(&self, args: &Args) -> T;
}
