mod parse_tests {
    // Imports
    use ssb_parser::{
        types::{
            ssb::*,
            objects::*
        },
        data::{Ssb, SsbRender}
    };
    use std::{
        collections::HashMap,
        convert::TryFrom,
        env::set_current_dir,
        io::{BufReader, Cursor},
        fs::File
    };


    // Tester
    #[test]
    fn test_ssb_simple() {
        // Parse
        let ssb = Ssb::default().parse_owned(Cursor::new(
"
#Info
Author: Youka

#Target
Width: 123

#Macros
foo: bar

#Events
0-1::.|foo|I'm a note!|[color=123abc]Hello world!

#Resources
Font: bar,bold,dXNhZ2k=
Texture: Fancy,data,RmFuY3k=
"
        )).unwrap();
        // Asserts
        assert_eq!(ssb.info_title, None);
        assert_eq!(ssb.info_author, Some("Youka".to_owned()));
        assert_eq!(ssb.target_width, Some(123));
        assert_eq!(ssb.target_height, None);
        assert_eq!(ssb.macros.get("foo"), Some(&"bar".to_owned()));
        assert_eq!(ssb.macros.get("abc"), None);
        let event = ssb.events.get(0).expect("One event expected!");
        assert_eq!(event.trigger, EventTrigger::Time((0, 3600000)));
        assert_eq!(event.data, "[color=123abc]Hello world!");
        assert_eq!(ssb.fonts.get(&FontFace {family: "bar".to_owned(), style: FontStyle::Bold}), Some(&vec![117, 115, 97, 103, 105]));
        assert_eq!(ssb.fonts.get(&FontFace {family: "".to_owned(), style: FontStyle::Regular}), None);
        assert_eq!(ssb.textures.get("Fancy"), Some(&TextureDataVariant::Raw(vec![70, 97, 110, 99, 121])));
        assert_eq!(ssb.textures.get("Nobody"), None);
    }

    #[test]
    fn test_ssb_complex() {
        // Parse 1st phase
        let mut ssb = Ssb::default();
        ssb.parse(
            BufReader::new(
                File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test.ssb"))
                .expect("Test SSB file must exist!")
            )
        ).unwrap_or_else(|exception| panic!("SSB parsing error: {}", exception) );
        //println!("{:#?}", ssb);
        // Parse 2nd phase
        set_current_dir(env!("CARGO_MANIFEST_DIR")).expect("Working directory couldn't set to manifest location?!");
        assert_eq!(
            SsbRender::try_from(ssb).unwrap_or_else(|exception| panic!("SSB render data error: {}", exception) ),
            SsbRender {
                target_width: Some(1280),
                target_height: Some(720),
                target_depth: 800,
                target_view: View::Orthogonal,
                events: vec![
                    EventRender {
                        trigger: EventTrigger::Time((2000,300000)),
                        objects: vec![
                            EventObject::TagPosition(Point3D {
                                x: 100.0,
                                y: 200.0,
                                z: -1.0
                            }),
                            EventObject::TagRotate(Rotate::Z(
                                180.0
                            )),
                            EventObject::TagBold(
                                false
                            ),
                            EventObject::TagColor(Color::Mono([
                                255, 0, 0
                            ])),
                            EventObject::GeometryText(
                                "I\'m a red, rotated\ntext over multiple lines.".to_owned()
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((300000,7500000)),
                        objects: vec![
                            EventObject::TagBold(
                                false
                            ),
                            EventObject::TagColor(Color::Mono([
                                255, 0, 0
                            ])),
                            EventObject::TagTexture(
                                "cute".to_owned()
                            ),
                            EventObject::GeometryShape(vec![
                                ShapeSegment::MoveTo(Point2D {
                                    x: 0.0,
                                    y: 0.0
                                }),
                                ShapeSegment::LineTo(Point2D {
                                    x: 50.5,
                                    y: 0.0
                                }),
                                ShapeSegment::LineTo(Point2D {
                                    x: 50.5,
                                    y: 20.125
                                }),
                                ShapeSegment::LineTo(Point2D {
                                    x: 0.0,
                                    y: 20.125
                                }),
                                ShapeSegment::CurveTo(
                                    Point2D {
                                        x: 42.0,
                                        y: 1337.0
                                    },
                                    Point2D {
                                        x: 26.0,
                                        y: 0.0
                                    },
                                    Point2D {
                                        x: 3.141,
                                        y: 2.718
                                    }
                                ),
                                ShapeSegment::ArcBy(
                                    Point2D {
                                        x: 0.0,
                                        y: 0.0
                                    },
                                    180.0
                                ),
                                ShapeSegment::Close
                            ])
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((600000,39000000)),
                        objects: vec![
                            EventObject::TagAnimate(Box::new(Animate {
                                time: Some((500, 1000)),
                                formula: None,
                                tags: vec![
                                    EventObject::TagScale(Scale::All(
                                        2.0, 2.0, 1.0
                                    ))
                                ]
                            })),
                            EventObject::GeometryText(
                                "This text is\ngetting huge".to_owned()
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((1200000,1260000)),
                        objects: vec![
                            EventObject::TagFont(
                                "Rabi-Ribi".to_owned()
                            ),
                            EventObject::GeometryPoints(vec![
                                Point2D {
                                    x: 0.0,
                                    y: 0.0
                                },
                                Point2D {
                                    x: 100.0,
                                    y: 0.0
                                },
                                Point2D {
                                    x: 66.6,
                                    y: 50.0
                                }
                            ]),
                            EventObject::TagBold(
                                false
                            ),
                            EventObject::TagColor(Color::Mono([
                                255, 0, 0
                            ])),
                            EventObject::GeometryPoints(vec![
                                Point2D {
                                    x: 33.3,
                                    y: 50.0
                                }
                            ])
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Id("show-something".to_owned()),
                        objects: vec![
                            EventObject::TagBold(
                                true
                            ),
                            EventObject::GeometryText(
                                "This will only be shown when the event id is given".to_owned()
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((0,3600000)),
                        objects: vec![
                            EventObject::TagFont(
                                "Arial".to_owned()
                            ),
                            EventObject::TagSize(
                                20.5
                            ),
                            EventObject::TagBold(
                                true
                            ),
                            EventObject::TagItalic(
                                false
                            ),
                            EventObject::TagUnderline(
                                true
                            ),
                            EventObject::TagStrikeout(
                                false
                            ),
                            EventObject::TagPosition(Point3D {
                                x: -20.0,
                                y: 1.5,
                                z: 0.0
                            }),
                            EventObject::TagPosition(Point3D {
                                x: 100.0,
                                y: 100.0,
                                z: -50.0
                            }),
                            EventObject::TagAlignment(Alignment::Numpad(
                                Numpad::MiddleCenter
                            )),
                            EventObject::TagAlignment(Alignment::Offset(Point2D {
                                x: 1.0,
                                y: 2.7
                            })),
                            EventObject::TagMargin(Margin::All(
                                1.0, 2.0, 3.0, 4.0
                            )),
                            EventObject::TagMargin(Margin::All(
                                5.0, 5.0, 5.0, 5.0
                            )),
                            EventObject::TagMargin(Margin::Top(
                                -1.23
                            )),
                            EventObject::TagMargin(Margin::Right(
                                4.56
                            )),
                            EventObject::TagMargin(Margin::Bottom(
                                -7.89
                            )),
                            EventObject::TagMargin(Margin::Left(
                                0.0
                            )),
                            EventObject::TagWrapStyle(
                                WrapStyle::NoWrap
                            ),
                            EventObject::TagDirection(
                                Direction::RightToLeft
                            ),
                            EventObject::TagSpace(Space::All(
                                9.8, 7.6
                            )),
                            EventObject::TagSpace(Space::All(
                                5.5, 5.5
                            )),
                            EventObject::TagSpace(Space::Horizontal(
                                4.0
                            )),
                            EventObject::TagSpace(Space::Vertical(
                                3.0
                            )),
                            EventObject::TagRotate(Rotate::All(
                                5.0, 9.0, 1.0
                            )),
                            EventObject::TagRotate(Rotate::X(
                                45.0
                            )),
                            EventObject::TagRotate(Rotate::Y(
                                90.0
                            )),
                            EventObject::TagRotate(Rotate::Z(
                                -135.0
                            )),
                            EventObject::TagScale(Scale::All(
                                0.75, 1.25, 1.0
                            )),
                            EventObject::TagScale(Scale::X(
                                0.5
                            )),
                            EventObject::TagScale(Scale::Y(
                                1.5
                            )),
                            EventObject::TagScale(Scale::Z(
                                2.0
                            )),
                            EventObject::TagTranslate(Translate::All(
                                100.0, 200.0, 0.0
                            )),
                            EventObject::TagTranslate(Translate::X(
                                -20.4
                            )),
                            EventObject::TagTranslate(Translate::Y(
                                210.0
                            )),
                            EventObject::TagTranslate(Translate::Z(
                                50.0
                            )),
                            EventObject::TagShear(Shear::All(
                                1.0, -1.0
                            )),
                            EventObject::TagShear(Shear::X(
                                1.2
                            )),
                            EventObject::TagShear(Shear::Y(
                                0.33
                            )),
                            EventObject::TagMatrix(Box::new([
                                0.5, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0, 1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0
                            ])),
                            EventObject::TagIdentity,
                            EventObject::TagBorder(Border::All(
                                42.0, 42.0
                            )),
                            EventObject::TagBorder(Border::All(
                                20.0, 22.0
                            )),
                            EventObject::TagBorder(Border::Horizontal(
                                7.5
                            )),
                            EventObject::TagBorder(Border::Vertical(
                                -17.83
                            )),
                            EventObject::TagJoin(
                                Join::Round
                            ),
                            EventObject::TagCap(
                                Cap::Square
                            ),
                            EventObject::TagTexture(
                                "cute".to_owned()
                            ),
                            EventObject::TagTexFill {
                                x0: 0.0,
                                y0: 0.0,
                                x1: 1.0,
                                y1: 0.5,
                                wrap: TextureWrapping::Repeat
                            },
                            EventObject::TagColor(Color::CornersWithStop([
                                [0, 0, 0],
                                [255, 255,  255],
                                [255, 0, 0],
                                [0, 255, 0],
                                [0, 0, 255]
                            ])),
                            EventObject::TagBorderColor(Color::LinearWithStop([
                                [255, 255, 0],
                                [0, 255, 255],
                                [255, 0, 255]
                            ])),
                            EventObject::TagAlpha(Alpha::Mono(
                                128
                            )),
                            EventObject::TagBorderAlpha(Alpha::Corners([
                                10,
                                11,
                                12,
                                13
                            ])),
                            EventObject::TagBlur(Blur::All(
                                1.2, 1.5
                            )),
                            EventObject::TagBlur(Blur::All(
                                6.66, 6.66
                            )),
                            EventObject::TagBlur(Blur::Horizontal(
                                11.0
                            )),
                            EventObject::TagBlur(Blur::Vertical(
                                5.0
                            )),
                            EventObject::TagBlend(
                                Blend::Screen
                            ),
                            EventObject::TagTarget(
                                Target::Frame
                            ),
                            EventObject::TagMaskMode(
                                MaskMode::Normal
                            ),
                            EventObject::TagMaskClear,
                            EventObject::TagAnimate(Box::new(Animate {
                                time: None,
                                formula: None,
                                tags: vec![]
                            })),
                            EventObject::TagAnimate(Box::new(Animate {
                                time: Some((100, -2000)),
                                formula: Some("t^2".to_owned()),
                                tags: vec![
                                    EventObject::TagSize(
                                        42.0
                                    ),
                                    EventObject::TagColor(Color::Mono([
                                        0, 128, 255
                                    ])),
                                    EventObject::TagTranslate(Translate::X(
                                        99.9
                                    ))
                                ]
                            })),
                            EventObject::TagKaraoke(
                                260
                            ),
                            EventObject::TagKaraokeSet(
                                0
                            ),
                            EventObject::TagKaraokeColor(
                                [248, 0, 143]
                            ),
                            EventObject::GeometryText(
                                "Super styled :)".to_owned()
                            )
                        ]
                    }
                ],
                fonts: {
                    let mut fonts = HashMap::new();
                    fonts.insert(
                        FontFace {
                            family: "Rabi-Ribi".to_owned(),
                            style: FontStyle::Bold
                        },
                        vec![82,97,98,105,45,82,105,98,105]
                    );
                    fonts
                },
                textures: {
                    let mut textures = HashMap::new();
                    textures.insert(
                        "Jitter".to_owned(),
                        vec![74,105,116,116,101,114]
                    );
                    textures.insert(
                        "cute".to_owned(),
                        vec![137,80,78,71,13,10,26,10,0,0,0,13,73,72,68,82,0,0,0,32,0,0,0,32,8,0,0,0,0,86,17,37,40,0,0,0,9,112,72,89,115,0,0,46,35,0,0,46,35,1,120,165,63,118,0,0,0,7,116,73,77,69,7,227,4,29,2,32,49,204,41,26,248,0,0,1,179,73,68,65,84,56,203,109,146,177,75,35,81,16,135,127,183,173,164,58,176,72,103,101,113,157,205,221,177,112,141,87,137,254,1,183,149,54,146,226,42,173,172,181,213,38,77,46,214,86,41,18,177,58,60,9,108,35,110,17,228,194,181,65,81,34,132,40,172,176,194,237,30,249,174,120,111,95,54,217,76,53,243,155,111,222,204,27,70,0,131,143,59,17,115,22,237,124,30,0,8,160,33,169,145,22,211,105,67,82,3,192,147,164,68,82,237,248,159,156,101,199,53,43,75,0,103,146,164,230,244,129,166,36,233,204,181,184,49,117,87,121,254,202,196,55,14,120,241,36,73,213,7,147,127,88,150,36,121,47,14,224,192,148,124,207,0,178,154,137,14,152,2,145,29,238,23,192,79,27,68,5,32,219,50,154,159,64,242,201,248,91,89,1,160,107,171,206,225,220,186,93,138,64,22,24,117,53,73,86,141,23,100,51,0,125,91,23,134,214,233,51,11,80,55,122,205,126,161,206,60,240,186,169,130,109,190,150,0,122,69,160,71,9,136,252,34,224,71,243,64,91,115,214,158,5,186,42,217,204,30,70,203,101,160,58,194,29,140,46,70,101,96,120,33,119,48,217,138,22,216,74,230,90,60,106,161,61,154,22,127,81,106,149,253,65,75,106,13,246,109,152,74,169,132,255,173,125,237,6,143,165,216,125,233,186,19,248,136,219,163,74,254,230,238,239,83,233,180,191,155,199,149,163,91,4,196,225,246,162,17,182,195,216,237,225,222,203,15,102,60,206,15,198,187,47,44,42,222,179,234,198,120,188,97,221,189,56,7,222,162,195,37,173,91,249,107,158,95,215,210,97,244,6,226,71,85,65,231,105,114,89,41,246,175,92,78,158,58,129,170,77,52,57,105,221,1,48,172,231,131,200,171,15,1,184,107,157,76,222,49,45,123,254,211,235,133,250,178,182,246,225,253,84,252,15,108,126,214,79,66,138,234,197,0,0,0,0,73,69,78,68,174,66,96,130]
                    );
                    textures
                }
            }
        );
    }

    #[test]
    fn test_ssb_errors() {
        // Section
        assert_eq!(
            Ssb::default().parse(Cursor::new("x")).map_err(|err| err.to_string()),
            Err("No section set! <0:0>".to_owned())
        );
        // Info
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Info\nINVALID_ENTRY")).map_err(|err| err.to_string()),
            Err("Invalid info entry! <1:0>".to_owned())
        );
        // Target
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Target\nWidth: 4096\nINVALID_ENTRY")).map_err(|err| err.to_string()),
            Err("Invalid target entry! <2:0>".to_owned())
        );
        // Macros
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Macros\nabc: []\n123: Hi!\nINVALID_ENTRY")).map_err(|err| err.to_string()),
            Err("Invalid macros entry! <3:0>".to_owned())
        );
        // Events
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Events\nINVALID_ENTRY")).map_err(|err| err.to_string()),
            Err("Invalid events entry! <1:0>".to_owned())
        );
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Events\n1:-0|||")).map_err(|err| err.to_string()),
            Err("Start time greater than end time! <1:0>".to_owned())
        );
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Events\n?|||")).map_err(|err| err.to_string()),
            Err("Invalid trigger format! <1:0>".to_owned())
        );
        // Resources
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Resources\nINVALID_ENTRY")).map_err(|err| err.to_string()),
            Err("Invalid resources entry! <1:0>".to_owned())
        );
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Resources\nFont: myfont,Regula")).map_err(|err| err.to_string()),
            Err("Font family, style and data expected! <1:6>".to_owned())
        );
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Resources\nTexture: ")).map_err(|err| err.to_string()),
            Err("Texture id, data type and data expected! <1:9>".to_owned())
        );
        assert_eq!(
            Ssb::default().parse(Cursor::new("#Resources\nTexture: Pikachu,data,INVALID_BASE64")).map_err(|err| err.to_string()),
            Err("Texture data not in base64 format! <1:22>".to_owned())
        );
    }
}