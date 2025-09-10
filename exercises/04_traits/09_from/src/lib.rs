// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    // The function signature now matches the trait's definition.
    // It takes a `u32` and returns `Self` (which is `WrappingU32`).
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
