/// Trait for all providers
///
/// The generic params are:
///
/// * C - Container type.
/// * V - Provided value type.
/// * P - Parameters type.
pub trait Provider<C, V, P> {
    /// Get the value
    ///
    /// Params are used for extra definition for the provider
    fn get(&self, container: &C, params: &P) -> V;
}
