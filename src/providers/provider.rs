

pub trait Provider<C, V, P> {
    fn get(&self, container: &C, params: &P) -> V;
}
