use num_traits::ConstZero;
use num_traits::Float;

/// A wrapper type indicating that the contained value is in radians.
/// `F` should be f32 or f64.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rad<F = f32>(pub F);

/// A wrapper type indicating that the contained value is in degrees.
/// `F` should be f32 or f64.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Deg<F = f32>(pub F);

impl<F: ConstZero> Default for Deg<F> {
    fn default() -> Self {
        Deg(F::ZERO)
    }
}

impl<F: ConstZero> Default for Rad<F> {
    fn default() -> Self {
        Rad(F::ZERO)
    }
}

impl From<Rad> for Rad<f64> {
    fn from(rad: Rad<f32>) -> Self {
        Rad(rad.0 as f64)
    }
}

impl From<Rad<f64>> for Rad {
    fn from(rad: Rad<f64>) -> Self {
        Rad(rad.0 as f32)
    }
}

impl From<Deg<f32>> for Deg<f64> {
    fn from(deg: Deg<f32>) -> Self {
        Deg(deg.0 as f64)
    }
}

impl<F> Rad<F> {
    /// Create a new `Rad` from a floating point value in radians.
    pub fn new(radians: F) -> Self {
        Rad(radians)
    }

    /// Extract the inner floating point value.
    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<F: Float> Rad<F> {
    /// Create a new `Rad` from a `Deg` instance, converting the value to radians.
    pub fn from_deg(deg: Deg<F>) -> Self {
        Rad(deg.0.to_radians())
    }

    /// Convert the value to degrees.
    pub fn to_deg(self) -> Deg<F> {
        Deg(self.0.to_degrees())
    }

    /// Compute the sine of the value.
    pub fn sin(self) -> F {
        self.0.sin()
    }

    /// Compute the cosine of the value.
    pub fn cos(self) -> F {
        self.0.cos()
    }

    /// Compute the tangent of the value.
    pub fn tan(self) -> F {
        self.0.tan()
    }
}

impl<F> Deg<F> {
    /// Create a new `Deg` from a floating point value in degrees.
    pub fn new(degrees: F) -> Self {
        Deg(degrees)
    }

    /// Extract the inner floating point value.
    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<F: Float> Deg<F> {
    /// Create a new `Deg` from a `Rad` instance, converting the value to degrees.
    pub fn from_rad(rad: Rad<F>) -> Self {
        Deg(rad.0.to_degrees())
    }

    /// Convert the value to radians.
    pub fn to_rad(self) -> Rad<F> {
        Rad(self.0.to_radians())
    }

    /// Compute the sine of the value.
    pub fn sin(self) -> F {
        self.0.to_radians().sin()
    }

    /// Compute the cosine of the value.
    pub fn cos(self) -> F {
        self.0.to_radians().cos()
    }

    /// Compute the tangent of the value.
    pub fn tan(self) -> F {
        self.0.to_radians().tan()
    }
}

impl From<f32> for Rad<f32> {
    fn from(value: f32) -> Self {
        Rad(value)
    }
}

impl From<f64> for Rad<f64> {
    fn from(value: f64) -> Self {
        Rad(value)
    }
}

impl From<f64> for Rad<f32> {
    fn from(value: f64) -> Self {
        Rad(value as f32)
    }
}

impl From<Rad<f32>> for f32 {
    fn from(rad: Rad<f32>) -> Self {
        rad.0
    }
}

impl From<Rad<f64>> for f64 {
    fn from(rad: Rad<f64>) -> Self {
        rad.0
    }
}

impl From<f32> for Deg<f32> {
    fn from(value: f32) -> Self {
        Deg(value)
    }
}

impl From<f64> for Deg<f64> {
    fn from(value: f64) -> Self {
        Deg(value)
    }
}

impl From<f64> for Deg<f32> {
    fn from(value: f64) -> Self {
        Deg(value as f32)
    }
}

impl From<Deg<f32>> for f32 {
    fn from(deg: Deg<f32>) -> Self {
        deg.0
    }
}

impl From<Deg<f64>> for f64 {
    fn from(deg: Deg<f64>) -> Self {
        deg.0
    }
}

impl<F: Float> From<Deg<F>> for Rad<F> {
    fn from(deg: Deg<F>) -> Self {
        Rad::from_deg(deg)
    }
}

impl<F: Float> From<Rad<F>> for Deg<F> {
    fn from(rad: Rad<F>) -> Self {
        rad.to_deg()
    }
}

// Add for Rad<F>
impl<F: Float> std::ops::Add for Rad<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Rad(self.0 + other.0)
    }
}

// AddAssign for Rad<F>
impl<F: Float> std::ops::AddAssign for Rad<F> {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(other.0);
    }
}

// Sub for Rad<F>
impl<F: Float> std::ops::Sub for Rad<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Rad(self.0 - other.0)
    }
}

// AddAssign for Rad<F>
impl<F: Float> std::ops::SubAssign for Rad<F> {
    fn sub_assign(&mut self, other: Self) {
        self.0 = self.0.sub(other.0);
    }
}
// Mul<F> for Rad<F>
impl<F: Float> std::ops::Mul<F> for Rad<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Rad(self.0 * rhs)
    }
}

// Mul<Rad<F>> for F, allowing F * Rad
impl std::ops::Mul<Rad<f32>> for f32 {
    type Output = Rad<f32>;

    fn mul(self, rhs: Rad<f32>) -> Self::Output {
        Rad(self * rhs.0)
    }
}

// Mul<Rad<F>> for F, allowing F * Rad
impl std::ops::Mul<Rad<f64>> for f64 {
    type Output = Rad<f64>;

    fn mul(self, rhs: Rad<f64>) -> Self::Output {
        Rad(self * rhs.0)
    }
}

// Add for Deg
impl<F: Float> std::ops::Add for Deg<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Deg(self.0 + other.0)
    }
}

// AddAssign for Deg
impl<F: Float> std::ops::AddAssign for Deg<F> {
    fn add_assign(&mut self, other: Self) {
        self.0 = self.0.add(other.0);
    }
}

// Mul<F> for Deg
impl<F: Float> std::ops::Mul<F> for Deg<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Deg(self.0 * rhs)
    }
}

// Mul<Deg> for F, allowing F * Deg
impl std::ops::Mul<Deg<f32>> for f32 {
    type Output = Deg<f32>;

    fn mul(self, rhs: Deg<f32>) -> Self::Output {
        Deg(self * rhs.0)
    }
}

// Mul<Deg> for F, allowing F * Deg
impl std::ops::Mul<Deg<f64>> for f64 {
    type Output = Deg<f64>;

    fn mul(self, rhs: Deg<f64>) -> Self::Output {
        Deg(self * rhs.0)
    }
}
