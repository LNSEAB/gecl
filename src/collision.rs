use crate::*;

pub trait Collision<T> {
    fn is_crossing(&self, rhs: &T) -> bool;
    fn contains(&self, v: &T) -> bool;
}

#[inline]
pub fn is_crossing<T: Collision<U>, U>(lhs: &T, rhs: &U) -> bool {
    lhs.is_crossing(rhs)
}

#[inline]
pub fn contains<T: Collision<U>, U>(outer: &T, inner: &U) -> bool {
    outer.contains(inner)
}

impl<T> Collision<Point<T>> for Circle<T>
where
    T: std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + PartialOrd
        + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Point<T>) -> bool {
        let d = self.center - *rhs;
        d.x * d.x + d.y * d.y <= self.radius * self.radius
    }

    #[inline]
    fn contains(&self, v: &Point<T>) -> bool {
        self.is_crossing(v)
    }
}

impl<T> Collision<Circle<T>> for Point<T>
where
    T: std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + PartialOrd
        + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Circle<T>) -> bool {
        rhs.is_crossing(self)
    }
    
    #[inline]
    fn contains(&self, _: &Circle<T>) -> bool {
        false
    }
}

impl<T> Collision<Circle<T>> for Circle<T>
where
    T: std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + PartialOrd
        + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Circle<T>) -> bool {
        let d = self.center - rhs.center;
        let r = self.radius + rhs.radius;
        d.x * d.x + d.y * d.y <= r * r
    }

    #[inline]
    fn contains(&self, v: &Circle<T>) -> bool {
        let d = self.center - v.center;
        let r = self.radius - v.radius;
        d.x * d.x + d.y * d.y <= r * r
    }
}

impl<T> Collision<Rect<T>> for Point<T>
where
    T: std::ops::Add<T, Output = T> + PartialOrd + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Rect<T>) -> bool {
        let ep = rhs.endpoint();
        self.x >= rhs.origin.x && self.x <= ep.x && self.y >= rhs.origin.y && self.y <= ep.y
    }

    #[inline]
    fn contains(&self, _: &Rect<T>) -> bool {
        false
    }
}

impl<T> Collision<Point<T>> for Rect<T>
where
    T: std::ops::Add<T, Output = T> + PartialOrd + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Point<T>) -> bool {
        rhs.is_crossing(self)
    }

    #[inline]
    fn contains(&self, v: &Point<T>) -> bool {
        self.is_crossing(v)
    }
}

impl<T> Collision<Rect<T>> for Rect<T>
where
    T: std::ops::Add<T, Output = T> + PartialOrd + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Rect<T>) -> bool {
        let lhs_ep = self.endpoint();
        let rhs_ep = rhs.endpoint();
        self.origin.x <= rhs_ep.x
            && self.origin.y <= rhs_ep.y
            && lhs_ep.x >= rhs.origin.x
            && lhs_ep.y >= rhs.origin.y
    }

    #[inline]
    fn contains(&self, v: &Rect<T>) -> bool {
        let self_ep = self.endpoint();
        let v_ep = v.endpoint();
        self.origin.x <= v.origin.x
            && self.origin.y <= v.origin.y
            && self_ep.x >= v_ep.x
            && self_ep.y >= v_ep.y
    }
}

impl<T> Collision<Circle<T>> for Rect<T>
where
    T: std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + PartialOrd
        + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Circle<T>) -> bool {
        let r = vector(rhs.radius, rhs.radius);
        let center = rhs.center;
        let origin = self.origin - r;
        let ep = self.endpoint() + r;
        if origin.x > center.x || origin.y > center.y || ep.x < center.x || ep.y < center.y {
            return false;
        }
        let origin = self.origin;
        let ep = self.endpoint();
        let rr = rhs.radius * rhs.radius;
        let dx = origin.x - center.x;
        let dy = origin.y - center.y;
        if origin.x > center.x && origin.y > center.y && dx * dx + dy * dy >= rr {
            return false;
        }
        let dx = ep.x - center.x;
        if ep.x < center.x && origin.y > center.y && dx * dx + dy * dy >= rr {
            return false;
        }
        let dx = origin.x - center.x;
        let dy = ep.y - center.y;
        if origin.x > center.x && ep.y < center.y && dx * dx + dy * dy >= rr {
            return false;
        }
        let dx = ep.x - center.x;
        if ep.x < center.x && ep.y < center.y && dx * dx + dy * dy >= rr {
            return false;
        }
        true
    }

    #[inline]
    fn contains(&self, v: &Circle<T>) -> bool {
        let ep = self.endpoint();
        let left = v.center.x - v.radius;
        let right = v.center.x + v.radius;
        let top = v.center.y - v.radius;
        let bottom = v.center.y + v.radius;
        left >= self.origin.x && right <= ep.x && top >= self.origin.y && bottom <= ep.y
    }
}

impl<T> Collision<Rect<T>> for Circle<T>
where
    T: std::ops::Add<T, Output = T>
        + std::ops::Sub<T, Output = T>
        + std::ops::Mul<T, Output = T>
        + PartialOrd
        + Copy,
{
    #[inline]
    fn is_crossing(&self, rhs: &Rect<T>) -> bool {
        rhs.is_crossing(self)
    }

    #[inline]
    fn contains(&self, v: &Rect<T>) -> bool {
        self.is_crossing(&v.origin) && self.is_crossing(&v.endpoint())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_point_is_crossing() {
        let a = circle((10, 10), 5);
        assert!(is_crossing(&a, &point(5, 10)));
        assert!(is_crossing(&a, &point(15, 10)));
        assert!(is_crossing(&a, &point(10, 5)));
        assert!(is_crossing(&a, &point(10, 15)));
        assert!(is_crossing(&a, &point(10, 10)));
        assert!(!is_crossing(&a, &point(1, 1)));
        assert!(!is_crossing(&a, &point(10, 16)));
        assert!(is_crossing(&point(5, 10), &a));
    }
    
    #[test]
    fn circle_circle_is_crossing() {
        let a = circle((10, 10), 5);
        assert!(is_crossing(&a, &circle((20, 10), 5)));
        assert!(!is_crossing(&a, &circle((20, 10), 4)));
        assert!(is_crossing(&circle((20, 10), 5), &a));
    }
    
    #[test]
    fn rect_point_is_crossing() {
        let a = rect((10, 10), (10, 10));
        assert!(is_crossing(&a, &point(10, 10)));
        assert!(is_crossing(&a, &point(20, 10)));
        assert!(is_crossing(&a, &point(10, 20)));
        assert!(is_crossing(&a, &point(20, 20)));
        assert!(is_crossing(&a, &point(15, 15)));
        assert!(!is_crossing(&a, &point(9, 10)));
        assert!(is_crossing(&point(15, 15), &a));
    }
    
    #[test]
    fn rect_rect_is_crossing() {
        let a = rect((10, 10), (10, 10));
        assert!(is_crossing(&a, &rect((15, 15), (10, 10))));
        assert!(is_crossing(&a, &rect((0, 0), (10, 10))));
        assert!(is_crossing(&a, &rect((0, 20), (10, 10))));
        assert!(is_crossing(&a, &rect((20, 00), (10, 10))));
        assert!(is_crossing(&a, &rect((20, 20), (10, 10))));
        assert!(!is_crossing(&a, &rect((20, 30), (10, 10))));
    }
    
    #[test]
    fn rect_circle_is_crossing() {
        let a = rect((10, 10), (10, 10));
        assert!(is_crossing(&a, &circle((5, 10), 5)));
        assert!(is_crossing(&a, &circle((5, 20), 5)));
        assert!(is_crossing(&a, &circle((25, 10), 5)));
        assert!(is_crossing(&a, &circle((25, 20), 5)));
        assert!(is_crossing(&a, &circle((10, 5), 5)));
        assert!(is_crossing(&a, &circle((20, 5), 5)));
        assert!(is_crossing(&a, &circle((10, 25), 5)));
        assert!(is_crossing(&a, &circle((20, 25), 5)));
        assert!(!is_crossing(&a, &circle((20, 25), 4)));
        assert!(is_crossing(&circle((20, 25), 5), &a));
    }

    #[test]
    fn circle_contains_point() {
        let a = circle((10, 10), 5);
        assert!(contains(&a, &point(5, 10)));
        assert!(contains(&a, &point(15, 10)));
        assert!(contains(&a, &point(10, 5)));
        assert!(contains(&a, &point(10, 15)));
        assert!(contains(&a, &point(10, 10)));
        assert!(!contains(&a, &point(1, 1)));
        assert!(!contains(&a, &point(10, 16)));
    }
    
    #[test]
    fn point_contains_circle() {
        let a = point(10, 10);
        assert!(!contains(&a, &circle((10, 10), 5)));
    }

    #[test]
    fn point_contains_rect() {
        let a = point(10, 10);
        assert!(!contains(&a, &rect((10, 10), (11, 11))));
    }

    #[test]
    fn rect_contains_point() {
        let a = rect((10, 10), (10, 10));
        assert!(contains(&a, &point(10, 10)));
        assert!(contains(&a, &point(20, 10)));
        assert!(contains(&a, &point(10, 20)));
        assert!(contains(&a, &point(20, 20)));
        assert!(contains(&a, &point(15, 15)));
        assert!(!contains(&a, &point(9, 10)));
    }

    #[test]
    fn rect_contains_rect() {
        let a = rect((10, 10), (10, 10));
        assert!(contains(&a, &rect((10, 10), (10, 10))));
        assert!(contains(&a, &rect((10, 10), (1, 1))));
        assert!(contains(&a, &rect((19, 10), (1, 1))));
        assert!(contains(&a, &rect((10, 19), (1, 1))));
        assert!(contains(&a, &rect((19, 19), (1, 1))));
        assert!(contains(&a, &rect((15, 15), (1, 1))));
        assert!(!contains(&a, &rect((9, 9), (1, 1))));
        assert!(!contains(&a, &rect((20, 20), (1, 1))));
    }

    #[test]
    fn rect_contains_circle() {
        let a = rect((10, 10), (10, 10));
        assert!(contains(&a, &circle((15, 15), 5)));
        assert!(contains(&a, &circle((11, 11), 1)));
        assert!(contains(&a, &circle((19, 11), 1)));
        assert!(contains(&a, &circle((11, 19), 1)));
        assert!(contains(&a, &circle((19, 19), 1)));
        assert!(contains(&a, &circle((15, 15), 1)));
    }

    #[test]
    fn circle_contains_rect() {
        let a = circle((10, 10), 5);
        assert!(contains(&a, &rect((8, 8), (3, 3))));
        assert!(!contains(&a, &rect((5, 5), (3, 3))));
        assert!(!contains(&a, &rect((8, 8), (7, 7))));
    }
}
