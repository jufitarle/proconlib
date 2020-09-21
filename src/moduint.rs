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

    fn inv(n: usize, modulo: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let modulo = modulo as isize;
        Musize::extgcd(n as isize, modulo, &mut x, &mut y);
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
}

impl Add<usize> for Musize {
    type Output = Musize;
    fn add(self, rhs: usize) -> Self::Output {
        Musize {
            val: (self.val + rhs) % self.modulo,
            modulo: self.modulo,
        }
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
        Musize {
            val: (n - m) % self.modulo,
            modulo: self.modulo,
        }
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
        Musize {
            val: (self.val * (rhs % self.modulo)) % self.modulo,
            modulo: self.modulo,
        }
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
        Musize {
            val: (self.val * Musize::inv(rhs, self.modulo)) % self.modulo,
            modulo: self.modulo,
        }
    }
}

impl DivAssign<usize> for Musize {
    fn div_assign(&mut self, rhs: usize) {
        self.val = (self.val * Musize::inv(rhs, self.modulo)) % self.modulo;
    }
}
