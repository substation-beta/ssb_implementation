mod accel_tests {
    // Imports
    use lru::LruCache;
    use rayon::prelude::*;

    #[test]
    fn test_lru_cache() {
        // Create initial cache
        let mut cache = LruCache::new(2);
        cache.put("apple", 3);
        cache.put("banana", 2);

        // Request cache
        assert_eq!(*cache.get(&"apple").unwrap(), 3);
        assert_eq!(*cache.get(&"banana").unwrap(), 2);
        assert!(cache.get(&"pear").is_none());

        // Modify cache & request again
        cache.put("pear", 4);
        assert_eq!(*cache.get(&"pear").unwrap(), 4);
        assert_eq!(*cache.get(&"banana").unwrap(), 2);
        assert!(cache.get(&"apple").is_none());

        // Reset cached value
        {
            let v = cache.get_mut(&"banana").unwrap();
            *v = 6;
        }
        assert_eq!(*cache.get(&"banana").unwrap(), 6);
    }

    #[test]
    fn test_rayon_iter() {
        assert_eq!(
            vec![2u32;1_000]
                .par_iter()
                .map(|num| num << 1)
                .sum::<u32>(),
            4_000u32
        );
    }

    #[cfg(all(
        target_feature = "sse2",
        any(
            target_arch = "x86",
            target_arch = "x86_64"
        )
    ))]
    mod sse2 {
        // Import compatible to architecture
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        #[test]
        fn test_add() {
            let mut result = vec![0f32;4];
            unsafe {
                _mm_storeu_ps(
                    result.as_mut_ptr(),
                    _mm_add_ps(
                        _mm_set_ps(1.0, 2.0, 3.0, 4.0),
                        _mm_set_ps(5.0, 6.0, 7.0, 8.0)
                    )
                );
            }
            assert_eq!(result, vec![12f32, 10f32, 8f32, 6f32]);
        }
    }

    #[cfg(feature = "gpgpu")]
    mod gpgpu {
        // Imports
        use ocl::ProQue;

        #[test]
        fn test_ocl() {
            // Create GPU processing queue
            let pro_que = ProQue::builder()
                .src(r##"
                    __kernel void fill(__global unsigned char* buffer, unsigned char value) {
                        buffer[get_global_id(0)] = value;
                    }
                "##)
                .dims(1920 * 1080 * 3)
                .build().expect("Couldn't build processing queue!");

            // Create GPU buffer
            let buffer = pro_que.create_buffer::<u8>().expect("Couldn't create buffer!");

            // Create kernel for execution on GPU
            let kernel = pro_que.kernel_builder("fill")
                .arg(&buffer)
                .arg(255u8)
                .build().expect("Couldn't build kernel!");

            // Execute kernel
            unsafe {kernel.enq().expect("Couldn't enqueue kernel for execution!");}

            // Read buffer from GPU to CPU
            let mut data = vec![0u8; buffer.len()];
            buffer.read(&mut data).enq().expect("Couldn't enqueue operation for buffer reading!");

            // Check buffer after kernel execution
            assert_eq!(data.len(), pro_que.dims().to_len());
            assert_eq!(data.last(), Some(&255u8));
        }
    }
}