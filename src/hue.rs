/// The hue of a color.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hue {
    pub(crate) unnormalized_radians: f32,
}

impl Hue {
    /// Creates a new `Hue` from a hue in degrees (from 0 to 360).
    pub fn from_degrees(degrees: f32) -> Self {
        let unnormalized_degrees = if degrees > 180.0 {
            degrees - 360.0
        } else {
            degrees
        };

        Self {
            unnormalized_radians: unnormalized_degrees.to_radians(),
        }
    }

    /// The hue in degrees (from 0 to 360).
    pub fn to_degrees(self) -> f32 {
        let unnormalized_degrees = self.unnormalized_radians.to_degrees();

        if unnormalized_degrees < 0.0 {
            unnormalized_degrees + 360.0
        } else {
            unnormalized_degrees
        }
    }
}
