use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<T> {
    pub origin: Point<T>,
    pub size: Size<T>,
}

impl<T> Rect<T> {
    #[inline]
    pub fn new(origin: impl Into<Point<T>>, size: impl Into<Size<T>>) -> Self {
        Self {
            origin: origin.into(),
            size: size.into(),
        }
    }
}

impl<T> Rect<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    pub fn endpoint(&self) -> Point<T> {
        self.origin + self.size
    }
}

impl<T> Rect<T>
where
    T: std::ops::Sub<T, Output = T> + Copy + PartialOrd,
{
    #[inline]
    pub fn from_points(a: impl Into<Point<T>>, b: impl Into<Point<T>>) -> Self {
        let a = a.into();
        let b = b.into();
        let (t, u) = {
            let (tx, ux) = (a.x < b.x).then(|| (a.x, b.x)).unwrap_or((b.x, a.x));
            let (ty, uy) = (a.y < b.y).then(|| (a.y, b.y)).unwrap_or((b.y, a.y));
            (point(tx, ty), point(ux, uy))
        };
        Self::new(t, u - t)
    }
}

impl<T> Rect<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    #[inline]
    pub fn translate(&self, d: impl Into<Vector<T>>) -> Self {
        let d = d.into();
        Self::new(self.origin + d, self.size)
    }
}

impl<T> Rect<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    #[inline]
    pub fn scale(&self, x: T, y: T) -> Self {
        Self::new(self.origin, (self.size.width * x, self.size.height * y))
    }
}

impl<T: ToPrimitive> Rect<T> {
    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Rect<U>> {
        Some(Rect::new(self.origin.cast::<U>()?, self.size.cast::<U>()?))
    }
}

impl<T> From<((T, T), (T, T))> for Rect<T> {
    #[inline]
    fn from(src: ((T, T), (T, T))) -> Self {
        Self::new(src.0, src.1)
    }
}

impl<T: Copy> From<([T; 2], [T; 2])> for Rect<T> {
    #[inline]
    fn from(src: ([T; 2], [T; 2])) -> Self {
        Self::new(src.0, src.1)
    }
}

#[inline]
pub fn rect<T>(point: impl Into<Point<T>>, size: impl Into<Size<T>>) -> Rect<T> {
    Rect::new(point, size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        assert!(rect((10, 20), (30, 40)) == rect((10, 20), (30, 40)));
    }

    #[test]
    fn from_points_test() {
        let rc = Rect::from_points((10, 20), (30, 40));
        assert!(rc == rect((10, 20), (20, 20)));
        assert!(rc.endpoint() == (30, 40));
    }

    #[test]
    fn translate_test() {
        assert!(rect((10, 20), (30, 40)).translate((1, 2)) == rect((11, 22), (30, 40)));
    }

    #[test]
    fn scale_test() {
        assert!(rect((10, 20), (30, 40)).scale(2, 3) == rect((10, 20), (60, 120)));
    }

    #[test]
    fn from_test() {
        let rc = Rect::from(((10, 20), (30, 40)));
        assert!(rc == rect((10, 20), (30, 40)));
        let rc = Rect::from(([10, 20], [30, 40]));
        assert!(rc == rect((10, 20), (30, 40)));
    }
}
