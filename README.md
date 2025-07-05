# enum_builder
Simple macros that allow building enum types from variants that can be defined in multiple dispersed files in the crate.

![Crates.io](https://img.shields.io/crates/v/enum_builder)
![Documentation](https://img.shields.io/docsrs/enum_builder)
![License](https://img.shields.io/github/license/garfunkel/enum_builder)
![GitHub](https://img.shields.io/github/stars/garfunkel/enum_builder)

## Example
#### main.rs
```rust
mod animals;

use animals::*;

#[enum_builder]
enum Animal {}

// expanded result
// enum Animal {
//     Dog(Dog),
//     Cow(Cow),
//     Fish(Fish),
// }
```

#### animals.rs
```rust
#[enum_builder_variant(Animal)]
struct Dog {}

#[enum_builder_variant(Animal)]
struct Cow {}

#[enum_builder_variant(Animal)]
struct Fish {}
```

## Tips
It can be very useful to combine this crate with the
[enum_dispatch](https://docs.rs/enum_dispatch/latest/enum_dispatch) crate, to allow for a simple
"plugin" architecture without the overhead of dynamic dispatch. When doing this, take care to note
that the order of macros is important, as [macro@enum_builder] must be used before
[enum_dispatch](https://docs.rs/enum_dispatch/latest/enum_dispatch/attr.enum_dispatch.html).

#### Example
```rust
#[enum_builder]
#[enum_dispatch]
enum Animal {}
```
