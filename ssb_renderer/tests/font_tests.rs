mod font_tests {
    // Imports
    use font_loader::system_fonts;

    #[test]
    fn test_has_system_fonts() {
        assert!(system_fonts::query_all().len() > 0);
    }

    // Windows-only
    #[cfg(windows)]
    mod windows {
        // Imports
        use super::system_fonts as system_fonts;
        use rusttype::{point, Font, PositionedGlyph, Scale};
        use image::{GrayAlphaImage, LumaA};
        use std::path::Path;

        #[test]
        fn test_find_arial() {
            let fonts = system_fonts::query_specific(
                &mut system_fonts::FontPropertyBuilder::new().family("Arial").build()
            );
            assert!(fonts.contains(&"Arial".to_string()));
        }

        #[test]
        fn test_arial_data() {
            // Load Arial (regular)
            let (data_regular, _) = system_fonts::get(
                &system_fonts::FontPropertyBuilder::new().family("Arial").build()
            ).unwrap();
            // Load Arial (bold)
            let (data_bold, _) = system_fonts::get(
                &system_fonts::FontPropertyBuilder::new().family("Arial").bold().build()
            ).unwrap();
            // Compare both Arial styles
            println!("Arial regular length: {}\nArial bold length: {}", data_regular.len(), data_bold.len());
            assert!(data_regular.len() > 0 && data_regular.len() != data_bold.len());
        }

        #[test]
        fn test_text_image() {
            // Load Georgia (regular)
            let (data, _) = system_fonts::get(
                &system_fonts::FontPropertyBuilder::new().family("Georgia").build()
            ).unwrap();

            // See <https://gitlab.redox-os.org/redox-os/rusttype/blob/master/examples/simple.rs>

            // Construct font object
            let font = Font::from_bytes(data).expect("Georgia data should be valid!");
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
}