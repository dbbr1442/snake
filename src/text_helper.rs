use macroquad::prelude::*;

/// font, font size, text center x, text center y, text
pub fn text_helper(font: Option<&Font>, size: u16, x: f32, y: f32, text: &str) {
        let text_params = TextParams {
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            rotation: 0.0,
            color: WHITE,
            font: font,
            font_size: size,
        };

        let center = get_text_center(text, font, size, 1.0, 0.0);
        draw_text_ex(text, x-center.x, y-center.y, text_params);
}
