use std::{borrow::Cow, sync::Arc};

use glam::{vec3, Mat4};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    *,
};
use winit::{event::*, event_loop::*, keyboard::KeyCode, window::Window};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn create_depth_texture(
    device: &Device,
    depth_format: TextureFormat,
    width: u32,
    height: u32,
) -> TextureView {
    let depth_texture = device.create_texture(&TextureDescriptor {
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: depth_format,
        usage: TextureUsages::RENDER_ATTACHMENT,
        label: None,
        view_formats: &[],
    });

    let depth_texture = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    depth_texture
}
pub async fn run(window: Window, event_loop: EventLoop<()>) -> Result<()> {
    let instance = Instance::new(InstanceDescriptor {
        backends: Backends::all(),
        ..Default::default()
    });
    let surface = instance.create_surface(&window)?;
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
            },
            None,
        )
        .await?;

    #[rustfmt::skip]
    let vertex_data = [
        -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0, // 0
         0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0, // 1
         0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0, // 2
        -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0, // 3

        -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,
         0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 0.0,
         0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 1.0,

        -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
        -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
        -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
        -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,

         0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
         0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
         0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
         0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,

        -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
         0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
         0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0f32,
    ];
    let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Vertex buffer"),
        usage: BufferUsages::VERTEX,
        contents: bytemuck::cast_slice(&vertex_data),
    });
    let mut indices: Vec<u16> = Vec::new();
    for i in 0..6 {
        let start = i * 4;
        indices.push(start);
        indices.push(start + 1);
        indices.push(start + 2);
        indices.push(start);
        indices.push(start + 2);
        indices.push(start + 3);
    }
    let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Index buffer"),
        usage: BufferUsages::INDEX,
        contents: bytemuck::cast_slice(&indices),
    });
    let vertex_buffers = [VertexBufferLayout {
        array_stride: (2 + 3 + 3) * 4,
        step_mode: VertexStepMode::Vertex,
        attributes: &[
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 0,
                shader_location: 0,
            },
            VertexAttribute {
                format: VertexFormat::Float32x3,
                offset: 3 * 4,
                shader_location: 1,
            },
            VertexAttribute {
                format: VertexFormat::Float32x2,
                offset: (3 + 3) * 4,
                shader_location: 2,
            },
        ],
    }];

    let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(4 * 4 * 4),
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(4 * 4 * 4),
                },
                count: None,
            },
        ],
    });

    let depth_format = TextureFormat::Depth24Plus;

    let mut size = window.inner_size();
    size.width = size.width.max(1);
    size.height = size.height.max(1);
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];
    let mut config = surface
        .get_default_config(&adapter, size.width, size.height)
        .unwrap();
    surface.configure(&device, &config);

    let shader = device.create_shader_module(ShaderModuleDescriptor {
        label: None,
        source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
    });

    let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &vertex_buffers,
        },
        fragment: Some(FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: PrimitiveState::default(),
        depth_stencil: Some({
            DepthStencilState {
                format: depth_format,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }
        }),
        multisample: MultisampleState::default(),
        multiview: None,
    });

    let mvp_matrix = Mat4::IDENTITY;
    let mvp_matrix_ref: &[f32; 16] = mvp_matrix.as_ref();
    let mvp_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("MVP buffer"),
        contents: bytemuck::cast_slice(mvp_matrix_ref),
        usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let model_matrix = Mat4::IDENTITY;
    let model_matrix_ref: &[f32; 16] = model_matrix.as_ref();
    let model_buffer = device.create_buffer_init(&util::BufferInitDescriptor {
        label: Some("Model buffer"),
        contents: bytemuck::cast_slice(model_matrix_ref),
        usage: BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let bind_group = device.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: mvp_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: model_buffer.as_entire_binding(),
            },
        ],
        label: None,
    });

    let mut depth_texture =
        create_depth_texture(&device, depth_format, config.width, config.height);

    let start = web_time::Instant::now();

    let window = Arc::new(&window);
    event_loop.run(move |event, elwt| {
        let window = window.clone();
        let _ = (&instance, &adapter, &shader, &pipeline_layout);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => elwt.exit(),
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                config.width = new_size.width.max(1);
                config.height = new_size.height.max(1);
                depth_texture =
                    create_depth_texture(&device, depth_format, config.width, config.height);
                surface.configure(&device, &config);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event: KeyEvent { physical_key, .. },
                        ..
                    },
                ..
            } => {
                if physical_key == KeyCode::KeyA {
                    elwt.exit();
                }
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                let time = start.elapsed().as_secs_f32();
                let model_matrix = Mat4::from_rotation_y(time);
                let mvp_matrix = Mat4::perspective_rh(
                    1.57,
                    (config.width as f32) / (config.height as f32),
                    0.001,
                    100.0,
                ) * Mat4::look_at_rh(
                    vec3(1.0, 1.0, 1.0),
                    vec3(0.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                ) * model_matrix;
                let mvp_matrix_ref: &[f32; 16] = mvp_matrix.as_ref();
                let model_matrix_ref: &[f32; 16] = model_matrix.as_ref();
                queue.write_buffer(&mvp_buffer, 0, bytemuck::cast_slice(mvp_matrix_ref));
                queue.write_buffer(&model_buffer, 0, bytemuck::cast_slice(model_matrix_ref));

                let frame = surface
                    .get_current_texture()
                    .expect("Failed to acquire next swap chain texture");
                let view = frame.texture.create_view(&TextureViewDescriptor::default());
                let mut encoder =
                    device.create_command_encoder(&CommandEncoderDescriptor { label: None });
                {
                    let mut rpass = encoder.begin_render_pass(&RenderPassDescriptor {
                        label: None,
                        color_attachments: &[Some(RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: Operations {
                                load: LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.1,
                                    a: 1.0,
                                }),
                                store: StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                            view: &depth_texture,
                            depth_ops: Some(wgpu::Operations {
                                load: wgpu::LoadOp::Clear(1.0),
                                store: wgpu::StoreOp::Discard,
                            }),
                            stencil_ops: None,
                        }),
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });
                    rpass.push_debug_group("Preparing for draw");
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint16);
                    rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    rpass.pop_debug_group();
                    rpass.insert_debug_marker("Draw");
                    rpass.draw_indexed(0..indices.len() as u32, 0, 0..1);
                    //rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(encoder.finish()));
                frame.present();

                window.request_redraw();
            }
            _ => {}
        };
    })?;
    Ok(())
}
