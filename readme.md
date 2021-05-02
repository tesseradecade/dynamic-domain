# dynamic domain

> Dynamic domain implementation for linear algebra purposes

> WIP

## creating domain

```rust
use dynamic_domain::{Domain, Value};

fn main() {
    // [5;10)
    let domain = Domain::Domain(
        Value::Included(5),
        Value::Secluded(10)
    );
    println("{}", domain.repr());
}
```

## border optimization

```rust
use dynamic_domain::{Domain, Value};

fn main() {
    // (3;5]
    let mut domain = Domain::new()
        .lt(Value::Included(5))
        .gt(Value::Secluded(3))
        .gt(Value::Secluded(1));
    println!("{}", domain.repr());
}
```

## representative notation

### union
```rust
use dynamic_domain::{Domain, Value};

fn main() {
    // domain1 = (-∞;5)
    // domain2 = [8;100]
    let domain = Domain::Union(vec![domain1, domain2]);
    println!("{}", domain.repr()); // (-∞;5)⋃[8;100]
}
```

### empty set

```rust
use dynamic_domain::{Domain, Value};

fn main() {
    let domain = Domain::None;
    println!("{}", domain.repr()); // ∅
}
```