use std::cell::RefCell;
use jector::{Factory, InstanceFactory, Provider, ProviderStack};
use jector::providers::Singleton;

fn main() {
    let mut container = Container::new();

    println!("The constant value is: {}", container.get_constant());  // value is 42

    for _i in 0..5 {
        println!("Next value of simple sequence is: {}", container.get_next_value());  // value is 42 + _i
    }

    for i in 0..5 {
        println!("Next value of sequence with param (param is {}) is: {}", i, container.get_next_parametrized_value(i));  // value is 42 + _i
    }

    println!("Value from provider stack is: {}", container.get_value_from_provider_stack()); // 13

    // Override original provider on stack by new one
    println!("Push new provider on stack");
    container.provider_stack.push(Singleton::boxed(MyConstant::boxed(7)));
    println!("Value from provider stack is: {}", container.get_value_from_provider_stack()); // 7

    // Remove overriding and return to original provider
    println!("Pop provider from stack");
    container.provider_stack.pop();
    println!("Value from provider stack is: {}", container.get_value_from_provider_stack()); // 13
}

// --------------------------- //
// THE CONSTANT VALUE PROVIDER //
// --------------------------- //


#[derive(Default)]
struct MyConstant {
    val: i32
}


impl MyConstant {
    pub fn boxed(val: i32) -> Box<MyConstant> {
        Box::new(MyConstant{val})
    }
}


impl<C> InstanceFactory<C, i32, ()> for MyConstant {
    fn new_instance(&self, _: &C, _: &()) -> i32 {
        self.val
    }
}


trait ConstantProvider {
    fn get_constant(&self) -> i32;
}


// -------------------------------------- //
// THE FACTORY PROVIDER WITH NO PARAMETER //
// -------------------------------------- //

#[derive(Default)]
struct SimpleSequence {
    val: RefCell<i32>
}


impl SimpleSequence {
    pub fn boxed() -> Box<SimpleSequence> {
        Box::new(SimpleSequence::default())
    }
}


impl<C> InstanceFactory<C, i32, ()> for SimpleSequence
    where C: ConstantProvider
{
    fn new_instance(&self, container: &C, params: &()) -> i32 {
        let mut val = self.val.borrow_mut();
        *val += 1;
        container.get_constant() + *val
    }
}


trait SimpleSequenceProvider {
    fn get_next_value(&self) -> i32;
}


// -------------------------------------- //
// THE FACTORY PROVIDER WITH NO PARAMETER //
// -------------------------------------- //

#[derive(Default)]
struct SequenceWithParam {
    val: RefCell<i32>
}


impl SequenceWithParam {
    pub fn boxed() -> Box<SequenceWithParam> {
        Box::new(SequenceWithParam::default())
    }
}


impl<C> InstanceFactory<C, i32, i32> for SequenceWithParam
    where C: ConstantProvider
{
    fn new_instance(&self, container: &C, params: &i32) -> i32 {
        let mut val = self.val.borrow_mut();
        *val += 1;
        (container.get_constant() + *val) * *params
    }
}


trait SequenceWithParamProvider {
    fn get_next_parametrized_value(&self, k: i32) -> i32;
}


// ------------------------------------- //
// HERE THE SERVICE CONTAINER IS DEFINED //
// ------------------------------------- //

struct Container {
    constant_provider: Singleton<Container, i32, ()>,
    simple_sequence_provider: Factory<Container, i32, ()>,
    sequence_with_param_provider: Factory<Container, i32, i32>,
    provider_stack: ProviderStack<Container, i32, ()>
}


impl Container {
    fn new() -> Container {
        Container{
            constant_provider: Singleton::new(MyConstant::boxed(42)),
            simple_sequence_provider: Factory::new(SimpleSequence::boxed()),
            sequence_with_param_provider: Factory::new(SequenceWithParam::boxed()),
            provider_stack: ProviderStack::new(Singleton::boxed(MyConstant::boxed(13)))
        }
    }
}


impl Container {
    pub fn get_value_from_provider_stack(&self) -> i32 {
        self.provider_stack.get(self, &())
    }
}

impl ConstantProvider for Container {
    fn get_constant(&self) -> i32 {
        self.constant_provider.get(self, &())
    }
}


impl SimpleSequenceProvider for Container {
    fn get_next_value(&self) -> i32 {
        self.simple_sequence_provider.get(self, &())
    }
}


impl SequenceWithParamProvider for Container {
    fn get_next_parametrized_value(&self, k: i32) -> i32 {
        self.sequence_with_param_provider.get(self, &k)
    }
}
