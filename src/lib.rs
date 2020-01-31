use num::Num;
use std::ops::{BitAnd, BitOr, Not};

pub trait FlagSet<T>: std::default::Default
where
    T: Clone + Num + BitAnd<Output = T> + BitOr<Output = T> + Not<Output = T>,
{
    fn value(&self) -> T;

    fn new() -> Self {
        Self::default()
    }

    fn from(v: T) -> Self {
        let mut f = Self::default();
        f.set(v);
        f
    }

    fn set(&mut self, v: T);

    fn enable(&mut self, f: T) {
        self.set(self.value() | f);
    }

    fn disable(&mut self, f: T) {
        self.set(self.value() & !f);
    }

    fn toggle(&mut self, f: T) {
        let v = if self.value() & f.clone() != T::zero() {
            self.value() & !f
        } else {
            self.value() | f
        };

        self.set(v);
    }

    fn is_enabled(&self, f: T) -> bool {
        self.value() & f != T::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    pub struct Flags {
        value: u8,
    }

    impl FlagSet<u8> for Flags {
        fn value(&self) -> u8 {
            self.value
        }

        fn set(&mut self, v: u8) {
            self.value = v
        }
    }

    const FLAG_ONE: u8 = 0b00000001;

    #[test]
    fn can_enable_a_flag() {
        let mut f = Flags::new();
        assert!(!f.is_enabled(FLAG_ONE));
        f.enable(FLAG_ONE);
        assert!(f.is_enabled(FLAG_ONE));
    }

    #[test]
    fn can_disable_a_flag() {
        let mut f: Flags = FlagSet::from(1);
        assert!(f.is_enabled(FLAG_ONE));

        f.disable(FLAG_ONE);
        assert!(!f.is_enabled(FLAG_ONE));
    }

    #[test]
    fn can_toggle_a_flag() {
        let mut f = Flags::new();
        assert!(!f.is_enabled(FLAG_ONE));

        f.toggle(FLAG_ONE);
        assert!(f.is_enabled(FLAG_ONE));

        f.toggle(FLAG_ONE);
        assert!(!f.is_enabled(FLAG_ONE));
    }
}
