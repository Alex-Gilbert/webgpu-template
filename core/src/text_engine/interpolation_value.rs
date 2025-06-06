#[derive(Debug, Clone)]
pub enum InterpolationValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl InterpolationValue {
    pub fn as_string(&self) -> String {
        match self {
            InterpolationValue::String(s) => s.clone(),
            InterpolationValue::Integer(i) => i.to_string(),
            InterpolationValue::Float(f) => {
                // Format floats nicely (remove trailing zeros)
                if f.fract() == 0.0 {
                    format!("{:.0}", f)
                } else {
                    format!("{:.2}", f)
                        .trim_end_matches('0')
                        .trim_end_matches('.')
                        .to_string()
                }
            }
            InterpolationValue::Boolean(b) => b.to_string(),
        }
    }
}

// Convenient From implementations
impl From<String> for InterpolationValue {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl From<&str> for InterpolationValue {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<i32> for InterpolationValue {
    fn from(i: i32) -> Self {
        Self::Integer(i as i64)
    }
}

impl From<i64> for InterpolationValue {
    fn from(i: i64) -> Self {
        Self::Integer(i)
    }
}

impl From<f32> for InterpolationValue {
    fn from(f: f32) -> Self {
        Self::Float(f as f64)
    }
}

impl From<f64> for InterpolationValue {
    fn from(f: f64) -> Self {
        Self::Float(f)
    }
}

impl From<bool> for InterpolationValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}
