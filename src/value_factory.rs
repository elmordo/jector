

/// Responsible for value instance creation
pub trait ValueFactory<C, V, P> {
    /// Create new instance of service
    fn new_instance(&self, container: &C, params: &P) -> V;
}
