use std::iter::Iterator;
use std::ops::{Rem, Div};
use std::convert::TryInto;

pub struct Digits<T> where T: Digital {
    current: T,
    radix: u8
}

pub trait Digital where Self: Sized + Rem<Output=Self> + Div<Output=Self> + Eq + From<u8> + TryInto<u8> + Copy{
    fn digits_reversed(&self) -> Digits<Self> {
        self.digits_reversed_with_radix(10)
    }

    fn digits_reversed_with_radix(&self, radix: u8)-> Digits<Self>;
}

impl<T> Iterator for Digits<T> where T: Digital {
    type Item = u8;

    fn next(&mut self)-> Option<Self::Item> {
        if self.current == 0.into() {
            None
        } else {
            let digit: u8 = (self.current % self.radix.into()).try_into().ok().unwrap();
            self.current = self.current / self.radix.into();
            Some(digit)
        }
    }
}

impl<T> Digital for T where T: Sized + Rem<Output=Self> + Div<Output=Self> + Eq + From<u8> + TryInto<u8> + Copy {
    fn digits_reversed_with_radix(&self, radix: u8) -> Digits<Self> {
        if radix == 0 {
            panic!("Attempted to use radix 0, which is undefined")
        }
        Digits {
            current: *self,
            radix: radix
        }
    }
}
