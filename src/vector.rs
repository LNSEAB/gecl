use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Vector<R> {
        Vector::new(f(self.x), f(self.y))
    }
}

impl<T: ToPrimitive> Vector<T> {
    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Vector<U>> {
        Some(Vector::new(U::from(self.x)?, U::from(self.y)?))
    }
}

impl<T> Vector<T>
where
    T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T>,
{
    #[inline]
    pub fn dot(self, rhs: impl Into<Self>) -> T {
        let rhs = rhs.into();
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T> Vector<T>
where
    T: std::ops::Sub<T, Output = T> + std::ops::Mul<T, Output = T>,
{
    #[inline]
    pub fn cross(self, rhs: impl Into<Self>) -> T {
        let rhs = rhs.into();
        self.x * rhs.y - self.y * rhs.x
    }
}

impl<T> Vector<T>
where
    T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> + Copy,
{
    #[inline]
    pub fn abs_pow2(self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T: Float> Vector<T> {
    #[inline]
    pub fn abs(self) -> T {
        T::sqrt(self.x.powi(2) + self.y.powi(2))
    }
}

impl<T> PartialEq<(T, T)> for Vector<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<T> PartialEq<[T; 2]> for Vector<T>
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &[T; 2]) -> bool {
        self.x == other[0] && self.y == other[1]
    }
}

impl<T> PartialEq<Vector<T>> for (T, T)
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Vector<T>) -> bool {
        self.0 == other.x && self.1 == other.y
    }
}

impl<T> PartialEq<Vector<T>> for [T; 2]
where
    T: PartialEq,
{
    #[inline]
    fn eq(&self, other: &Vector<T>) -> bool {
        self[0] == other.x && self[1] == other.y
    }
}

impl<T> From<(T, T)> for Vector<T> {
    #[inline]
    fn from(src: (T, T)) -> Vector<T> {
        Vector::new(src.0, src.1)
    }
}

impl<T: Copy> From<[T; 2]> for Vector<T> {
    #[inline]
    fn from(src: [T; 2]) -> Vector<T> {
        Vector::new(src[0], src[1])
    }
}

impl<T> From<Point<T>> for Vector<T> {
    #[inline]
    fn from(src: Point<T>) -> Vector<T> {
        Vector::new(src.x, src.y)
    }
}

impl<T> From<Size<T>> for Vector<T> {
    #[inline]
    fn from(src: Size<T>) -> Vector<T> {
        Vector::new(src.width, src.height)
    }
}

impl<T, U> std::ops::Add<U> for Vector<T>
where
    T: std::ops::Add<T, Output = T>,
    U: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, U> std::ops::Sub<U> for Vector<T>
where
    T: std::ops::Sub<T, Output = T>,
    U: Into<Self>,
{
    type Output = Self;

    #[inline]
    fn sub(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> std::ops::Mul<T> for Vector<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> std::ops::Div<T> for Vector<T>
where
    T: std::ops::Div<T, Output = T> + Copy,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T, U> std::ops::AddAssign<U> for Vector<T>
where
    T: std::ops::AddAssign<T>,
    U: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, U> std::ops::SubAssign<U> for Vector<T>
where
    T: std::ops::SubAssign<T>,
    U: Into<Self>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> std::ops::MulAssign<T> for Vector<T>
where
    T: std::ops::MulAssign<T> + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> std::ops::DivAssign<T> for Vector<T>
where
    T: std::ops::DivAssign<T> + Copy,
{
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[inline]
pub fn vector<T>(x: T, y: T) -> Vector<T> {
    Vector::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        assert!(vector(1, 2).map(|x| x * 2) == (2, 4));
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn dot_test() {
        assert!(vector(1, 2).dot((3, 4)) == 1 * 3 + 2 * 4);
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn cross_test() {
        assert!(vector(3, 4).cross((1, 2)) == 3 * 2 - 1 * 4);
    }

    #[test]
    fn abs_pow2_test() {
        assert!(vector(2, 3).abs_pow2() == 2 * 2 + 3 * 3);
    }

    #[test]
    fn abs_test() {
        let d = vector(2.0, 3.0).abs() - f32::sqrt(2.0 * 2.0 + 3.0 * 3.0);
        assert!(d.abs() <= f32::EPSILON);
    }

    #[test]
    fn eq_test() {
        assert!(vector(1, 2) == vector(1, 2));
        assert!(vector(1, 2) == (1, 2));
        assert!(vector(1, 2) == [1, 2]);
        assert!((1, 2) == vector(1, 2));
        assert!([1, 2] == vector(1, 2));
    }

    #[test]
    fn add_test() {
        let a = vector(1, 2);
        let b = vector(6, 7);
        let c = a + b;
        assert!(c == (7, 9));
        let c = a + (6, 7);
        assert!(c == (7, 9));
    }

    #[test]
    fn sub_test() {
        let a = vector(1, 2);
        let b = vector(6, 7);
        let c = b - a;
        assert!(c == (5, 5));
        let c = b - (1, 2);
        assert!(c == (5, 5));
    }

    #[test]
    fn mul_test() {
        let a = vector(1, 2);
        let b = a * 2;
        assert!(b == (2, 4));
    }

    #[test]
    fn div_test() {
        let a = vector(2, 6);
        let b = a / 2;
        assert!(b == (1, 3));
    }

    #[test]
    fn add_assign_test() {
        let mut a = vector(1, 2);
        let b = vector(6, 7);
        a += b;
        assert!(a == (7, 9));
        let mut a = vector(1, 2);
        a += (6, 7);
        assert!(a == (7, 9));
    }

    #[test]
    fn sub_assign_test() {
        let mut a = vector(6, 7);
        let b = vector(1, 2);
        a -= b;
        assert!(a == (5, 5));
        let mut a = vector(6, 7);
        a -= (1, 2);
        assert!(a == (5, 5));
    }

    #[test]
    fn mul_assign_test() {
        let mut a = vector(1, 2);
        a *= 2;
        assert!(a == (2, 4));
    }

    #[test]
    fn div_assign_test() {
        let mut a = vector(3, 6);
        a /= 3;
        assert!(a == (1, 2));
    }
}
