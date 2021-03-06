/// A color from the CIE 1931 XYZ color space.
///
/// It is assumed that the color’s illuminant and observer are the standard D65 and 2-degree.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Xyz {
    /// A mixture of cone cell response curves chosen by the CIE to be nonnegative.
    /// Ranges from 0 to 0.95047003.
    pub x: f32,
    /// Lightness of the color.
    /// 0 is complete black, 1 is the brightest white.
    pub y: f32,
    /// Roughly a measure of the blueness of the color.
    /// Ranges from 0 (no blue) to 1.08883 (maxiumum blue).
    pub z: f32,
}

impl crate::ColorSpace for Xyz {
    const BLACK: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    const WHITE: Self = Self {
        x: 0.95047003,
        y: 1.0,
        z: 0.108883,
    };

    fn in_bounds(self) -> bool {
        crate::approx_in_range(self.x, 0.0..0.95047003)
            && crate::approx_in_range(self.y, 0.0..1.0)
            && crate::approx_in_range(self.z, 0.0..1.08883)
    }
}

impl crate::CoreColorSpace for Xyz {
    fn from_xyz(xyz: Xyz) -> Self {
        xyz
    }

    fn to_xyz(self) -> Xyz {
        self
    }
}
