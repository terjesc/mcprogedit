//! Integer data types with limited range of values.

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 1 inclusive.
    pub struct Int0Through1 { 0..=1 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 2 inclusive.
    pub struct Int0Through2 { 0..=2 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 3 inclusive.
    pub struct Int0Through3 { 0..=3 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 4 inclusive.
    pub struct Int0Through4 { 0..=4 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 5 inclusive.
    pub struct Int0Through5 { 0..=5 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 6 inclusive.
    pub struct Int0Through6 { 0..=6 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 7 inclusive.
    pub struct Int0Through7 { 0..=7 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 8 inclusive.
    pub struct Int0Through8 { 0..=8 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 15 inclusive.
    pub struct Int0Through15 { 0..=15 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 24 inclusive.
    pub struct Int0Through24 { 0..=24 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 0 to 25 inclusive.
    pub struct Int0Through25 { 0..=25 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 1 to 4 inclusive.
    pub struct Int1Through4 { 1..=4 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 1 to 7 inclusive.
    pub struct Int1Through7 { 1..=7 }
}

bounded_integer! {
    #[repr(i8)]
    /// Integers in the range from 1 to 8 inclusive.
    pub struct Int1Through8 { 1..=8 }
}
