use crate::render::color::RGBA;
use iced::widget::shader::wgpu;

#[derive(Debug, Clone)]
pub struct Palette {
    pub colors: Vec<RGBA>,
}

// impl Default for Palette {
//     fn default() -> Self {
//         Self {
//             colors: vec![
//                 RGBA::new(72, 132, 90),
//                 RGBA::new(163, 186, 112),
//                 RGBA::new(51, 81, 69),
//                 RGBA::new(254, 240, 134),
//                 RGBA::new(95, 155, 252),
//                 RGBA::new(244, 164, 231),
//                 RGBA::new(170, 137, 190),
//             ],
//         }
//     }
// }

impl Into<Vec<wgpu::Color>> for Palette {
    fn into(self) -> Vec<wgpu::Color> {
        self.colors
            .into_iter()
            .map(Into::<wgpu::Color>::into)
            .collect()
    }
}

impl Palette {
    fn new(colors: &[&str]) -> Self {
        Self {
            colors: colors.into_iter().map(|&c| c.try_into().unwrap()).collect(),
        }
    }
    // https://lospec.com/palette-list/desatur8
    pub fn desatur8() -> Self {
        Self::new(&[
            "#f0f0eb", "#ffff8f", "#7be098", "#849ad8", "#e8b382", "#d8828e", "#a776c1", "#545155",
        ])
    }

    pub fn dream_haze() -> Self {
        Self::new(&[
            "#3c42c4", "#6e51c8", "#a065cd", "#ce79d2", "#d68fb8", "#dda2a3", "#eac4ae", "#f4dfbe",
        ])
    }
}
