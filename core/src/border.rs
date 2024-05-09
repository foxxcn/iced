//! Draw lines around containers.
use crate::{Color, Pixels};

/// A border.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Border {
    /// The color of the border.
    pub color: Color,

    /// The width of the border.
    pub width: f32,

    /// The radius of the border.
    pub radius: Radius,
}

impl Border {
    /// Creates a new default rounded [`Border`] with the given [`Radius`].
    ///
    /// ```
    /// # use iced_core::Border;
    /// #
    /// assert_eq!(Border::rounded(10), Border::default().with_radius(10));
    /// ```
    pub fn rounded(radius: impl Into<Radius>) -> Self {
        Self::default().with_radius(radius)
    }

    /// Updates the [`Color`] of the [`Border`].
    pub fn with_color(self, color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    /// Updates the [`Radius`] of the [`Border`].
    pub fn with_radius(self, radius: impl Into<Radius>) -> Self {
        Self {
            radius: radius.into(),
            ..self
        }
    }

    /// Updates the width of the [`Border`].
    pub fn with_width(self, width: impl Into<Pixels>) -> Self {
        Self {
            width: width.into().0,
            ..self
        }
    }
}

/// The border radii for the corners of a graphics primitive in the order:
/// top-left, top-right, bottom-right, bottom-left.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Radius([f32; 4]);

impl From<f32> for Radius {
    fn from(w: f32) -> Self {
        Self([w; 4])
    }
}

impl From<u8> for Radius {
    fn from(w: u8) -> Self {
        Self::from(f32::from(w))
    }
}

impl From<u16> for Radius {
    fn from(w: u16) -> Self {
        Self::from(f32::from(w))
    }
}

impl From<i32> for Radius {
    fn from(w: i32) -> Self {
        Self::from(w as f32)
    }
}

impl From<[f32; 4]> for Radius {
    fn from(radi: [f32; 4]) -> Self {
        Self(radi)
    }
}

impl From<Radius> for [f32; 4] {
    fn from(radi: Radius) -> Self {
        radi.0
    }
}
