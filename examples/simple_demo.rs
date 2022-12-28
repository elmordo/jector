use jector::{ValueFactory, Provider};
use jector::providers::Singleton;

fn main() {
    let container = Container::new();
    println!("The constant value is: {}", container.get_constant());  // value is 42
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

impl<C> ValueFactory<C, i32, ()> for MyConstant {
    fn new_instance(&self, _: &C, _: &()) -> i32 {
        self.val
    }
}

trait ConstantProvider {
    fn get_constant(&self) -> i32;
}

// ------------------------------------- //
// HERE THE SERVICE CONTAINER IS DEFINED //
// ------------------------------------- //

struct Container {
    constant_provider: Singleton<Container, i32, ()>,
}


impl Container {
    fn new() -> Container {
        Container{
            constant_provider: Singleton::new(MyConstant::boxed(42)),
        }
    }
}

impl ConstantProvider for Container {
    fn get_constant(&self) -> i32 {
        self.constant_provider.get(self, &())
    }
}
