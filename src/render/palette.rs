use iced::widget::shader::wgpu;
use crate::render::color::RGB;

#[derive(Debug, Clone)]
pub struct Palette {
    colors: Vec<RGB>,
}


impl Default for Palette {
    fn default() -> Self {
        Self { colors: vec![
                RGB::new(72, 132, 90),
                RGB::new(163, 186, 112),
                RGB::new(51, 81, 69),
                RGB::new(254, 240, 134),
                RGB::new(95, 155, 252),
                RGB::new(244, 164, 231),
                RGB::new(170, 137, 190),
            ]
        }
    }
}

impl Into<Vec<wgpu::Color>> for Palette {
    fn into(self) -> Vec<wgpu::Color> {
        self.colors.into_iter().map(Into::<wgpu::Color>::into).collect()
    }
}

/* impl Palette {
    pub fn meow() {}
} */
