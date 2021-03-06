//! A crate for converting colors between different color spaces.
//!
//! Color spaces can be converted between one another using [`convert`]:
//!
//! ```
//! use tincture::{LinearRgb, Oklab};
//!
//! let rebeccapurple = LinearRgb {
//!     r: 0.4,
//!     g: 0.2,
//!     b: 0.6,
//! };
//!
//! let oklab: Oklab = tincture::convert(rebeccapurple);
//!
//! assert_eq!(
//!     oklab,
//!     Oklab {
//!         l: 0.66066486,
//!         a: 0.079970956,
//!         b: -0.095915854,
//!     },
//! );
//! ```
//!
//! Variations on the core color spaces do not implement [`CoreColorSpace`], which is necessary for [`convert`].
//! Instead, they implement `From<ACoreColorSpace>`, allowing you to convert this variation to its corresponding core color space and call [`convert`].
//! Examples of variations include [`Oklch`] (a variation on [`Oklab`]) and [`Srgb`] (a variation on [`LinearRgb`]).
//!
//! ```
//! use tincture::{Hue, LinearRgb, Oklab, Oklch, Srgb};
//!
//! // `Oklch` is a variation on `Oklab` (`Oklch` uses polar coordinates).
//! let peach = Oklch {
//!     l: 0.8,
//!     c: 0.25,
//!     h: Hue::from_degrees(40.0).unwrap(),
//! };
//!
//! // This means we can create an `Oklab` using `From`.
//! let oklab = Oklab::from(peach);
//!
//! // We can now convert `oklab` to any other core color space, such as `LinearRgb`.
//! let linear_rgb: LinearRgb = tincture::convert(oklab);
//!
//! // `Srgb` is a variant of `LinearRgb`, so we again create one using `From`.
//! let srgb = Srgb::from(linear_rgb);
//! ```
//!
//! _All_ color spaces implement [`ColorSpace`], which provides the constants `BLACK` and `WHITE`:
//!
//! ```
//! use tincture::{ColorSpace, Srgb};
//!
//! assert_eq!(Srgb::BLACK, Srgb { r: 0.0, g: 0.0, b: 0.0 });
//! assert_eq!(Srgb::WHITE, Srgb { r: 1.0, g: 1.0, b: 1.0 });
//! ```
//!
//! [`ColorSpace`] also provides the [`in_bounds`] method:
//!
//! ```
//! use tincture::{ColorSpace, Srgb};
//!
//! let out_of_bounds = Srgb {
//!     r: 2.0,
//!     g: -100.0,
//!     b: 0.5,
//! };
//!
//! let in_bounds = Srgb {
//!     r: 0.25,
//!     g: 0.75,
//!     b: 0.25,
//! };
//!
//! assert!(!out_of_bounds.in_bounds());
//! assert!(in_bounds.in_bounds());
//! ```

#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]
#![allow(clippy::excessive_precision)]

mod hex;
mod hue;
mod linear_rgb;
mod oklab;
mod oklch;
mod srgb;
mod xyz;

pub use hex::Hex;
pub use hue::Hue;
pub use linear_rgb::LinearRgb;
pub use oklab::Oklab;
pub use oklch::Oklch;
pub use srgb::Srgb;
pub use xyz::Xyz;

/// A color space that can be converted to any other `CoreColorSpace`.
pub trait CoreColorSpace {
    /// Convert a color in the XYZ color space to the color space that `Self` represents.
    fn from_xyz(xyz: Xyz) -> Self;

    /// Convert the color of `Self` to the XYZ color space.
    fn to_xyz(self) -> Xyz;
}

/// A color space.
pub trait ColorSpace {
    /// The color ‘black’.
    const BLACK: Self;

    /// The color ‘white’.
    const WHITE: Self;

    /// Checks if the color is in bounds.
    fn in_bounds(self) -> bool;
}

/// Convert a color from one color space to another.
pub fn convert<In: CoreColorSpace, Out: CoreColorSpace>(color: In) -> Out {
    let xyz = color.to_xyz();
    Out::from_xyz(xyz)
}

fn approx_in_range(n: f32, range: std::ops::Range<f32>) -> bool {
    let fudged_range = range.start - 0.005..range.end + 0.005;
    fudged_range.contains(&n)
}
