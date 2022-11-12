use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vector2(pub f64, pub f64);

impl Vector2 {
    #[inline]
    pub const fn zero() -> Self {
        Vector2(0., 0.)
    }

    #[inline]
    pub const fn forward() -> Self {
        Vector2(0., 1.)
    }

    #[inline]
    pub const fn right() -> Self {
        Vector2(1., 0.)
    }

    #[inline]
    pub fn sqr_magnitude(self) -> f64 {
        self.0 * self.0 + self.1 * self.1
    }

    #[inline]
    pub fn magnitude(self) -> f64 {
        self.sqr_magnitude().sqrt()
    }

    #[inline]
    pub fn normalized(self) -> Self {
        self / self.magnitude()
    }

    /// 逆时针绕原点旋转指定的弧度
    #[inline]
    pub fn rotate(self, radian: f64) -> Self {
        Vector2(
            self.0 * radian.cos() - self.1 * radian.sin(),
            self.0 * radian.sin() + self.1 * radian.cos(),
        )
    }
}

impl ops::Add for Vector2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl ops::AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vector2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Vector2(-self.0, -self.1)
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl ops::Mul<f64> for Vector2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

impl ops::Mul<Vector2> for f64 {
    type Output = Vector2;

    #[inline]
    fn mul(self, rhs: Vector2) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f64> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Mul for Vector2 {
    type Output = f64;

    /// 点乘
    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.0 * rhs.0 + self.1 * rhs.1
    }
}

impl ops::Div<f64> for Vector2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Vector2(self.0 / rhs, self.1 / rhs)
    }
}

impl ops::DivAssign<f64> for Vector2 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Vector2::zero()
    }
}
