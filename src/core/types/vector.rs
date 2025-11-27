use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign
};
/// A simple 2D coordinate type
#[derive(Clone, Copy)]
pub struct Vector2{
    pub x: f32,
    pub y: f32
}
impl Vector2{
    /// Create a new 2D Vector
    pub fn new(x: f32, y: f32) -> Self {
        Self{x, y}
    }
    /// Returns the dot product of `self` and `other`
    /// 
    /// Simpler explanation: How similar the two Vectors are
    /// - Sign defines direction, positive means same direction, negative means opposite direction
    /// - Value defines how similar the magnitude is
    pub fn dot(self, other: Self) -> f32{
        (self.x * other.x) + (self.y * other.y)
    }
    /// Returns a Vector that's a projection of `self` onto `other`
    /// 
    /// Simpler explanation: It flattens `self` onto `other`
    pub fn project(self, other: Self) -> Self{
        let scalar = self.dot(other)/other.magnitude().powi(2);
        self * scalar
    }
    /// Returns a reflection of `self` around `other`
    pub fn reflect(self, other: Self) -> Self{
        self.project(other) * 2.0 - self
    }
    /// Returns distance between endpoint of `self` and `other`
    pub fn distance(self, other: Self) -> f32{
        (self - other).magnitude()
    }
    /// Returns self normalized to magnitude of 1
    pub fn normalize(self) -> Self{
        self / self.magnitude()
    }
    /// Returns the angle between `self` and `other` in Radians
    pub fn angle_between(self, other: Self) -> f32{
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
    /// Returns the length of `self`
    /// 
    /// ## WARNING
    /// Because this uses Pythagorean Theorem to calculate, a Vector too big may cause an overflow, and subsequently a crash
    pub fn magnitude(self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

}
impl std::fmt::Display for Vector2{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.x, self.y))
    }
}
impl Add for Vector2{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl AddAssign for Vector2{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Vector2{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl SubAssign for Vector2{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y
    }
}
impl Mul<f32> for Vector2{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}
impl MulAssign<f32> for Vector2{
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
impl Div<f32> for Vector2{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

/// A simple 3D coordinate type
#[derive(Clone, Copy)]
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vector3{
    /// Create a new 2D Vector
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }
    /// Returns the dot product of `self` and `other`
    /// 
    /// Simpler explanation: How similar the two Vectors are
    /// - Sign defines direction, positive means same direction, negative means opposite direction
    /// - Value defines how similar the magnitude is
    pub fn dot(self, other: Self) -> f32{
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
    /// Returns a Vector that's a projection of `self` onto `other`
    /// 
    /// Simpler explanation: It flattens `self` onto `other`
    pub fn project(self, other: Self) -> Self{
        let scalar = self.dot(other)/other.magnitude().powi(2);
        self * scalar
    }
    /// Returns the cross product Vector of `self` and `other`
    /// 
    /// Simpler explanation: Returns a Vector perpendicular to `self` and `other`, the magnitude of it is inverse of dot product: it defines how *different* the two Vectors are
    pub fn cross(self, other: Self) -> Self{
        Self{
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    /// Returns a reflection of `self` around `other`
    pub fn reflect(self, other: Self) -> Self{
        self.project(other) * 2.0 - self
    }
    /// Returns distance between endpoint of `self` and `other`
    pub fn distance(self, other: Self) -> f32{
        (self - other).magnitude()
    }
    /// Returns self normalized to magnitude of 1
    pub fn normalize(self) -> Self{
        self / self.magnitude()
    }
    /// Returns the angle between `self` and `other` in Radians
    pub fn angle_between(self, other: Self) -> f32{
        (self.dot(other) / (self.magnitude() * other.magnitude())).acos()
    }
    /// Returns the length of `self`
    /// 
    /// ## WARNING
    /// Because this uses Pythagorean Theorem to calculate, a Vector too big may cause an overflow, and subsequently a crash
    pub fn magnitude(self) -> f32{
        (self.x.powi(2) + self.y.powi(2) + self.x.powi(2)).sqrt()
    }
}   
impl std::fmt::Display for Vector3{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {}, {})", self.x, self.y, self.z))
    }
}
impl Add for Vector3{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}
impl AddAssign for Vector3{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z
    }
}
impl Sub for Vector3{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
impl SubAssign for Vector3{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z
    }
}
impl Mul<f32> for Vector3{
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}
impl MulAssign<f32> for Vector3{
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs
    }
}
impl Div<f32> for Vector3{
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self{
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs
        }
    }
}
impl DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs
    }
}