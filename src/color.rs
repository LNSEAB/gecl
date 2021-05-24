use crate::*;

pub trait Values: Sized {
    fn values(value: u32, alpha: Self) -> Rgba<Self>;
}

impl Values for u8 {
    #[inline]
    fn values(value: u32, alpha: u8) -> Rgba<u8> {
        Rgba::new(
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
            alpha,
        )
    }
}

impl Values for f32 {
    #[inline]
    fn values(value: u32, alpha: f32) -> Rgba<f32> {
        Rgba::new(
            ((value >> 16) & 0xff) as f32 / std::u8::MAX as f32,
            ((value >> 8) & 0xff) as f32 / std::u8::MAX as f32,
            (value & 0xff) as f32 / std::u8::MAX as f32,
            alpha
        )
    }
}

impl Values for f64 {
    #[inline]
    fn values(value: u32, alpha: f64) -> Rgba<f64> {
        Rgba::new(
            ((value >> 16) & 0xff) as f64 / std::u8::MAX as f64,
            ((value >> 8) & 0xff) as f64 / std::u8::MAX as f64,
            (value & 0xff) as f64 / std::u8::MAX as f64,
            alpha
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Rgba<T> {
    #[inline]
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }

    #[inline]
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Rgba<R> {
        Rgba::new(f(self.r), f(self.g), f(self.b), f(self.a))
    }

    #[inline]
    pub fn values(value: u32, alpha: T) -> Rgba<T> 
    where
        T: Values
    {
        <T as Values>::values(value, alpha)
    }
}

impl<T: ToPrimitive> Rgba<T> {
    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Rgba<U>> {
        Some(Rgba::new(
            U::from(self.r)?,
            U::from(self.g)?,
            U::from(self.b)?,
            U::from(self.a)?,
        ))
    }
}

impl<T> From<(T, T, T, T)> for Rgba<T> {
    #[inline]
    fn from(src: (T, T, T, T)) -> Rgba<T> {
        Self::new(src.0, src.1, src.2, src.3)
    }
}

impl<T: Copy> From<[T; 4]> for Rgba<T> {
    #[inline]
    fn from(src: [T; 4]) -> Rgba<T> {
        Self::new(src[0], src[1], src[2], src[3])
    }
}

impl<T> PartialEq<(T, T, T, T)> for Rgba<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &(T, T, T, T)) -> bool {
        self.r == other.0 && self.g == other.1 && self.b == other.2 && self.a == other.3
    }
}

impl<T> PartialEq<[T; 4]> for Rgba<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &[T; 4]) -> bool {
        self.r == other[0] && self.g == other[1] && self.b == other[2] && self.a == other[3]
    }
}

impl<T> PartialEq<Rgba<T>> for (T, T, T, T)
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Rgba<T>) -> bool {
        self.0 == other.r && self.1 == other.g && self.2 == other.b && self.3 == other.a
    }
}

impl<T> PartialEq<Rgba<T>> for [T; 4]
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Rgba<T>) -> bool {
        self[0] == other.r && self[1] == other.g && self[2] == other.b && self[3] == other.a
    }
}

impl<T, U> std::ops::Add<U> for Rgba<T>
where
    T: std::ops::Add<T, Output = T>,
    U: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Rgba::new(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

impl<T, U> std::ops::Sub<U> for Rgba<T>
where
    T: std::ops::Sub<T, Output = T>,
    U: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Rgba::new(
            self.r - rhs.r,
            self.g - rhs.g,
            self.b - rhs.b,
            self.a - rhs.a,
        )
    }
}

impl<T> std::ops::Mul<T> for Rgba<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Rgba::new(self.r * rhs, self.g * rhs, self.b * rhs, self.a * rhs)
    }
}

impl<T> std::ops::Div<T> for Rgba<T>
where
    T: std::ops::Div<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Rgba::new(self.r / rhs, self.g / rhs, self.b / rhs, self.a / rhs)
    }
}

impl<T, U> std::ops::AddAssign<U> for Rgba<T>
where
    T: std::ops::AddAssign<T>,
    U: Into<Self>,
{
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl<T, U> std::ops::SubAssign<U> for Rgba<T>
where
    T: std::ops::SubAssign<T>,
    U: Into<Self>,
{
    fn sub_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

impl<T> std::ops::MulAssign<T> for Rgba<T>
where
    T: std::ops::MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

impl<T> std::ops::DivAssign<T> for Rgba<T>
where
    T: std::ops::DivAssign<T> + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}

#[inline]
pub fn rgba<T>(r: T, g: T, b: T, a: T) -> Rgba<T> {
    Rgba::new(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        assert!(rgba(1, 2, 3, 4).map(|x| x + 1) == rgba(2, 3, 4, 5));
    }

    #[test]
    fn eq_test() {
        assert!(rgba(1, 2, 3, 4) == rgba(1, 2, 3, 4));
        assert!(rgba(1, 2, 3, 4) == (1, 2, 3, 4));
        assert!(rgba(1, 2, 3, 4) == [1, 2, 3, 4]);
        assert!((1, 2, 3, 4) == rgba(1, 2, 3, 4));
        assert!([1, 2, 3, 4] == rgba(1, 2, 3, 4));
    }

    #[test]
    fn values_test() {
        assert!(Rgba::values(0x010203, 255u8) == (1, 2, 3, 255));
    }

    #[test]
    fn add_test() {
        let a = rgba(1, 2, 3, 4);
        let b = rgba(10, 11, 12, 13);
        let c = a + b;
        assert!(c == (11, 13, 15, 17));
        let c = a + (10, 11, 12, 13);
        assert!(c == (11, 13, 15, 17));
    }

    #[test]
    fn sub_test() {
        let a = rgba(1, 2, 3, 4);
        let b = rgba(10, 11, 12, 13);
        let c = b - a;
        assert!(c == (9, 9, 9, 9));
        let c = b - (1, 2, 3, 4);
        assert!(c == (9, 9, 9, 9));
    }

    #[test]
    fn mul_test() {
        let a = rgba(1, 2, 3, 4) * 2;
        assert!(a == (2, 4, 6, 8));
    }

    #[test]
    fn div_test() {
        let a = rgba(2, 4, 6, 8) / 2;
        assert!(a == (1, 2, 3, 4));
    }

    #[test]
    fn add_assign_test() {
        let mut a = rgba(1, 2, 3, 4);
        let b = rgba(10, 11, 12, 13);
        a += b;
        assert!(a == (11, 13, 15, 17));
        let mut a = rgba(1, 2, 3, 4);
        a += (10, 11, 12, 13);
        assert!(a == (11, 13, 15, 17));
    }

    #[test]
    fn sub_assign_test() {
        let mut a = rgba(10, 11, 12, 13);
        let b = rgba(1, 2, 3, 4);
        a -= b;
        assert!(a == (9, 9, 9, 9));
        let mut a = rgba(10, 11, 12, 13);
        a -= (1, 2, 3, 4);
        assert!(a == (9, 9, 9, 9));
    }

    #[test]
    fn mul_assign_test() {
        let mut a = rgba(1, 2, 3, 4);
        a *= 2;
        assert!(a == (2, 4, 6, 8));
    }

    #[test]
    fn div_assign_test() {
        let mut a = rgba(2, 4, 6, 8);
        a /= 2;
        assert!(a == (1, 2, 3, 4));
    }
}
