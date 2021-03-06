// vim: sts=4 sw=4 et


use super::std;
use super::std::rand::{Rand, RngUtil};

use std::cell::Cell;
use std::hashmap::{HashMap, HashSet};
/* Arbitrary */

/**
 The Arbitrary trait can generate a randomly chosen value (with restrictions).
 You can pass a size factor to allow specifying test size (sizes of vectors and
 numbers).
 */
pub trait Arbitrary {
    /**
     arbitrary should return an arbitrary value of its type.
     The value should be randomly chosen and its size should be scaled by the size
     parameter.
     */
    fn arbitrary(uint) -> Self;
}

/// Create an arbitrary value of type T
#[inline]
pub fn arbitrary<T: Arbitrary>(sz: uint) -> T {
    Arbitrary::arbitrary(sz)
}

/// A wrapper type to reuse an existing Rand instance for the Arbitrary impl
#[deriving(IterBytes, Eq, Clone)]
pub struct Random<T>(T);

/// A small number >= 0.
#[deriving(Eq, Clone)]
pub struct SmallN(uint);

fn small_n(size: uint) -> uint {
    let f: std::rand::distributions::Exp1 = std::rand::random();
    let n = ((*f) * (size as f64)) as uint;
    n.min(&(16 * size))
}

/* Helper: Iter */
#[deriving(Clone)]
struct Iter<T> {
    count: uint,
    size: uint,
}

fn arbiter<T>(sz: uint) -> Iter<T> {
    Iter{count: small_n(sz), size: sz }
}

impl<T: Arbitrary> Iterator<T> for Iter<T> {
    fn next(&mut self) -> Option<T> {
        if self.count > 0 {
            self.count -= 1;
            Some(arbitrary(self.size))
        } else { None }
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (self.count, Some(self.count))
    }
}


macro_rules! arb_rand( ($T:ty) => (
        impl Arbitrary for $T {
            fn arbitrary(_: uint) -> $T {
                std::rand::random()
            }
        }
    )
)

macro_rules! arb_tuple( ($($T:ident),+ ) => (
        impl<$($T: Arbitrary),+> Arbitrary for ($($T),+) {
            fn arbitrary(sz: uint) -> ($($T),+) {
                ($(arbitrary::<$T>(sz)),+)
            }
        }
    )
)

arb_rand!(i8)
//arb_rand!(u8)
arb_rand!(int)
arb_rand!(uint)
arb_rand!(float)
arb_rand!(bool)
arb_rand!(())

arb_tuple!(A, B)
arb_tuple!(A, B, C)
arb_tuple!(A, B, C, D)
arb_tuple!(A, B, C, D, E)
arb_tuple!(A, B, C, D, E, F)
arb_tuple!(A, B, C, D, E, F, G)
arb_tuple!(A, B, C, D, E, F, G, H)

impl<T: Rand> Arbitrary for Random<T> {
    fn arbitrary(_: uint) -> Random<T> {
        Random(std::rand::random())
    }
}

impl<T: Arbitrary> Arbitrary for ~T {
    #[inline]
    fn arbitrary(sz: uint) -> ~T { ~arbitrary(sz) }
}

impl<T: 'static + Arbitrary> Arbitrary for @T {
    #[inline]
    fn arbitrary(sz: uint) -> @T { @arbitrary(sz) }
}

impl<T: 'static + Arbitrary> Arbitrary for @mut T {
    #[inline]
    fn arbitrary(sz: uint) -> @mut T { @mut arbitrary(sz) }
}

impl Arbitrary for u8 {
    fn arbitrary(_: uint) -> u8 {
        std::rand::random()
    }
}

impl Arbitrary for char {
    fn arbitrary(_: uint) -> char {
        std::rand::random::<u8>() as char
    }
}

impl Arbitrary for SmallN {
    fn arbitrary(sz: uint) -> SmallN {
        SmallN(small_n(sz))
    }
}

impl<T: Arbitrary> Arbitrary for ~[T] {
    fn arbitrary(sz: uint) -> ~[T] {
        arbiter::<T>(sz).collect()
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn arbitrary(sz: uint) -> Option<T> {
        if std::rand::random() {
            Some(arbitrary(sz))
        } else {
            None
        }
    }

}

impl<T: Arbitrary, U: Arbitrary> Arbitrary for Result<T, U> {
    fn arbitrary(sz: uint) -> Result<T, U> {
        if std::rand::random() {
            Ok(arbitrary(sz))
        } else {
            Err(arbitrary(sz))
        }
    }
}

impl<T: Arbitrary, U: Arbitrary> Arbitrary for Either<T, U> {
    fn arbitrary(sz: uint) -> Either<T, U> {
        if std::rand::random() {
            Left(arbitrary(sz))
        } else {
            Right(arbitrary(sz))
        }
    }
}

impl Arbitrary for ~str {
    fn arbitrary(sz: uint) -> ~str {
        let rng = &mut *std::rand::task_rng();
        let n = small_n(sz);
        rng.gen_str(n)
    }
}

impl <T: Arbitrary> Arbitrary for Cell<T> {
    fn arbitrary(sz: uint) -> Cell<T> {
        if std::rand::random() {
            Cell::new(arbitrary(sz))
        } else {
            Cell::new_empty()
        }
    }
}

impl<K: Eq + Hash + Arbitrary> Arbitrary for HashSet<K> {
    fn arbitrary(sz: uint) -> HashSet<K> {
        arbiter::<K>(sz).collect()
    }
}

impl<K: Eq + Hash + Arbitrary, V: Arbitrary> Arbitrary for HashMap<K, V> {
    fn arbitrary(sz: uint) -> HashMap<K, V> {
        arbiter::<(K, V)>(sz).collect()
    }
}
