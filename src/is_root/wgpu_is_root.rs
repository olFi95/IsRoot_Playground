use crate::is_root::is_root::Root;
use wgpu::util::DeviceExt;
use wgpu::wgt::PollType;

pub struct WgpuIsRoot;

impl Root for WgpuIsRoot {
    async fn is_root(squareroot: &Vec<f32>, input: &Vec<f32>, delta: f32) -> Option<bool> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::from_env_or_default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let squareroot_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Squareroot Buffer"),
            contents: bytemuck::cast_slice(squareroot),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        });
        let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Input Buffer"),
            contents: bytemuck::cast_slice(input),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        });
        let delta_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Delta Uniform Buffer"),
            contents: bytemuck::bytes_of(&delta),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let vector_length = input.len() as u32;
        let vector_length_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vector Length Uniform Buffer"),
            contents: bytemuck::bytes_of(&vector_length),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let initial_result: u32 = 1;
        let result_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Result Buffer"),
            contents: bytemuck::bytes_of(&initial_result),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
        let result_readback = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result Readback Buffer"),
            size: std::mem::size_of::<u32>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: squareroot_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: input_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: delta_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: result_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: vector_length_buffer.as_entire_binding(),
                },
            ],
            label: Some("Is Root Bind Group"),
        });
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("is_root shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("wgpu_is_root_shader.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("is_root pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(vector_length, 1, 1);
        }

        encoder.copy_buffer_to_buffer(
            &result_buffer,
            0,
            &result_readback,
            0,
            std::mem::size_of::<u32>() as u64,
        );
        let command_buffer = encoder.finish();
        queue.submit(Some(command_buffer));

        let (sender, receiver) = futures_channel::oneshot::channel();
        result_readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, |result| {
                let _ = sender.send(result);
            });
        device.poll(PollType::Wait).unwrap(); // TODO: poll in the background instead of blocking
        receiver
            .await
            .expect("communication failed")
            .expect("buffer reading failed");
        let slice: &[u8] = &result_readback.slice(..).get_mapped_range();
        let value = bytemuck::from_bytes::<u32>(&slice);

        Some(*value == 1)
    }
}

#[cfg(test)]
mod test {
    use crate::is_root::is_root::Root;
    use crate::is_root::wgpu_is_root::WgpuIsRoot;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_root_wgpu_happy_path_1() {
        let squareroot = &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let input = &vec![1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0, 64.0];
        let result = WgpuIsRoot::is_root(squareroot, input, 0.001).await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), true);
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_root_wgpu_happy_path_2() {
        let squareroot = &vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let input = &vec![1.0, 4.0, 9.0, 16.0, 25.0, 36.0, 49.0];
        let result = WgpuIsRoot::is_root(squareroot, input, 0.001).await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), true);
    }
    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_is_root_one_error() {
        let squareroot = &vec![1.0, 4.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let input = &vec![1.0, 4.0, 10.0, 16.0, 25.0, 36.0, 49.0];
        let result = WgpuIsRoot::is_root(squareroot, input, 0.001).await;
        assert!(result.is_some());
        assert_eq!(result.unwrap(), false);
    }
}
