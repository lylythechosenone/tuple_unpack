# tuple_unpack
Unpack Rust tuples.

## Usage
```rust
use tuple_unpack::TupleUnpack;
fn main() {
    let some_tuple = (1, 2, 3);
    let types = some_tuple.unpack_types();
    assert_eq!(types, vec![TypeId::of::<i32>; 3]);
    let values = some_tuple.unpack_values();
    assert_eq!(values, vec![1, 2, 3]);
}
```