mod palettes;

use palette::{rgb::Rgb, Srgb};

use palettes::{ALL, COLORS_PER_PALETTE};

#[derive(Debug)]
pub struct Palette {
    pub colors: [Srgb<u8>; COLORS_PER_PALETTE],
}

impl Palette {
    fn new(colors: [Srgb<u8>; COLORS_PER_PALETTE]) -> Self {
        Palette { colors }
    }
}

pub fn all_hex() -> Vec<[u32; COLORS_PER_PALETTE]> {
    ALL.to_vec()
}

pub fn all() -> Vec<Palette> {
    ALL.iter()
        .map(|colors| Palette::new(colors.map(Rgb::from)))
        .collect()
}
