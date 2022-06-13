use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline]
    pub fn map<R>(self, mut f: impl FnMut(T) -> R) -> Point<R> {
        Point::new(f(self.x), f(self.y))
    }
}

impl<T: ToPrimitive> Point<T> {
    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Point<U>> {
        Some(Point::new(U::from(self.x)?, U::from(self.y)?))
    }
}

impl<T> From<(T, T)> for Point<T> {
    #[inline]
    fn from(src: (T, T)) -> Point<T> {
        Point::new(src.0, src.1)
    }
}

impl<T: Copy> From<[T; 2]> for Point<T> {
    #[inline]
    fn from(src: [T; 2]) -> Point<T> {
        Point::new(src[0], src[1])
    }
}

impl<T> From<Vector<T>> for Point<T> {
    #[inline]
    fn from(src: Vector<T>) -> Point<T> {
        Point::new(src.x, src.y)
    }
}

impl<T> PartialEq<(T, T)> for Point<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<T> PartialEq<[T; 2]> for Point<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T; 2]) -> bool {
        self.x == other[0] && self.y == other[1]
    }
}

impl<T> PartialEq<Point<T>> for (T, T)
where
    T: PartialEq,
{
    fn eq(&self, other: &Point<T>) -> bool {
        self.0 == other.x && self.1 == other.y
    }
}

impl<T> PartialEq<Point<T>> for [T; 2]
where
    T: PartialEq,
{
    fn eq(&self, other: &Point<T>) -> bool {
        self[0] == other.x && self[1] == other.y
    }
}

impl<T> std::ops::Add<Point<T>> for Point<T>
where
    T: std::ops::Add<T, Output = T>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: Point<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T, U> std::ops::Add<U> for Point<T>
where
    T: std::ops::Add<T, Output = T>,
    U: Into<Size<T>>,
{
    type Output = Self;

    #[inline]
    fn add(self, rhs: U) -> Self {
        let rhs = rhs.into();
        Self::new(self.x + rhs.width, self.y + rhs.height)
    }
}

impl<T> std::ops::Sub<Point<T>> for Point<T>
where
    T: std::ops::Sub<T, Output = T>,
{
    type Output = Vector<T>;

    #[inline]
    fn sub(self, rhs: Point<T>) -> Vector<T> {
        Vector::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> std::ops::Sub<Vector<T>> for Point<T>
where
    T: std::ops::Sub<T, Output = T>,
{
    type Output = Point<T>;

    #[inline]
    fn sub(self, rhs: Vector<T>) -> Point<T> {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> std::ops::Mul<T> for Point<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Point<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> std::ops::Div<T> for Point<T>
where
    T: std::ops::Div<T, Output = T> + Copy,
{
    type Output = Point<T>;

    #[inline]
    fn div(self, rhs: T) -> Self {
        Point::new(self.x / rhs, self.y / rhs)
    }
}

impl<T, U> std::ops::AddAssign<U> for Point<T>
where
    T: std::ops::AddAssign<T>,
    U: Into<Size<T>>,
{
    #[inline]
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        self.x += rhs.width;
        self.y += rhs.height;
    }
}

impl<T> std::ops::SubAssign<Vector<T>> for Point<T>
where
    T: std::ops::SubAssign<T>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: Vector<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> std::ops::MulAssign<T> for Point<T>
where
    T: std::ops::MulAssign<T> + Copy,
{
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T> std::ops::DivAssign<T> for Point<T>
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
pub fn point<T>(x: T, y: T) -> Point<T> {
    Point::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        assert!(point(1, 2).map(|x| x + 1) == point(2, 3));
    }

    #[test]
    fn eq_test() {
        assert!(point(1, 2) == point(1, 2));
        assert!(point(1, 2) == (1, 2));
        assert!(point(1, 2) == [1, 2]);
        assert!((1, 2) == point(1, 2));
        assert!([1, 2] == point(1, 2));
    }

    #[test]
    fn add_test() {
        let a = point(1, 2);
        let b = point(6, 7);
        let c = a + b;
        assert!(c == (7, 9));
        let a = point(1, 2);
        let b = size(6, 7);
        let c = a + b;
        assert!(c == (7, 9));
        let a = point(1, 2);
        let b = vector(6, 7);
        let c = a + b;
        assert!(c == (7, 9));
        let c = a + (6, 7);
        assert!(c == (7, 9));
    }

    #[test]
    fn sub_test() {
        let a = point(1, 2);
        let b = point(6, 7);
        let c = b - a;
        assert!(c == vector(5, 5));
        let a = point(6, 7);
        let b = vector(1, 2);
        let c = a - b;
        assert!(c == point(5, 5));
    }

    #[test]
    fn mul_test() {
        let a = point(1, 2);
        let b = a * 2;
        assert!(b == (2, 4));
    }

    #[test]
    fn div_test() {
        let a = point(2, 6);
        let b = a / 2;
        assert!(b == (1, 3));
    }

    #[test]
    fn add_assign_test() {
        let mut a = point(1, 2);
        let b = size(6, 7);
        a += b;
        assert!(a == (7, 9));
        let mut a = point(1, 2);
        let b = vector(6, 7);
        a += b;
        assert!(a == (7, 9));
        let mut a = point(1, 2);
        a += (6, 7);
        assert!(a == (7, 9));
    }

    #[test]
    fn sub_assign_test() {
        let mut a = point(6, 7);
        let b = vector(1, 2);
        a -= b;
        assert!(a == (5, 5));
    }

    #[test]
    fn mul_assign_test() {
        let mut a = point(1, 2);
        a *= 2;
        assert!(a == (2, 4));
    }

    #[test]
    fn div_assign_test() {
        let mut a = point(3, 6);
        a /= 3;
        assert!(a == (1, 2));
    }
}
