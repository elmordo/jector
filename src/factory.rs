

pub trait Factory<T, S> {
    /// Create new instance of service
    fn new_instance(&mut self, container: &T) -> S;
}
