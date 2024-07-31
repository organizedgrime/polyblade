//use three_d::{Srgba, Vec4};
use kas::geom::Vec3;

#[derive(Debug, Default)]
pub struct HSL {
    /// Hue in 0-360 degree
    h: f64,
    /// Saturation in 0...1 (percent)
    s: f64,
    /// Luminosity in 0...1 (percent)
    l: f64,
}

impl HSL {
    // New
    pub fn new(h: f64, s: f64, l: f64) -> Self {
        Self { h, s, l }
    }

    pub fn from_rgb(rgb: &[u8]) -> HSL {
        use std::cmp::{max, min};

        let mut h: f64;

        let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

        let max = max(max(r, g), b);
        let min = min(min(r, g), b);

        // Normalized RGB: Divide everything by 255 to get percentages of colors.
        let (r, g, b) = (r as f64 / 255_f64, g as f64 / 255_f64, b as f64 / 255_f64);
        let (min, max) = (min as f64 / 255_f64, max as f64 / 255_f64);

        // Luminosity is the average of the max and min rgb color intensities.
        let l: f64 = (max + min) / 2_f64;

        // Saturation
        let delta: f64 = max - min;
        if delta == 0_f64 {
            // it's gray
            return HSL {
                h: 0_f64,
                s: 0_f64,
                l,
            };
        }

        // it's not gray
        let s = if l < 0.5_f64 {
            delta / (max + min)
        } else {
            delta / (2_f64 - max - min)
        };

        // Hue
        let r2 = (((max - r) / 6_f64) + (delta / 2_f64)) / delta;
        let g2 = (((max - g) / 6_f64) + (delta / 2_f64)) / delta;
        let b2 = (((max - b) / 6_f64) + (delta / 2_f64)) / delta;

        h = match max {
            x if x == r => b2 - g2,
            x if x == g => (1_f64 / 3_f64) + r2 - b2,
            _ => (2_f64 / 3_f64) + g2 - r2,
        };

        // Fix wraparounds
        if h < 0 as f64 {
            h += 1_f64;
        } else if h > 1_f64 {
            h -= 1_f64;
        }

        // Hue is precise to milli-degrees, e.g. `74.52deg`.
        let h = (h * 360_f64 * 100_f64).round() / 100_f64;
        HSL { h, s, l }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        if self.s == 0.0 {
            // Achromatic, i.e., grey.
            let l = percent_to_byte(self.l);
            return (l, l, l);
        }

        let h = self.h / 360.0; // treat this as 0..1 instead of degrees
        let s = self.s;
        let l = self.l;

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - (l * s)
        };
        let p = 2.0 * l - q;

        (
            percent_to_byte(hue_to_rgb(p, q, h + 1.0 / 3.0)),
            percent_to_byte(hue_to_rgb(p, q, h)),
            percent_to_byte(hue_to_rgb(p, q, h - 1.0 / 3.0)),
        )
    }

    pub fn to_rgb_float(&self) -> Vec3 {
        let rgb = self.to_rgb();
        Vec3(
            rgb.0 as f32 / 255.0,
            rgb.1 as f32 / 255.0,
            rgb.2 as f32 / 255.0,
        )
    }

    /*
    pub fn to_srgba(&self) -> Srgba {
        let color = self.to_rgb();
        Srgba::new_opaque(color.0, color.1, color.2)
    }

    pub fn to_linear_srgb(&self) -> Vec4 {
        let color = self.to_rgb();
        Srgba::new_opaque(color.0, color.1, color.2).to_linear_srgb()
    }
    */
}

fn percent_to_byte(percent: f64) -> u8 {
    (percent * 255.0).round() as u8
}

fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
    // Normalize
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

/*
fn scheme(sides: usize) -> HSL {
    match sides {

    }

}
*/
