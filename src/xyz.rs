/// A color from the CIE 1931 XYZ color space.
///
/// It is assumed that the color’s illuminant and observer are the standard D65 and 2-degree.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Xyz {
    /// A mixture of cone cell response curves chosen by the CIE to be nonnegative.
    /// Ranges from 0 to 0.95047.
    pub x: f32,
    /// Lightness of the color.
    /// 0 is complete black, 1 is the brightest white.
    pub y: f32,
    /// Roughly a measure of the blueness of the color.
    /// Ranges from 0 (no blue) to 1.08883 (maxiumum blue).
    pub z: f32,
}