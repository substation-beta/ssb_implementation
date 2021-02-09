mod code_structure_tests {
    // Imports
    use std::mem::size_of;
    use ssb_parser::objects::event_objects::EventObject;

    #[test]
    fn test_sizes() {
        let event_object_size = size_of::<EventObject>();
        assert!(event_object_size <= 40, "EventObject is larger than 40 bytes: {}!", event_object_size);
    }
}