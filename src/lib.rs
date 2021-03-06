use palette::{rgb::Rgb, Srgb};

const PALETTE_COUNT: usize = 100;
const COLORS_COUNT: usize = 5;

#[derive(Debug)]
pub struct Palette {
    pub colors: [Srgb<u8>; COLORS_COUNT],
}

impl Palette {
    fn new(colors: [Srgb<u8>; COLORS_COUNT]) -> Self {
        Palette { colors }
    }
}
pub const ALL_PALETTES: [[u32; COLORS_COUNT]; PALETTE_COUNT] = [
    [0x69D2E7, 0xA7DBD8, 0xE0E4CC, 0xF38630, 0xFA6900],
    [0xFE4365, 0xFC9D9A, 0xF9CDAD, 0xC8C8A9, 0x83AF9B],
    [0xECD078, 0xD95B43, 0xC02942, 0x542437, 0x53777A],
    [0x556270, 0x4ECDC4, 0xC7F464, 0xFF6B6B, 0xC44D58],
    [0x774F38, 0xE08E79, 0xF1D4AF, 0xECE5CE, 0xC5E0DC],
    [0xE8DDCB, 0xCDB380, 0x036564, 0x033649, 0x031634],
    [0x490A3D, 0xBD1550, 0xE97F02, 0xF8CA00, 0x8A9B0F],
    [0x594F4F, 0x547980, 0x45ADA8, 0x9DE0AD, 0xE5FCC2],
    [0x00A0B0, 0x6A4A3C, 0xCC333F, 0xEB6841, 0xEDC951],
    [0xE94E77, 0xD68189, 0xC6A49A, 0xC6E5D9, 0xF4EAD5],
    [0x3FB8AF, 0x7FC7AF, 0xDAD8A7, 0xFF9E9D, 0xFF3D7F],
    [0xD9CEB2, 0x948C75, 0xD5DED9, 0x7A6A53, 0x99B2B7],
    [0xFFFFFF, 0xCBE86B, 0xF2E9E1, 0x1C140D, 0xCBE86B],
    [0x343838, 0x005F6B, 0x008C9E, 0x00B4CC, 0x00DFFC],
    [0xEFFFCD, 0xDCE9BE, 0x555152, 0x2E2633, 0x99173C],
    [0x413E4A, 0x73626E, 0xB38184, 0xF0B49E, 0xF7E4BE],
    [0xFF4E50, 0xFC913A, 0xF9D423, 0xEDE574, 0xE1F5C4],
    [0x99B898, 0xFECEA8, 0xFF847C, 0xE84A5F, 0x2A363B],
    [0x655643, 0x80BCA3, 0xF6F7BD, 0xE6AC27, 0xBF4D28],
    [0x00A8C6, 0x40C0CB, 0xF9F2E7, 0xAEE239, 0x8FBE00],
    [0x351330, 0x424254, 0x64908A, 0xE8CAA4, 0xCC2A41],
    [0x554236, 0xF77825, 0xD3CE3D, 0xF1EFA5, 0x60B99A],
    [0x5D4157, 0x838689, 0xA8CABA, 0xCAD7B2, 0xEBE3AA],
    [0xFAD089, 0xFF9C5B, 0xF5634A, 0xED303C, 0x3B8183],
    [0x8C2318, 0x5E8C6A, 0x88A65E, 0xBFB35A, 0xF2C45A],
    [0xF8B195, 0xF67280, 0xC06C84, 0x6C5B7B, 0x355C7D],
    [0xFF4242, 0xF4FAD2, 0xD4EE5E, 0xE1EDB9, 0xF0F2EB],
    [0xD1E751, 0xFFFFFF, 0x000000, 0x4DBCE9, 0x26ADE4],
    [0x1B676B, 0x519548, 0x88C425, 0xBEF202, 0xEAFDE6],
    [0x5E412F, 0xFCEBB6, 0x78C0A8, 0xF07818, 0xF0A830],
    [0xBCBDAC, 0xCFBE27, 0xF27435, 0xF02475, 0x3B2D38],
    [0x452632, 0x91204D, 0xE4844A, 0xE8BF56, 0xE2F7CE],
    [0xEEE6AB, 0xC5BC8E, 0x696758, 0x45484B, 0x36393B],
    [0xF0D8A8, 0x3D1C00, 0x86B8B1, 0xF2D694, 0xFA2A00],
    [0x2A044A, 0x0B2E59, 0x0D6759, 0x7AB317, 0xA0C55F],
    [0xF04155, 0xFF823A, 0xF2F26F, 0xFFF7BD, 0x95CFB7],
    [0xB9D7D9, 0x668284, 0x2A2829, 0x493736, 0x7B3B3B],
    [0xB3CC57, 0xECF081, 0xFFBE40, 0xEF746F, 0xAB3E5B],
    [0xBBBB88, 0xCCC68D, 0xEEDD99, 0xEEC290, 0xEEAA88],
    [0xAAB3AB, 0xC4CBB7, 0xEBEFC9, 0xEEE0B7, 0xE8CAAF],
    [0xA3A948, 0xEDB92E, 0xF85931, 0xCE1836, 0x009989],
    [0x300030, 0x480048, 0x601848, 0xC04848, 0xF07241],
    [0xB6D8C0, 0xC8D9BF, 0xDADABD, 0xECDBBC, 0xFEDCBA],
    [0xE8D5B7, 0x0E2430, 0xFC3A51, 0xF5B349, 0xE8D5B9],
    [0x67917A, 0x170409, 0xB8AF03, 0xCCBF82, 0xE33258],
    [0x607848, 0x789048, 0xC0D860, 0xF0F0D8, 0x604848],
    [0xAB526B, 0xBCA297, 0xC5CEAE, 0xF0E2A4, 0xF4EBC3],
    [0xA8E6CE, 0xDCEDC2, 0xFFD3B5, 0xFFAAA6, 0xFF8C94],
    [0x3E4147, 0xFFFEDF, 0xDFBA69, 0x5A2E2E, 0x2A2C31],
    [0x515151, 0xFFFFFF, 0x00B4FF, 0xEEEEEE, 0x00B4FF],
    [0xFC354C, 0x29221F, 0x13747D, 0x0ABFBC, 0xFCF7C5],
    [0xA7C5BD, 0xE5DDCB, 0xEB7B59, 0xCF4647, 0x524656],
    [0xCC0C39, 0xE6781E, 0xC8CF02, 0xF8FCC1, 0x1693A7],
    [0xDAD6CA, 0x1BB0CE, 0x4F8699, 0x6A5E72, 0x563444],
    [0x1C2130, 0x028F76, 0xB3E099, 0xFFEAAD, 0xD14334],
    [0xEDEBE6, 0xD6E1C7, 0x94C7B6, 0x403B33, 0xD3643B],
    [0x5C323E, 0xA82743, 0xE15E32, 0xC0D23E, 0xE5F04C],
    [0xFDF1CC, 0xC6D6B8, 0x987F69, 0xE3AD40, 0xFCD036],
    [0x230F2B, 0xF21D41, 0xEBEBBC, 0xBCE3C5, 0x82B3AE],
    [0xB9D3B0, 0x81BDA4, 0xB28774, 0xF88F79, 0xF6AA93],
    [0x3A111C, 0x574951, 0x83988E, 0xBCDEA5, 0xE6F9BC],
    [0xA1DBB2, 0xFEE5AD, 0xFACA66, 0xF7A541, 0xF45D4C],
    [0x000000, 0x9F111B, 0xB11623, 0x292C37, 0xCCCCCC],
    [0x382F32, 0xFFEAF2, 0xFCD9E5, 0xFBC5D8, 0xF1396D],
    [0xF6F6F6, 0xE8E8E8, 0x333333, 0x990100, 0xB90504],
    [0x1B325F, 0x9CC4E4, 0xE9F2F9, 0x3A89C9, 0xF26C4F],
    [0x1C0113, 0x6B0103, 0xA30006, 0xC21A01, 0xF03C02],
    [0x5E3929, 0xCD8C52, 0xB7D1A3, 0xDEE8BE, 0xFCF7D3],
    [0xE3DFBA, 0xC8D6BF, 0x93CCC6, 0x6CBDB5, 0x1A1F1E],
    [0x5E9FA3, 0xDCD1B4, 0xFAB87F, 0xF87E7B, 0xB05574],
    [0x951F2B, 0xF5F4D7, 0xE0DFB1, 0xA5A36C, 0x535233],
    [0xC1B398, 0x605951, 0xFBEEC2, 0x61A6AB, 0xACCEC0],
    [0x8DCCAD, 0x988864, 0xFEA6A2, 0xF9D6AC, 0xFFE9AF],
    [0x2D2D29, 0x215A6D, 0x3CA2A2, 0x92C7A3, 0xDFECE6],
    [0xFFEFD3, 0xFFFEE4, 0xD0ECEA, 0x9FD6D2, 0x8B7A5E],
    [0xFF003C, 0xFF8A00, 0xFABE28, 0x88C100, 0x00C176],
    [0x413D3D, 0x040004, 0xC8FF00, 0xFA023C, 0x4B000F],
    [0xEFF3CD, 0xB2D5BA, 0x61ADA0, 0x248F8D, 0x605063],
    [0xF8EDD1, 0xD88A8A, 0x474843, 0x9D9D93, 0xC5CFC6],
    [0x046D8B, 0x309292, 0x2FB8AC, 0x93A42A, 0xECBE13],
    [0xF38A8A, 0x55443D, 0xA0CAB5, 0xCDE9CA, 0xF1EDD0],
    [0xA70267, 0xF10C49, 0xFB6B41, 0xF6D86B, 0x339194],
    [0x9DC9AC, 0xFFFEC7, 0xF56218, 0xFF9D2E, 0x919167],
    [0x4D3B3B, 0xDE6262, 0xFFB88C, 0xFFD0B3, 0xF5E0D3],
    [0xA8A7A7, 0xCC527A, 0xE8175D, 0x474747, 0x363636],
    [0xCFFFDD, 0xB4DEC1, 0x5C5863, 0xA85163, 0xFF1F4C],
    [0x4E395D, 0x827085, 0x8EBE94, 0xCCFC8E, 0xDC5B3E],
    [0x4E4D4A, 0x353432, 0x94BA65, 0x2790B0, 0x2B4E72],
    [0xFFEDBF, 0xF7803C, 0xF54828, 0x2E0D23, 0xF8E4C1],
    [0x0CA5B0, 0x4E3F30, 0xFEFEEB, 0xF8F4E4, 0xA5B3AA],
    [0xAAFF00, 0xFFAA00, 0xFF00AA, 0xAA00FF, 0x00AAFF],
    [0xFFFBB7, 0xA6F6AF, 0x66B6AB, 0x5B7C8D, 0x4F2958],
    [0xEDF6EE, 0xD1C089, 0xB3204D, 0x412E28, 0x151101],
    [0x9D7E79, 0xCCAC95, 0x9A947C, 0x748B83, 0x5B756C],
    [0x805841, 0xDCF7F3, 0xFFFCDD, 0xFFD8D8, 0xF5A2A2],
    [0xFCFEF5, 0xE9FFE1, 0xCDCFB7, 0xD6E6C3, 0xFAFBE3],
    [0x30261C, 0x403831, 0x36544F, 0x1F5F61, 0x0B8185],
    [0x9CDDC8, 0xBFD8AD, 0xDDD9AB, 0xF7AF63, 0x633D2E],
    [0x73C8A9, 0xDEE1B6, 0xE1B866, 0xBD5532, 0x373B44],
    [0xD1313D, 0xE5625C, 0xF9BF76, 0x8EB2C5, 0x615375],
];

pub fn all_hex() -> Vec<[u32; COLORS_COUNT]> {
    ALL_PALETTES.to_vec()
}

pub fn all() -> Vec<Palette> {
    ALL_PALETTES
        .iter()
        .map(|colors| Palette::new(colors.map(Rgb::from)))
        .collect::<Vec<Palette>>()
}
