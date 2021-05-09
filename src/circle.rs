use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Circle<T> {
    pub center: Point<T>,
    pub radius: T,
}

impl<T> Circle<T> {
    #[inline]
    pub fn new(center: impl Into<Point<T>>, radius: T) -> Self {
        Self {
            center: center.into(),
            radius,
        }
    }
}

impl<T: ToPrimitive> Circle<T> {
    #[inline]
    pub fn cast<U: NumCast>(self) -> Option<Circle<U>> {
        Some(Circle::new(self.center.cast::<U>()?, U::from(self.radius)?))
    }
}

impl<T> Circle<T>
where
    T: std::ops::Add<T, Output = T> + Copy,
{
    #[inline]
    pub fn translate(&self, v: impl Into<Vector<T>>) -> Self {
        let v = v.into();
        Self::new(self.center + v, self.radius)
    }
}

impl<T> Circle<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    #[inline]
    pub fn scale(&self, s: T) -> Self {
        Self::new(self.center, self.radius * s)
    }
}

#[inline]
pub fn circle<T>(center: impl Into<Point<T>>, radius: T) -> Circle<T> {
    Circle::new(center, radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_test() {
        assert!(circle((10, 20), 3) == circle((10, 20), 3));
    }

    #[test]
    fn translate_test() {
        assert!(circle((10, 20), 3).translate((1, 2)) == circle((11, 22), 3));
    }

    #[test]
    fn scale_test() {
        assert!(circle((10, 20), 3).scale(2) == circle((10, 20), 6));
    }
}
