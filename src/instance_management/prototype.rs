use std::ops::Deref;
use std::sync::Arc;
use crate::InstanceManager;

/// Always returns clone of initial instance
///
/// Similar to the `Instance` instance manager but does not returns reference to the original
/// but its clone.
pub struct Prototype<T> where T: Clone {
    /// Stored instance
    instance: Arc<T>
}

impl<T> Prototype<T> where T: Clone {
    /// Create new instance of the `Prototype` instance manager
    pub fn new(instance: T) -> Prototype<T> {
        Prototype{instance: Arc::new(instance)}
    }
}


impl<T> InstanceManager<T, ()> for Prototype<T> where T: Clone {
    fn get_instance(&self, _: &()) -> T {
        self.instance.deref().clone()
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use crate::instance_management::prototype::Prototype;
    use crate::InstanceManager;

    struct MyStruct {
        val: i32,
        offset: RefCell<i32>
    }

    impl Clone for MyStruct {
        fn clone(&self) -> Self {
            *self.offset.borrow_mut() += 1;
            MyStruct{val: self.val + *self.offset.borrow(), offset: RefCell::new(0)}
        }
    }

    fn make_prototype() -> Prototype<MyStruct> {
        Prototype::new(MyStruct{val: 42, offset: RefCell::new(0)})
    }

    #[test]
    fn copy_value() {
        let p = make_prototype();
        let v1 = p.get_instance(&());
        let v2 = p.get_instance(&());

        assert_ne!(v1.val, v2.val);
    }
}
