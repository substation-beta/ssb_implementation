mod grammar_tests {
    // Imports
    use ssb_parser::{
        types::ssb::*,
        types::geometries::*,
        types::tags::*,
        data::{Ssb, SsbRender}
    };
    use std::{
        collections::HashMap,
        convert::TryFrom,
        io::{BufReader, Cursor},
        fs::File,
        path::Path
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
        ), None).unwrap();
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
        assert_eq!(ssb.textures.get("Fancy"), Some(&vec![70, 97, 110, 99, 121]));
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
            ),
            Some(Path::new(env!("CARGO_MANIFEST_DIR")))
        ).unwrap_or_else(|exception| panic!("SSB parsing error: {}", exception) );
        //println!("{:#?}", ssb);
        // Parse 2nd phase
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
                            EventObject::Tag(
                                EventTag::Position(Point3D {
                                    x: 100.0,
                                    y: 200.0,
                                    z: -1.0
                                })
                            ),
                            EventObject::Tag(
                                EventTag::Rotate(Rotate::Z(
                                    180.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Bold(
                                    false
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Color(Color::Mono([
                                    255, 0, 0
                                ]))
                            ),
                            EventObject::Geometry(
                                EventGeometry::Text(
                                    "I\'m a red, rotated\ntext over multiple lines.".to_owned()
                                )
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((300000,7500000)),
                        objects: vec![
                            EventObject::Tag(
                                EventTag::Bold(
                                    false
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Color(Color::Mono([
                                    255, 0, 0
                                ]))
                            ),
                            EventObject::Tag(
                                EventTag::Texture(
                                    "cute".to_owned()
                                )
                            ),
                            EventObject::Geometry(
                                EventGeometry::Shape(vec![
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
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((600000,39000000)),
                        objects: vec![
                            EventObject::Geometry(
                                EventGeometry::Text(
                                    "This text is\ngetting huge".to_owned()
                                )
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((1200000,1260000)),
                        objects: vec![
                            EventObject::Tag(
                                EventTag::Font(
                                    "Rabi-Ribi".to_owned()
                                )
                            ),
                            EventObject::Geometry(
                                EventGeometry::Points(vec![
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
                                ])
                            ),
                            EventObject::Tag(
                                EventTag::Bold(
                                    false
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Color(Color::Mono([
                                    255, 0, 0
                                ]))
                            ),
                            EventObject::Geometry(
                                EventGeometry::Points(vec![
                                    Point2D {
                                        x: 33.3,
                                        y: 50.0
                                    }
                                ])
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Id("show-something".to_owned()),
                        objects: vec![
                            EventObject::Tag(
                                EventTag::Bold(
                                    true
                                )
                            ),
                            EventObject::Geometry(
                                EventGeometry::Text(
                                    "This will only be shown when the event id is given".to_owned()
                                )
                            )
                        ]
                    },
                    EventRender {
                        trigger: EventTrigger::Time((0,3600000)),
                        objects: vec![
                            EventObject::Tag(
                                EventTag::Font(
                                    "Arial".to_owned()
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Size(
                                    20.5
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Bold(
                                    true
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Italic(
                                    false
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Underline(
                                    true
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Strikeout(
                                    false
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Position(Point3D {
                                    x: -20.0,
                                    y: 1.5,
                                    z: 0.0
                                })
                            ),
                            EventObject::Tag(
                                EventTag::Position(Point3D {
                                    x: 100.0,
                                    y: 100.0,
                                    z: -50.0
                                })
                            ),
                            EventObject::Tag(
                                EventTag::Alignment(Alignment::Numpad(
                                    Numpad::MiddleCenter
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Alignment(Alignment::Offset(Point2D {
                                    x: 1.0,
                                    y: 2.7
                                }))
                            ),
                            EventObject::Tag(
                                EventTag::Margin(Margin::All(
                                    1.0, 2.0, 3.0, 4.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Margin(Margin::Top(
                                    -1.23
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::WrapStyle(
                                    WrapStyle::NoWrap
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Direction(
                                    Direction::RightToLeft
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Space(Space::All(
                                    9.8, 7.6
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Space(Space::Horizontal(
                                    4.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Rotate(Rotate::All(
                                    5.0, 9.0, 1.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Rotate(Rotate::Y(
                                    90.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Scale(Scale::All(
                                    0.75, 1.25, 1.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Scale(Scale::Z(
                                    2.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Translate(Translate::All(
                                    100.0, 200.0, 0.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Translate(Translate::Z(
                                    50.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Shear(Shear::All(
                                    1.0, -1.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Shear(Shear::Y(
                                    0.33
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Matrix([
                                    0.5, 0.0, 0.0, 0.0,
                                    0.0, 1.0, 0.0, 0.0,
                                    0.0, 0.0, 1.0, 0.0,
                                    0.0, 0.0, 0.0, 1.0
                                ])
                            ),
                            EventObject::Tag(
                                EventTag::Border(Border::All(
                                    42.0, 42.0
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Border(Border::Vertical(
                                    -17.83
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::Join(
                                    Join::Round
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Cap(
                                    Cap::Square
                                )
                            ),
                            EventObject::Tag(
                                EventTag::Texture(
                                    "cute".to_owned()
                                )
                            ),
                            EventObject::Tag(
                                EventTag::TexFill(TexFill {
                                    x0: 0.0,
                                    y0: 0.0,
                                    x1: 1.0,
                                    y1: 0.5,
                                    wrap: TextureWrapping::Repeat
                                })
                            ),
                            EventObject::Tag(
                                EventTag::Color(Color::CornersWithStop([
                                    [0, 0, 0],
                                    [255, 255,  255],
                                    [255, 0, 0],
                                    [0, 255, 0],
                                    [0, 0, 255]
                                ]))
                            ),
                            EventObject::Tag(
                                EventTag::BorderColor(Color::LinearWithStop([
                                    [255, 255, 0],
                                    [0, 255, 255],
                                    [255, 0, 255]
                                ]))
                            ),
                            EventObject::Tag(
                                EventTag::Alpha(Alpha::Mono(
                                    128
                                ))
                            ),
                            EventObject::Tag(
                                EventTag::BorderAlpha(Alpha::Corners([
                                    10,
                                    11,
                                    12,
                                    13
                                ]))
                            ),
                            EventObject::Geometry(
                                EventGeometry::Text(
                                    "Super styled :)".to_owned()
                                )
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
}