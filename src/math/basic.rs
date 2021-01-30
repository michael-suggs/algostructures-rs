//! Basic mathematical algorithms.

use num_traits::{PrimInt, Signed};

pub trait GCD: PrimInt + Signed {
    fn gcd_euclidean(self, other: Self) -> Self {
        if other == Self::zero() {
            self.abs()
        } else {
            other.gcd_euclidean(self % other)
        }
    }

    fn gcd_euclidean_extended(self, other: Self) -> Self {
        let mut a0: Self = Self::one();
        let mut a1: Self = Self::zero();
        let mut b0: Self = Self::zero();
        let mut b1: Self = Self::one();
        let mut q: Self;
        let mut temp: Self;
        let mut m: Self = self;
        let mut n: Self = other;

        while !m.is_zero() {
            q = self.div(other);
            temp = a0 - a1 * q;
            a0 = a1;
            a1 = temp;
            temp = b0 - b1 * q;
            b0 = b1;
            b1 = temp;
            temp = self - other * q;
            m = n;
            n = temp;
        }
        n
    }

    fn gcd_dijkstra(self, other: Self) -> Self {
        if self == other {
            self
        } else if self > other {
            Self::gcd_dijkstra(self - other, other)
        } else {
            Self::gcd_dijkstra(self, other - self)
        }
    }

    fn gcd_bishop(self, other: Self) -> Self {
        let mut x: Self = self;
        let mut y: Self = other;

        while x != y {
            if x > y {
                x = x - y
            } else {
                let temp = y;
                y = x;
                x = temp;
            }
        }
        x
    }
}

impl<I: PrimInt + Signed> GCD for I {}

pub trait LCM: GCD {
    fn lcm(self, other: Self) -> Self {
        (self / self.gcd_euclidean(other) * other).abs()
    }
}

impl<I: GCD> LCM for I {}
