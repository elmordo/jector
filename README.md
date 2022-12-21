# Jector

Jector is just another dependency injector for Rust.

## Project structure

* **Application** is your app (or lib) where dependency injection is used.
* **Dependency injector** is container struct where providers are stored and. Also implements
specific provider traits.
* **Specific providers** are traits implemented by **dependency injector** and used by **providers**
and **application** to access **values** provided by **providers**.
* **Provider** are responsible for the **value** providing strategy. At this time, 
the `Factory` (always returns unique **value**) and the `Singleton` (create **value** only once and then returns 
same instance) providers are supported.
* **Value factories** are responsible for **value** creation strategy.
* **Values** are instances of services, config values, ... provided created by **value factories** 
and provided by **provides**

```
               Application
                    ↑
                    |
                    |
           Dependency injector : Specific provider 1 + Specific provider 2
                    ↑
                    |
         +----------+-----------+
         |                      |
         |                      |
     Provider 1             Provider 2
         ↑                      ↑
         |                      |
         |                      |
  Value factory 1         Value factory 2
```

## Value factories

Foundation of the injector is `ValueFactory` trait. This trait provides 
functionality to create new and unique instances of value.

## Providers

The value created by `ValueFactory` is given to a provider. Providers deliver 
values to rest of application.

### `Factory` provider

The first type of provider is the `Factory` provider. This provider always return 
unique instance of a value.

### `Singleton` provider

The second type of provider is the `Singleton` provider. This provider create value
only once and then always return copy of this instance. To reach real singleton
behavior, the `ValueFactory` should wrap the value into `Rc` or `Arc` container.

### `ProviderStack` provider

Special provider is `ProviderStack`. Allows override provider by another provider 
and then revert the override back to original provider.

## Example

Following example can be found in `examples/simple_demo.rs` or you can run it by `cargo run --example simple_demo`

```rust
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

```
