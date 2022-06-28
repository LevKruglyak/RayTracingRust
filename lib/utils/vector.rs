use num::*;
use std::ops::*;

// Vector types
pub type Vec3f = Vector3<f32>;
pub type Vec2f = Vector2<f32>;

pub type Vec3i = Vector3<i32>;
pub type Vec2i = Vector2<i32>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T>
where
    T: Num + Copy,
{
    #[inline]
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(*self)
    }
}

impl<T> Vector3<T>
where
    T: Float,
{
    #[inline]
    pub fn magnitude(&self) -> T{
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    #[inline]
    pub fn abs_dot(&self, rhs: Self) -> T {
        self.dot(rhs).abs()
    }

    #[inline]
    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    #[inline]
    /// Returns an angle to another vector in radians
    pub fn angle_to(&self, rhs: Self) -> T {
        (self.dot(rhs) / (self.magnitude2() * self.magnitude2()).sqrt()).acos()
    }

    #[inline]
    pub fn cross(&self, rhs: Self) -> Self {
        // All this casting to avoid floating point errors
        Self {
            x: T::from(self.y.to_f64().unwrap() * rhs.z.to_f64().unwrap() - self.z.to_f64().unwrap() * rhs.y.to_f64().unwrap()).unwrap(),
            y: T::from(self.z.to_f64().unwrap() * rhs.x.to_f64().unwrap() - self.x.to_f64().unwrap() * rhs.z.to_f64().unwrap()).unwrap(),
            z: T::from(self.x.to_f64().unwrap() * rhs.y.to_f64().unwrap() - self.y.to_f64().unwrap() * rhs.x.to_f64().unwrap()).unwrap(),
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign for Vector3<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T> Mul for Vector3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> MulAssign for Vector3<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// Vector2

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T>
where
    T: Num + Copy,
{
    #[inline]
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    #[inline]
    pub fn magnitude2(&self) -> T {
        self.dot(*self)
    }
}

impl<T> Vector2<T>
where
    T: Float,
{
    #[inline]
    pub fn magnitude(&self) -> T{
        self.magnitude2().sqrt()
    }

    #[inline]
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    #[inline]
    pub fn abs_dot(&self, rhs: Self) -> T {
        self.dot(rhs).abs()
    }

    #[inline]
    pub fn has_nans(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    #[inline]
    /// Returns an angle to another vector in radians
    pub fn angle_to(&self, rhs: Self) -> T {
        (self.dot(rhs) / (self.magnitude2() * self.magnitude2()).sqrt()).acos()
    }
}

impl<T> Add for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Vector2<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Mul for Vector2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> MulAssign for Vector2<T>
where
    T: MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> Div<T> for Vector2<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
