use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    #[inline]
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    #[inline]
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Size<R> {
        Size::new(f(self.width), f(self.height))
    }
}

impl<T: ToPrimitive> Size<T> {
    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Size<U>> {
        Some(Size::new(U::from(self.width)?, U::from(self.height)?))
    }
}

impl<T> From<(T, T)> for Size<T> {
    #[inline]
    fn from(src: (T, T)) -> Size<T> {
        Size::new(src.0, src.1)
    }
}

impl<T: Copy> From<[T; 2]> for Size<T> {
    #[inline]
    fn from(src: [T; 2]) -> Size<T> {
        Size::new(src[0], src[1])
    }
}

impl<T> From<Vector<T>> for Size<T> {
    #[inline]
    fn from(src: Vector<T>) -> Size<T> {
        Size::new(src.x, src.y)
    }
}

impl<T> PartialEq<(T, T)> for Size<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &(T, T)) -> bool {
        self.width == other.0 && self.height == other.1
    }
}

impl<T> PartialEq<[T; 2]> for Size<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &[T; 2]) -> bool {
        self.width == other[0] && self.height == other[1]
    }
}

impl<T> PartialEq<Size<T>> for (T, T)
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Size<T>) -> bool {
        self.0 == other.width && self.1 == other.height
    }
}

impl<T> PartialEq<Size<T>> for [T; 2]
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Size<T>) -> bool {
        self[0] == other.width && self[1] == other.height
    }
}

impl<T, U> std::ops::Add<U> for Size<T>
where
    T: std::ops::Add<T, Output = T>,
    U: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Self::new(self.width + rhs.width, self.height + rhs.height)
    }
}

impl<T, U> std::ops::Sub<U> for Size<T>
where
    T: std::ops::Sub<T, Output = T>,
    U: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Self::new(self.width - rhs.width, self.height - rhs.height)
    }
}

impl<T> std::ops::Mul<T> for Size<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self::new(self.width * rhs, self.height * rhs)
    }
}

impl<T> std::ops::Div<T> for Size<T>
where
    T: std::ops::Div<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self::new(self.width / rhs, self.height / rhs)
    }
}

impl<T, U> std::ops::AddAssign<U> for Size<T>
where
    T: std::ops::AddAssign,
    U: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.width += rhs.width;
        self.height += rhs.height;
    }
}

impl<T, U> std::ops::SubAssign<U> for Size<T>
where
    T: std::ops::SubAssign,
    U: Into<Self>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.width -= rhs.width;
        self.height -= rhs.height;
    }
}

impl<T> std::ops::MulAssign<T> for Size<T>
where
    T: std::ops::MulAssign + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.width *= rhs;
        self.height *= rhs;
    }
}

impl<T> std::ops::DivAssign<T> for Size<T>
where
    T: std::ops::DivAssign + Copy,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.width /= rhs;
        self.height /= rhs;
    }
}

#[inline]
pub fn size<T>(width: T, height: T) -> Size<T> {
    Size::new(width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        assert!(size(1, 2).map(|x| x + 1) == size(2, 3));
    }

    #[test]
    fn eq_test() {
        assert!(size(1, 2) == size(1, 2));
        assert!(size(1, 2) == (1, 2));
        assert!(size(1, 2) == [1, 2]);
        assert!((1, 2) == size(1, 2));
        assert!([1, 2] == size(1, 2));
    }

    #[test]
    fn add_test() {
        let a = size(1, 2);
        let b = size(6, 7);
        let c = a + b;
        assert!(c == (7, 9));
        let a = size(1, 2);
        let b = vector(6, 7);
        let c = a + b;
        assert!(c == (7, 9));
        let c = a + (6, 7);
        assert!(c == (7, 9));
    }

    #[test]
    fn sub_test() {
        let a = size(6, 7);
        let b = vector(1, 2);
        let c = a - b;
        assert!(c == size(5, 5));
    }

    #[test]
    fn mul_test() {
        let a = size(1, 2);
        let b = a * 2;
        assert!(b == (2, 4));
    }

    #[test]
    fn div_test() {
        let a = size(2, 6);
        let b = a / 2;
        assert!(b == (1, 3));
    }

    #[test]
    fn add_assign_test() {
        let mut a = size(1, 2);
        let b = size(6, 7);
        a += b;
        assert!(a == (7, 9));
        let mut a = size(1, 2);
        let b = vector(6, 7);
        a += b;
        assert!(a == (7, 9));
        let mut a = size(1, 2);
        a += (6, 7);
        assert!(a == (7, 9));
    }

    #[test]
    fn sub_assign_test() {
        let mut a = size(6, 7);
        let b = vector(1, 2);
        a -= b;
        assert!(a == (5, 5));
    }

    #[test]
    fn mul_assign_test() {
        let mut a = size(1, 2);
        a *= 2;
        assert!(a == (2, 4));
    }

    #[test]
    fn div_assign_test() {
        let mut a = size(3, 6);
        a /= 3;
        assert!(a == (1, 2));
    }
}
