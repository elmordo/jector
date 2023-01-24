use crate::InstanceManager;

/// Helper instance manager capable to stack other manager and use the top one
struct InstanceManagerStack<T, Args> {
    /// The stack of the stored managers
    stack: Vec<Box<dyn InstanceManager<T, Args>>>
}


impl<T, Args> InstanceManagerStack<T, Args> {
    pub fn new(manager: Box<dyn InstanceManager<T, Args>>) -> Self {
        let mut instance = Self::empty();
        instance.push(manager);
        instance
    }

    pub fn empty() -> Self {
        Self{stack: Vec::new()}
    }

    pub fn push(&mut self, manager: Box<dyn InstanceManager<T, Args>>) {
        self.stack.push(manager)
    }

    pub fn pop(&mut self) -> Option<Box<dyn InstanceManager<T, Args>>> {
        self.stack.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}


impl<T, Args> InstanceManager<T, Args> for InstanceManagerStack<T, Args> {
    fn get_instance(&self, args: &Args) -> T {
        if let Some(m) = self.stack.last() {
            m.get_instance(args)
        } else {
            panic!("No manager on the stack");
        }
    }
}
