use freetype::{face::LoadFlag, Face};

use super::Error;

pub struct Metrics {
    pub ascent: u16,
    pub descent: u16,
    pub height: u32
}

pub fn determine_metrics_from_font(face: &Face) -> Result<Metrics, Error> {
    let metrics = face.size_metrics().ok_or(Error::MetricsError);
    let y_scale = metrics?.y_scale as f32 / 65536.0; // from fixed
    let ascender = ((face.ascender() as f32 * y_scale) as i32) >> 6;
    let descender = -((face.descender() as f32 * y_scale) as i32) >> 6;
    Ok(Metrics {
        ascent: ascender as u16,
        descent: descender as u16,
        height: (ascender + descender) as u32
    })
}

pub fn determine_metrics_from_render(face: &Face, chars: &[u8]) -> Result<Metrics, Error> {    
    let mut max_ascent: u16 = 0;
    let mut max_descent: u16 = 0;
    for code in chars {
        face.load_char(*code as usize, LoadFlag::RENDER)
            .expect("Unable to load character.");
        let glyph = face.glyph();
        let height = (glyph.metrics().height >> 6) as i32;
        let top = glyph.bitmap_top();
        let ascent = glyph.bitmap_top();

        if ascent > 0 && ascent as u16 > max_ascent {
            max_ascent = ascent as u16;
        }

        if height >= top {
            let descent = (height - ascent) as u32;
    
            if descent > max_descent as u32 {
                max_descent = descent as u16;
            }
        }
    }

    Ok(Metrics {
        ascent: max_ascent,
        descent: max_descent,
        height: (max_ascent + max_descent) as u32
    })
}
