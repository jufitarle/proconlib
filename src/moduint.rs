use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Ord, Hash)]
struct Musize {
    val: usize,
    modulo: usize,
}

impl Musize {
    fn new(val: usize, modulo: usize) -> Musize {
        Musize {
            val: val % modulo,
            modulo,
        }
    }

    fn inv(&self) -> usize {
        let mut x = 0;
        let mut y = 0;
        let modulo = self.modulo as isize;
        Musize::extgcd(self.val as isize, modulo, &mut x, &mut y);
        x %= modulo;
        if x < 0 {
            x += modulo;
        }
        x as usize
    }

    fn extgcd(a: isize, b: isize, x: &mut isize, y: &mut isize) -> isize {
        if b == 0 {
            *x = 1;
            *y = 0;
            return a;
        }

        let q = a / b;
        let g = Musize::extgcd(b, a - q * b, x, y);
        let z = *x - q * *y;
        *x = *y;
        *y = z;
        g
    }

    fn pow(&mut self, e: usize) {
        let mut e = e;
        let mut p = Musize::new(1, self.modulo);
        let mut cur = Musize::new(self.val, self.modulo);
        while e > 0 {
            if e % 2 == 1 {
                p *= cur;
            }
            cur *= cur;
            e /= 2;
        }
        *self = p;
    }
}

impl Add<usize> for Musize {
    type Output = Musize;
    fn add(self, rhs: usize) -> Self::Output {
        Musize::new((self.val + rhs) % self.modulo, self.modulo)
    }
}

impl AddAssign<usize> for Musize {
    fn add_assign(&mut self, rhs: usize) {
        self.val = (self.val + rhs) % self.modulo;
    }
}

impl Sub<usize> for Musize {
    type Output = Musize;
    fn sub(self, rhs: usize) -> Self::Output {
        let n = self.val + self.modulo;
        let m = rhs % self.modulo;
        Musize::new((n - m) % self.modulo, self.modulo)
    }
}

impl SubAssign<usize> for Musize {
    fn sub_assign(&mut self, rhs: usize) {
        self.val += self.modulo;
        let rhs = rhs % self.modulo;
        self.val = (self.val - rhs) % self.modulo;
    }
}

impl Mul<usize> for Musize {
    type Output = Musize;
    fn mul(self, rhs: usize) -> Self::Output {
        Musize::new((self.val * (rhs % self.modulo)) % self.modulo, self.modulo)
    }
}

impl MulAssign<usize> for Musize {
    fn mul_assign(&mut self, rhs: usize) {
        self.val = (self.val * (rhs % self.modulo)) % self.modulo;
    }
}

impl Div<usize> for Musize {
    type Output = Musize;
    fn div(self, rhs: usize) -> Self::Output {
        let rhs = Musize::new(rhs, self.modulo);
        Musize::new(self.val * rhs.inv() % self.modulo, self.modulo)
    }
}

impl DivAssign<usize> for Musize {
    fn div_assign(&mut self, rhs: usize) {
        let rhs = Musize::new(rhs, self.modulo);
        self.val = (self.val * rhs.inv()) % self.modulo;
    }
}

impl Add<Musize> for Musize {
    type Output = Musize;
    fn add(self, rhs: Musize) -> Self::Output {
        Musize::new((self.val + rhs.val) % self.modulo, self.modulo)
    }
}

impl AddAssign<Musize> for Musize {
    fn add_assign(&mut self, rhs: Musize) {
        self.val = (self.val * rhs.val) % self.modulo;
    }
}

impl Sub<Musize> for Musize {
    type Output = Musize;
    fn sub(self, rhs: Musize) -> Self::Output {
        let n = self.val + self.modulo;
        let m = rhs.val % self.modulo;
        Musize::new((n - m) % self.modulo, self.modulo)
    }
}

impl SubAssign<Musize> for Musize {
    fn sub_assign(&mut self, rhs: Musize) {
        let n = self.val + self.modulo;
        let m = rhs.val % self.modulo;
        self.val = (n - m) % self.modulo;
    }
}

impl Mul<Musize> for Musize {
    type Output = Musize;
    fn mul(self, rhs: Musize) -> Self::Output {
        Musize::new(
            (self.val * (rhs.val % self.modulo)) % self.modulo,
            self.modulo,
        )
    }
}

impl MulAssign<Musize> for Musize {
    fn mul_assign(&mut self, rhs: Musize) {
        self.val = (self.val * (rhs.val % self.modulo)) % self.modulo;
    }
}

impl Div<Musize> for Musize {
    type Output = Musize;
    fn div(self, rhs: Musize) -> Self::Output {
        Musize::new(self.val * rhs.inv() % self.modulo, self.modulo)
    }
}

impl DivAssign<Musize> for Musize {
    fn div_assign(&mut self, rhs: Musize) {
        self.val = (self.val * rhs.inv()) % self.modulo;
    }
}
