use glam::Vec4;

/// An sRGB color with easy conversion methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Srgb {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Srgb {
    /// Create from sRGB values (0.0-1.0)
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create from sRGB bytes (0-255)
    pub fn from_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create from hex color (e.g., 0xFF8040FF for RGBA)
    pub const fn from_hex(hex: u32) -> Self {
        let r = ((hex >> 24) & 0xFF) as f32 / 255.0;
        let g = ((hex >> 16) & 0xFF) as f32 / 255.0;
        let b = ((hex >> 8) & 0xFF) as f32 / 255.0;
        let a = (hex & 0xFF) as f32 / 255.0;
        Self { r, g, b, a }
    }

    /// Convert to linear color
    pub fn to_linear(&self) -> Color {
        Color::from_srgb_color(*self)
    }

    /// Convert to HSV color
    pub fn to_hsv(&self) -> Hsv {
        rgb_to_hsv(self.r, self.g, self.b, self.a)
    }

    /// Get as Vec4 in sRGB space
    pub fn to_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// Get as bytes (0-255)
    pub fn to_bytes(&self) -> [u8; 4] {
        [
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8,
            (self.a * 255.0) as u8,
        ]
    }

    /// Set alpha channel
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self { r: self.r, g: self.g, b: self.b, a: alpha }
    }

    /// Naive sRGB blending (fast but mathematically incorrect)
    pub fn blend_naive(&self, other: &Srgb, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r * (1.0 - t) + other.r * t,
            g: self.g * (1.0 - t) + other.g * t,
            b: self.b * (1.0 - t) + other.b * t,
            a: self.a * (1.0 - t) + other.a * t,
        }
    }

    /// Linear space blending (correct)
    pub fn blend_linear(&self, other: &Srgb, t: f32) -> Self {
        self.to_linear().blend(&other.to_linear(), t).to_srgb()
    }

    /// HSV space blending (good for hue transitions)
    pub fn blend_hsv(&self, other: &Srgb, t: f32) -> Self {
        self.to_hsv().blend(&other.to_hsv(), t).to_srgb()
    }

    /// HSV blending with shortest hue path
    pub fn blend_hsv_shortest(&self, other: &Srgb, t: f32) -> Self {
        self.to_hsv().blend_shortest_hue(&other.to_hsv(), t).to_srgb()
    }

    /// Common sRGB colors
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { r: 1.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const CYAN: Self = Self { r: 0.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { r: 1.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

/// An HSV color with easy conversion methods
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hsv {
    pub h: f32, // Hue (0.0-360.0)
    pub s: f32, // Saturation (0.0-1.0)
    pub v: f32, // Value (0.0-1.0)
    pub a: f32, // Alpha (0.0-1.0)
}

impl Hsv {
    /// Create from HSV values
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self { 
            h: h.rem_euclid(360.0), 
            s: s.clamp(0.0, 1.0), 
            v: v.clamp(0.0, 1.0), 
            a: a.clamp(0.0, 1.0) 
        }
    }

    /// Convert to sRGB color
    pub fn to_srgb(&self) -> Srgb {
        hsv_to_rgb(self.h, self.s, self.v, self.a)
    }

    /// Convert to linear color
    pub fn to_linear(&self) -> Color {
        self.to_srgb().to_linear()
    }

    /// Set alpha channel
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self { h: self.h, s: self.s, v: self.v, a: alpha }
    }

    /// Adjust hue by degrees
    pub fn with_hue_offset(&self, degrees: f32) -> Self {
        Self::new(self.h + degrees, self.s, self.v, self.a)
    }

    /// Set saturation
    pub fn with_saturation(&self, saturation: f32) -> Self {
        Self::new(self.h, saturation, self.v, self.a)
    }

    /// Set value/brightness
    pub fn with_value(&self, value: f32) -> Self {
        Self::new(self.h, self.s, value, self.a)
    }

    /// HSV blending (components blended independently)
    pub fn blend(&self, other: &Hsv, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::new(
            self.h * (1.0 - t) + other.h * t,
            self.s * (1.0 - t) + other.s * t,
            self.v * (1.0 - t) + other.v * t,
            self.a * (1.0 - t) + other.a * t,
        )
    }

    /// HSV blending with shortest hue path (better for color wheels)
    pub fn blend_shortest_hue(&self, other: &Hsv, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        
        // Calculate shortest distance between hues
        let mut hue_diff = other.h - self.h;
        if hue_diff > 180.0 {
            hue_diff -= 360.0;
        } else if hue_diff < -180.0 {
            hue_diff += 360.0;
        }
        
        Self::new(
            self.h + hue_diff * t,
            self.s * (1.0 - t) + other.s * t,
            self.v * (1.0 - t) + other.v * t,
            self.a * (1.0 - t) + other.a * t,
        )
    }

    /// sRGB blending (convert to sRGB, blend, convert back)
    pub fn blend_srgb(&self, other: &Hsv, t: f32) -> Self {
        self.to_srgb().blend_naive(&other.to_srgb(), t).to_hsv()
    }

    /// Linear space blending
    pub fn blend_linear(&self, other: &Hsv, t: f32) -> Self {
        self.to_linear().blend(&other.to_linear(), t).to_srgb().to_hsv()
    }

    /// Common HSV colors
    pub const RED: Self = Self { h: 0.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const YELLOW: Self = Self { h: 60.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const GREEN: Self = Self { h: 120.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const CYAN: Self = Self { h: 180.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const BLUE: Self = Self { h: 240.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const MAGENTA: Self = Self { h: 300.0, s: 1.0, v: 1.0, a: 1.0 };
    pub const WHITE: Self = Self { h: 0.0, s: 0.0, v: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { h: 0.0, s: 0.0, v: 0.0, a: 1.0 };
}

/// A color type that handles sRGB/linear conversions for shader use
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Internal storage in linear RGB space
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    /// Create from sRGB values (0.0-1.0)
    pub fn from_srgb(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self::from_srgb_color(Srgb::new(r, g, b, a))
    }

    /// Create from an Srgb color
    pub fn from_srgb_color(srgb: Srgb) -> Self {
        Self {
            r: srgb_to_linear(srgb.r),
            g: srgb_to_linear(srgb.g),
            b: srgb_to_linear(srgb.b),
            a: srgb.a,
        }
    }

    /// Create from linear RGB values (0.0-1.0)
    pub fn from_linear(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create from sRGB bytes (0-255)
    pub fn from_srgb_bytes(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from_srgb(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Convert to sRGB color
    pub fn to_srgb(&self) -> Srgb {
        Srgb {
            r: linear_to_srgb(self.r),
            g: linear_to_srgb(self.g),
            b: linear_to_srgb(self.b),
            a: self.a,
        }
    }

    /// Convert to HSV color
    pub fn to_hsv(&self) -> Hsv {
        self.to_srgb().to_hsv()
    }

    /// Get as Vec4 in linear space (for shader uniforms)
    pub fn to_linear_vec4(&self) -> Vec4 {
        Vec4::new(self.r, self.g, self.b, self.a)
    }

    /// Get as Vec4 in sRGB space (for UI/debug display)
    pub fn to_srgb_vec4(&self) -> Vec4 {
        Vec4::new(
            linear_to_srgb(self.r),
            linear_to_srgb(self.g),
            linear_to_srgb(self.b),
            self.a,
        )
    }

    /// Get individual linear components
    pub fn linear_rgba(&self) -> (f32, f32, f32, f32) {
        (self.r, self.g, self.b, self.a)
    }

    /// Get individual sRGB components
    pub fn srgb_rgba(&self) -> (f32, f32, f32, f32) {
        (
            linear_to_srgb(self.r),
            linear_to_srgb(self.g),
            linear_to_srgb(self.b),
            self.a,
        )
    }

    /// Multiply by a scalar (useful for brightness/exposure)
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            r: self.r * factor,
            g: self.g * factor,
            b: self.b * factor,
            a: self.a,
        }
    }

    /// Set alpha channel
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: alpha,
        }
    }

    /// Linear space blending (correct for physically-based rendering)
    pub fn blend(&self, other: &Color, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self {
            r: self.r * (1.0 - t) + other.r * t,
            g: self.g * (1.0 - t) + other.g * t,
            b: self.b * (1.0 - t) + other.b * t,
            a: self.a * (1.0 - t) + other.a * t,
        }
    }

    /// sRGB space blending
    pub fn blend_srgb(&self, other: &Color, t: f32) -> Self {
        self.to_srgb().blend_naive(&other.to_srgb(), t).to_linear()
    }

    /// HSV space blending
    pub fn blend_hsv(&self, other: &Color, t: f32) -> Self {
        self.to_hsv().blend(&other.to_hsv(), t).to_linear()
    }

    /// HSV blending with shortest hue path
    pub fn blend_hsv_shortest(&self, other: &Color, t: f32) -> Self {
        self.to_hsv().blend_shortest_hue(&other.to_hsv(), t).to_linear()
    }

    /// Common linear colors
    pub const WHITE: Self = Self { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
    pub const BLACK: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const RED: Self = Self { r: 1.0, g: 0.0, b: 0.0, a: 1.0 };
    pub const GREEN: Self = Self { r: 0.0, g: 1.0, b: 0.0, a: 1.0 };
    pub const BLUE: Self = Self { r: 0.0, g: 0.0, b: 1.0, a: 1.0 };
    pub const TRANSPARENT: Self = Self { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
}

// RGB <-> HSV conversion functions
fn rgb_to_hsv(r: f32, g: f32, b: f32, a: f32) -> Hsv {
    let max = r.max(g.max(b));
    let min = r.min(g.min(b));
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    Hsv::new(h, s, v, a)
}

fn hsv_to_rgb(h: f32, s: f32, v: f32, a: f32) -> Srgb {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Srgb::new(r + m, g + m, b + m, a)
}

// sRGB <-> Linear conversion functions
/// Convert sRGB component to linear
fn srgb_to_linear(srgb: f32) -> f32 {
    if srgb <= 0.04045 {
        srgb / 12.92
    } else {
        ((srgb + 0.055) / 1.055).powf(2.4)
    }
}

/// Convert linear component to sRGB
fn linear_to_srgb(linear: f32) -> f32 {
    if linear <= 0.0031308 {
        linear * 12.92
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    }
}

// Conversions between color types
impl From<Srgb> for Color {
    fn from(srgb: Srgb) -> Self {
        Self::from_srgb_color(srgb)
    }
}

impl From<Color> for Srgb {
    fn from(color: Color) -> Self {
        color.to_srgb()
    }
}

impl From<Hsv> for Srgb {
    fn from(hsv: Hsv) -> Self {
        hsv.to_srgb()
    }
}

impl From<Srgb> for Hsv {
    fn from(srgb: Srgb) -> Self {
        srgb.to_hsv()
    }
}

impl From<Hsv> for Color {
    fn from(hsv: Hsv) -> Self {
        hsv.to_linear()
    }
}

impl From<Color> for Hsv {
    fn from(color: Color) -> Self {
        color.to_hsv()
    }
}

// Conversions from common types to Srgb
impl From<wgpu::Color> for Srgb {
    fn from(color: wgpu::Color) -> Self {
        Self::new(color.r as f32, color.g as f32, color.b as f32, color.a as f32)
    }
}

impl From<[f32; 4]> for Srgb {
    fn from(rgba: [f32; 4]) -> Self {
        Self::new(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

impl From<[u8; 4]> for Srgb {
    fn from(rgba: [u8; 4]) -> Self {
        Self::from_bytes(rgba[0], rgba[1], rgba[2], rgba[3])
    }
}

impl From<u32> for Srgb {
    fn from(hex: u32) -> Self {
        Self::from_hex(hex)
    }
}

// Conversions from common types to Color
impl From<wgpu::Color> for Color {
    fn from(color: wgpu::Color) -> Self {
        Srgb::from(color).into()
    }
}

impl From<[f32; 4]> for Color {
    fn from(rgba: [f32; 4]) -> Self {
        Srgb::from
