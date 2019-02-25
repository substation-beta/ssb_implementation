#[cfg(test)]
mod tessellation_tests {
    // Imports
    use lyon::math::point;
    use lyon::path::Path;
    use lyon::tessellation::{VertexBuffers, FillVertex, FillTessellator, FillOptions, geometry_builder};

    #[test]
    fn test_simple_shape2triangles() {
        // Build a path
        let mut path_builder = Path::builder();
        path_builder.move_to(point(0.0, 0.0));
        path_builder.line_to(point(1.0, 0.0));
        path_builder.quadratic_bezier_to(point(2.0, 0.0), point(2.0, 1.0));
        path_builder.cubic_bezier_to(point(1.0, 1.0), point(0.0, 1.0), point(0.0, 0.0));
        path_builder.close();
        let path = path_builder.build();

        // Tessellation output buffers
        let mut buffers = VertexBuffers::<FillVertex, u16>::new();
        assert!(
            // Tessellate path into buffers
            FillTessellator::new().tessellate_path(
                &path,
                &FillOptions::default(),
                &mut geometry_builder::simple_builder(&mut buffers)
            ).is_ok()
        );

        // Print tessellation output
        println!(
            "Vertices:\n{:?}\n\nIndices:\n{:?}",
            buffers.vertices,
            buffers.indices
        );
        assert!(
            !buffers.vertices.is_empty() &&
            !buffers.indices.is_empty()
        );
    }
}