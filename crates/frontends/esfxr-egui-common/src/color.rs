use egui::Color32;

pub trait Color32Ext {
    /// Blend two colors using an amount between 0 and 1
    fn blend_mix(self, other: Self, amount: f32) -> Self;
}

impl Color32Ext for Color32 {
    fn blend_mix(self, other: Self, amount: f32) -> Self {
        Color32::from_rgb(
            (self.r() as f32 * (1.0 - amount) + other.r() as f32 * amount) as u8,
            (self.g() as f32 * (1.0 - amount) + other.g() as f32 * amount) as u8,
            (self.b() as f32 * (1.0 - amount) + other.b() as f32 * amount) as u8,
        )
    }
}
