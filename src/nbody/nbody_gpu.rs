use crate::nbody::nbody::NBody;
use rand::Rng;
use wgpu::util::DeviceExt;
use wgpu::wgt::PollType;

pub struct NBodyGPU {
    vector_length: u32,
    point_locations: wgpu::Buffer,
    point_speeds: wgpu::Buffer,
    point_mass: wgpu::Buffer,
    instance: wgpu::Instance,
    device: wgpu::Device,
    queue: wgpu::Queue,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    shader_module: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    compute_pipeline: wgpu::ComputePipeline,
}

impl NBodyGPU {
    pub async fn new(start_locations: &Vec<[f32; 2]>) -> Self {
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
        let point_locations = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Locations"),
            contents: bytemuck::cast_slice(start_locations.clone().as_slice()),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        });
        let point_speeds = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Locations"),
            contents: bytemuck::cast_slice(&vec![[0.0f32; 2]; start_locations.len()]),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        });
        let point_mass = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Point Locations"),
            contents: bytemuck::cast_slice(&vec![100.0f32; start_locations.len()]),
            usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
        });
        let vector_length = start_locations.len() as u32;
        let vector_length_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vector Length Uniform Buffer"),
            contents: bytemuck::bytes_of(&vector_length),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
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
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
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
                    resource: point_locations.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: point_speeds.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: point_mass.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: vector_length_buffer.as_entire_binding(),
                },
            ],
            label: Some("NBody Bind Group"),
        });
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("NBody Compute shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("nbody.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("NBody Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader_module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        NBodyGPU {
            vector_length,
            point_locations,
            point_speeds,
            point_mass,
            instance,
            device,
            queue,
            bind_group_layout,
            bind_group,
            shader_module,
            pipeline_layout,
            compute_pipeline,
        }
    }
}

impl NBody for NBodyGPU {
    async fn step(&mut self, steps: usize) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute Encoder"),
            });

        for _ in 0..steps {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("is_root pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &self.bind_group, &[]);
            compute_pass.dispatch_workgroups(self.vector_length, 1, 1);
        }
        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
    }
    async fn get_point_locations(&mut self) -> Vec<[f32; 2]> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Compute Encoder"),
            });
        let point_location_readback = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Result Readback Buffer"),
            size: (std::mem::size_of::<[f32; 2]>() * self.vector_length as usize) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        encoder.copy_buffer_to_buffer(
            &self.point_locations,
            0,
            &point_location_readback,
            0,
            (std::mem::size_of::<[f32; 2]>() * self.vector_length as usize) as u64,
        );
        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
        let (sender, receiver) = futures_channel::oneshot::channel();
        point_location_readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, |result| {
                let _ = sender.send(result);
            });
        self.device.poll(PollType::Wait).unwrap(); // TODO: poll in the background instead of blocking
        receiver
            .await
            .expect("communication failed")
            .expect("buffer reading failed");

        let slice: &[u8] = &point_location_readback.slice(..).get_mapped_range();
        let float_slice: &[[f32; 2]] = bytemuck::cast_slice(slice);
        float_slice.to_vec()
    }
}
#[cfg(test)]
mod test {
    use crate::nbody::nbody::NBody;
    use crate::nbody::nbody_gpu::NBodyGPU;
    use std::time::Instant;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn nbody_test_queue_depth() {
        let start_locations = &vec![[0.0, 1.0], [0.0, -1.0]];
        let mut nbody = NBodyGPU::new(&start_locations.to_vec()).await;
        let mut step_size = 1;
        let max_step_size = 10000;
        while step_size <= max_step_size {
            let start = Instant::now();
            for _ in (0..10000).step_by(step_size) {
                nbody.step(step_size).await;
            }
            let duration = Instant::now() - start;
            println!("For step_size {step_size} it took {duration:?}");
            step_size *= 10;
        }
    }
}
