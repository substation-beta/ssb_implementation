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
                .par_iter() // Parallel processing starts here
                .map(|num| num << 1)
                .sum::<u32>(),
            4_000u32
        );
    }

    #[test]
    fn test_rayon_threadpool() {
        let (mut x, mut y) = (2u8, 5u8);
        rayon::scope(|scope| {
            scope.spawn(|_| x *= x);
            scope.spawn(|_| y <<= 2);
        });
        assert_eq!(x, 4u8);
        assert_eq!(y, 20u8);
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    mod x86_simd {
        // Imports compatible to architecture
        #[cfg(target_arch = "x86")]
        use std::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use std::arch::x86_64::*;

        #[test]
        fn test_features_support() {
            // Information buffer
            let mut info = vec![];
            // Check common features
            info.push(format!("\tSSE2: {}", is_x86_feature_detected!("sse2")));
            info.push(format!("\tAVX: {}", is_x86_feature_detected!("avx")));
            // Request feature bits
            for (leaf, profile) in &[(1, "basic"), (7, "extensions")] {
                unsafe {
                    let cpuid_result = __cpuid_count(*leaf, 0);
                    info.push(format!("\tCPUID ({}):", profile));
                    info.push(format!("\t\teax: {:032b}", cpuid_result.eax));
                    info.push(format!("\t\tebx: {:032b}", cpuid_result.ebx));
                    info.push(format!("\t\tecx: {:032b}", cpuid_result.ecx));
                    info.push(format!("\t\tedx: {:032b}", cpuid_result.edx));
                }
            }
            // Print collected information
            println!("SIMD support:\n{}", info.join("\n"));
        }

        #[cfg(target_feature = "sse2")]
        #[test]
        fn test_sse2_mul() {
            let mut result = vec![0f32;4];
            unsafe {
                _mm_storeu_ps(
                    result.as_mut_ptr(),
                    _mm_mul_ps(
                        _mm_set_ps(1., 2., 3., 4.),
                        _mm_set_ps(5., 6., 7., 8.)
                    )
                );
            }
            assert_eq!(result, vec![32f32, 21f32, 12f32, 5f32]);
        }
    }

    #[cfg(feature = "gpgpu")]
    mod gpgpu {
        // Imports
        use ocl::{
            Platform,
            Device,
            enums::{
                PlatformInfo,
                DeviceInfo
            },
            ProQue
        };

        #[test]
        fn test_ocl_support() {
            // Information buffer
            let mut info = vec![];
            // Iterate through available OpenCL platforms
            info.push("\tPlatforms:".to_owned());
            for (platform_index, platform) in Platform::list().into_iter().enumerate() {
                info.push(format!("\t\t{}:", platform_index));
                // Save platform information
                for platform_info_key in &[
                    PlatformInfo::Name,
                    PlatformInfo::Vendor,
                    PlatformInfo::Version,
                    PlatformInfo::Profile,
                    PlatformInfo::Extensions
                ] {
                    if let Ok(platform_info_value) = platform.info(*platform_info_key) {
                        info.push(format!("\t\t\t{:?}: {}", platform_info_key, platform_info_value));
                    }
                }
                // Iterate through available OpenCL devices by platform
                if let Ok(platform_devices) = Device::list_all(platform) {
                    info.push("\t\t\tDevices:".to_owned());
                    for (device_index, device) in platform_devices.into_iter().enumerate() {
                        info.push(format!("\t\t\t\t{}:", device_index));
                        // Save device information
                        for device_info_key in &[
                            DeviceInfo::Type,
                            DeviceInfo::Name,
                            DeviceInfo::Vendor,
                            DeviceInfo::DriverVersion,
                            DeviceInfo::OpenclCVersion,
                            DeviceInfo::Profile,
                            DeviceInfo::MaxComputeUnits,
                            DeviceInfo::MaxWorkGroupSize,
                            DeviceInfo::AddressBits,
                            DeviceInfo::ImageSupport,
                            DeviceInfo::MaxSamplers,
                            DeviceInfo::MemBaseAddrAlign,
                            DeviceInfo::EndianLittle,
                            DeviceInfo::Available,
                            DeviceInfo::BuiltInKernels,
                            DeviceInfo::ParentDevice
                        ] {
                            if let Ok(device_info_value) = device.info(*device_info_key) {
                                info.push(format!("\t\t\t\t\t{:?}: {}", device_info_key, device_info_value));
                            }
                        }
                    }
                }
            }
            // List default platform & devices
            info.push(format!("\tDefault platform: {:?}", Platform::list().into_iter().position(|platform| *platform == *Platform::default() ).expect("Default platform missing?!")));
            info.push(format!("\tDevice specifier: {:?}", Device::specifier()));
            // Print collected information
            println!("OpenCL support:\n{}", info.join("\n"));
        }

        #[test]
        fn test_ocl_kernel() {
            // Create GPU processing queue
            let pro_que = ProQue::builder()
                .src(r##"
                    __kernel void fill(__global unsigned char* buffer, unsigned char value) {
                        buffer[get_global_id(0)] = value;
                    }
                "##)
                .dims(1920 * 1080 * 3)  // Number of iterations per run / size of buffers
                .build().expect("Couldn't build processing queue!");

            // Create GPU buffer
            let buffer = pro_que.create_buffer::<u8>().expect("Couldn't create buffer!");

            // Create kernel instance on processing queue
            let kernel = pro_que.kernel_builder("fill")
                .arg(&buffer)
                .arg(255u8)
                .build().expect("Couldn't instantiate kernel!");

            // Execute kernel
            unsafe {
                kernel.enq().expect("Couldn't enqueue kernel for execution!");
            }

            // Read buffer from GPU to CPU
            let mut data = vec![0u8; buffer.len()];
            buffer.read(&mut data).enq().expect("Couldn't enqueue buffer reading operation!");

            // Check buffer after kernel execution
            assert_eq!(data.len(), pro_que.dims().to_len());
            assert_eq!(data.last(), Some(&255u8));
        }
    }
}