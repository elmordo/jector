use crate::providers::Provider;


/// Stack proxy for normal providers
/// Provider encapsulated in this proxy can be overridden by another provider.
/// Overridden provider can be reverted to original one
#[derive(Default)]
pub struct ProviderStack<C, V, P> {
    provider_stack: Vec<Box<dyn Provider<C, V, P>>>
}


impl<C, V, P> ProviderStack<C, V, P> {

    /// Create new instance with initial provider set on top of the stack
    pub fn new(provider: Box<dyn Provider<C, V, P>>) -> ProviderStack<C, V, P> {
        ProviderStack{provider_stack: vec![provider]}
    }

    /// Push new provider on the stack
    pub fn push(&mut self, provider: Box<dyn Provider<C, V, P>>) {
        self.provider_stack.push(provider);
    }

    /// Pop current provider from the stack and set new provider
    pub fn pop(&mut self) -> Option<Box<dyn Provider<C, V, P>>> {
        self.provider_stack.pop()
    }

    /// Return size of the stack
    pub fn len(&self) -> usize {
        self.provider_stack.len()
    }

    /// Return true if stack is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.provider_stack.is_empty()
    }
}



impl<C, V, P> Provider<C, V, P> for ProviderStack<C, V, P> {
    /// Get value from top level provider.
    ///
    /// Panic, if there is no provider on the stack
    fn get(&self, container: &C, params: &P) -> V {
        if self.provider_stack.is_empty() {
            panic!("There is no provider on the stack")
        }
        let p = self.provider_stack.last().unwrap();
        p.get(container, params)
    }
}



#[cfg(test)]
mod tests {
    use crate::providers::Provider;
    use crate::providers::provider_stack::ProviderStack;
    type MyProviderStack = ProviderStack<(), i32, ()>;

    struct MyProvider {
        val: i32
    }

    impl MyProvider {
        fn new(val: i32) -> MyProvider {
            MyProvider{val}
        }
    }

    impl Provider<(), i32, ()> for MyProvider {
        fn get(&self, _: &(), _: &()) -> i32 {
            self.val
        }
    }

    #[test]
    fn default_is_empty() {
        let provider: MyProviderStack = ProviderStack::default();
        assert!(provider.is_empty());
        assert_eq!(provider.len(), 0);
    }

    #[test]
    #[should_panic]
    fn empty_instance_panic_on_get() {
        let provider: MyProviderStack = ProviderStack::default();
        provider.get(&(), &());
    }

    #[test]
    fn instance_with_initial_provider_is_not_empty() {
        let instance = ProviderStack::new(Box::new(MyProvider::new(42)));
        assert!(!instance.is_empty());
        assert_eq!(instance.len(), 1);
    }

    #[test]
    fn instance_with_initial_provider_return_value() {
        let instance = ProviderStack::new(Box::new(MyProvider::new(42)));
        assert_eq!(instance.get(&(), &()), 42);
    }

    #[test]
    fn provider_can_be_overridden() {
        let mut instance = ProviderStack::new(Box::new(MyProvider::new(42)));
        instance.push(Box::new(MyProvider::new(666)));
        assert_eq!(instance.get(&(), &()), 666);
    }

    #[test]
    fn override_can_be_reverted() {
        let mut instance = ProviderStack::new(Box::new(MyProvider::new(42)));
        instance.push(Box::new(MyProvider::new(666)));
        instance.pop();
        assert_eq!(instance.get(&(), &()), 42);
    }

}
