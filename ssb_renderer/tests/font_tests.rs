mod font_tests {
    // Imports
    use font_loader::system_fonts;
    use rusttype::{point, Font, PositionedGlyph, Scale};
    use image::{GrayAlphaImage, LumaA};
    use std::path::Path;

    // Native font (post-installed on linux)
    const TEST_FONT: &str = "Arial";

    #[test]
    fn test_has_system_fonts() {
        let fonts = system_fonts::query_all();
        assert!(fonts.len() > 0, "Fonts:\n{:?}", fonts);
    }

    #[test]
    fn test_find_font() {
        let fonts = system_fonts::query_specific(
            &mut system_fonts::FontPropertyBuilder::new().family(TEST_FONT).build()
        );
        assert!(fonts.contains(&TEST_FONT.to_owned()), "Found fonts: {:?}", fonts);
    }

    #[test]
    fn test_font_data() {
        // Load font (regular)
        let (data_regular, _) = system_fonts::get(
            &system_fonts::FontPropertyBuilder::new().family(TEST_FONT).build()
        ).unwrap();
        // Load font (bold)
        let (data_bold, _) = system_fonts::get(
            &system_fonts::FontPropertyBuilder::new().family(TEST_FONT).bold().build()
        ).unwrap();
        // Compare both font styles
        println!("{0} regular length: {1}\n{0} bold length: {2}", TEST_FONT, data_regular.len(), data_bold.len());
        assert!(data_regular.len() > 0 && data_regular.len() != data_bold.len());
    }

    #[test]
    fn test_text_image() {
        // Load font (regular)
        let (data, _) = system_fonts::get(
            &system_fonts::FontPropertyBuilder::new().family(TEST_FONT).build()
        ).unwrap();

        // See <https://gitlab.redox-os.org/redox-os/rusttype/blob/master/examples/simple.rs>

        // Construct font object
        let font = Font::from_bytes(data).expect("Font data should be valid!");
        // Font size
        let font_scale = Scale::uniform(54.7);
        // Get glyphs from text (as vector to not consume items by reading)
        let baseline_point = point(0.0, font.v_metrics(font_scale).ascent); // Glyphs render upwards, so we need a fitting baseline
        let glyphs = font.layout("ssb_renderer", font_scale, baseline_point).collect::<Vec<PositionedGlyph<'_>>>();
        // Calculate text bounding by pixels
        let pixel_height = font_scale.y.ceil() as u32;
        let pixel_width = if let Some(last_glyph) = glyphs.last() {
            (
                last_glyph.position().x +
                last_glyph.unpositioned().h_metrics().advance_width
            ).ceil() as u32
            + 1 // Pass last pixel to get real width
        } else {
            0
        };
        // Fill image pixels
        let mut text_image = GrayAlphaImage::new(pixel_width, pixel_height);
        for glyph in glyphs {
            if let Some(glyph_bounding) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, opacity| {
                    text_image.put_pixel(
                        glyph_bounding.min.x as u32 + x,
                        glyph_bounding.min.y as u32 + y,
                        LumaA {
                            data: [std::u8::MAX >> 1, (opacity * std::u8::MAX as f32) as u8]
                        }
                    );
                });
            }
        }
        // Save image
        text_image.save(
            Path::new(&env!("CARGO_MANIFEST_DIR"))
            .join("../target/text_image.png")
        ).expect("Image saving failed!");
    }
}