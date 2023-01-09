/// Common interface for instance managers
pub trait InstanceManager<T, Args = ()> {
    /// Get instance for the instance manager
    fn get_instance(&self, args: &Args) -> T;
}
