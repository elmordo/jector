# Jector

Jector is just another dependency injector for Rust.

## Project structure

```
               Application
                    ↑
                    |
                    |
           Dependency injector
                    ↑
                    |
         +----------+-----------+
         |                      |
         |                      |
     Provider 1             Provider 2
         ↑                      ↑
         |                      |
         |                      |
Instance factory 1      Instance factory 2
```

## Instance factories

Foundation of the injector is `InstanceFactory` trait. This trait provides 
functionality to create new and unique instances of value.

## Providers

The value created by `InstanceFactory` is given to a provider. Providers deliver 
values to rest of application.

### `Factory` provider

The first type of provider is the `Factory` provider. This provider always return 
unique instance of a value.

### `Singleton` provider

The second type of provider is the `Singleton` provider. This provider create value
only once and then always return copy of this instance. To reach real singleton
behavior, the `InstanceFactory` should wrap the value into `Rc` or `Arc` container.

### `ProviderStack` provider

Special provider is `ProviderStack`. Allows override provider by another provider 
and then revert the override back to original provider.

## Example

```rust
// HERE WILL BE SOME EXAMPLE
```
