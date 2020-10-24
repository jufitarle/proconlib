
    pub mod mint {
        use super::constants::MOD;

        #[derive(Debug, Default, Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
        pub struct Mint {
            pub val: isize,
        }

        #[allow(dead_code)]
        impl Mint {
            pub fn new(val: isize) -> Self {
                Mint {
                    val: Mint::adjust(val),
                }
            }

            fn init(&mut self) {
                self.val = Self::adjust(self.val);
            }

            fn adjust(val: isize) -> isize {
                let val = if val < 0 {
                    val + (val.abs() / MOD + 1) * MOD
                } else {
                    val
                };
                val % MOD
            }

            pub fn inv(&self) -> Mint {
                self.pow((MOD - 2) as usize)
            }

            pub fn pow(&self, n: usize) -> Mint {
                let mut a = self.val;
                let mut res = Mint::new(1);
                let mut n = n;
                while n > 0 {
                    if n & 1 == 1 {
                        res *= a;
                    }
                    a = Self::adjust(a * a);
                    n >>= 1;
                }
                res
            }
        }

        mod mint_traits {
            use super::Mint;

            impl std::fmt::Display for Mint {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.val)
                }
            }

            impl proconio::source::Readable for Mint {
                type Output = Mint;

                fn read<R: std::io::BufRead, S: proconio::source::Source<R>>(
                    source: &mut S,
                ) -> Self::Output {
                    Mint::new(isize::read(source))
                }
            }

            mod operators {
                use super::Mint;
                use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

                impl Add<Mint> for Mint {
                    type Output = Mint;

                    fn add(self, rhs: Mint) -> Self::Output {
                        Mint::new(self.val + rhs.val)
                    }
                }

                impl AddAssign<Mint> for Mint {
                    fn add_assign(&mut self, rhs: Mint) {
                        self.val += rhs.val;
                        self.init();
                    }
                }

                impl Sub<Mint> for Mint {
                    type Output = Mint;

                    fn sub(self, rhs: Mint) -> Self::Output {
                        Mint::new(self.val - rhs.val)
                    }
                }

                impl SubAssign<Mint> for Mint {
                    fn sub_assign(&mut self, rhs: Mint) {
                        self.val -= rhs.val;
                        self.init();
                    }
                }

                impl Mul<Mint> for Mint {
                    type Output = Mint;

                    fn mul(self, rhs: Mint) -> Self::Output {
                        Mint::new(self.val * rhs.val)
                    }
                }

                impl MulAssign<Mint> for Mint {
                    fn mul_assign(&mut self, rhs: Mint) {
                        self.val *= rhs.val;
                        self.init();
                    }
                }

                impl Div<Mint> for Mint {
                    type Output = Mint;

                    fn div(self, rhs: Mint) -> Self::Output {
                        self * rhs.inv()
                    }
                }

                impl DivAssign<Mint> for Mint {
                    fn div_assign(&mut self, rhs: Mint) {
                        self.val = rhs.inv().val * self.val;
                        self.init();
                    }
                }

                macro_rules! impl_primitive {
                    ($($t: ty),*) => {
                        $(
                            impl Add<$t> for Mint {
                                type Output = Mint;

                                fn add(self, rhs: $t) -> Self::Output {
                                    Mint::new(rhs as isize + self.val)
                                }
                            }

                            impl AddAssign<$t> for Mint {
                                fn add_assign(&mut self, rhs: $t) {
                                    self.val += rhs as isize;
                                    self.init();
                                }
                            }

                            impl Sub<$t> for Mint {
                                type Output = Mint;

                                fn sub(self, rhs: $t) -> Self::Output {
                                    Mint::new(self.val - rhs as isize)
                                }
                            }

                            impl SubAssign<$t> for Mint {
                                fn sub_assign(&mut self, rhs: $t) {
                                    self.val -= rhs as isize;
                                    self.init();
                                }
                            }

                            impl Mul<$t> for Mint {
                                type Output = Mint;

                                fn mul(self, rhs: $t) -> Self::Output {
                                    Mint::new(self.val * rhs as isize)
                                }
                            }

                            impl MulAssign<$t> for Mint {
                                fn mul_assign(&mut self, rhs: $t) {
                                    self.val *= rhs as isize;
                                    self.init();
                                }
                            }

                            impl Div<$t> for Mint {
                                type Output = Mint;

                                fn div(self, rhs: $t) -> Self::Output {
                                    let rhs = Mint::new(rhs as isize);
                                    self * rhs.inv()
                                }
                            }

                            impl DivAssign<$t> for Mint {
                                fn div_assign(&mut self, rhs: $t) {
                                    let rhs = Mint::new(rhs as isize);
                                    self.val = rhs.inv().val * self.val;
                                    self.init();
                                }
                            }

                        )*
                    };
                }

                impl_primitive!(isize, i32, usize, i64, u64, u32, i128, u128, u8, i8, u16, i16);
            }
        }
    }
