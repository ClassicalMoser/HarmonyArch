//! Simple WGPU 3D viewer
//! 
//! This viewer displays 3D geometry using WGPU with proper 3D rendering.

use crate::domain::Element;
use crate::infrastructure::GeometryUtils;
use eframe::egui;
use wgpu::util::DeviceExt;
use std::sync::Arc;

/// Simple WGPU 3D viewer
pub struct SimpleWgpuViewer {
    elements: Vec<Element>,
    camera_distance: f32,
    camera_rotation_x: f32,
    camera_rotation_y: f32,
    show_wireframe: bool,
    show_normals: bool,
    
    // WGPU state
    device: Option<Arc<wgpu::Device>>,
    queue: Option<Arc<wgpu::Queue>>,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    num_indices: u32,
    uniform_buffer: Option<wgpu::Buffer>,
    uniform_bind_group: Option<wgpu::BindGroup>,
    
    // Render target
    render_texture: Option<wgpu::Texture>,
    render_texture_view: Option<wgpu::TextureView>,
    depth_texture: Option<wgpu::Texture>,
    depth_texture_view: Option<wgpu::TextureView>,
    
    // Viewport size
    viewport_width: u32,
    viewport_height: u32,
    
    // Texture bridge (for WGPU integration)
    rendered_image: Option<egui::ColorImage>,
    
    // Initialization state
    wgpu_initialized: bool,
    wgpu_initializing: bool,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_projection: [[f32; 4]; 4],
    camera_position: [f32; 3],
    _padding: f32,
}

impl SimpleWgpuViewer {
    /// Create a new WGPU viewer
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            camera_distance: 10.0,
            camera_rotation_x: 0.0,
            camera_rotation_y: 0.0,
            show_wireframe: false,
            show_normals: false,
            
            // WGPU state
            device: None,
            queue: None,
            render_pipeline: None,
            vertex_buffer: None,
            index_buffer: None,
            num_indices: 0,
            uniform_buffer: None,
            uniform_bind_group: None,
            
            // Render target
            render_texture: None,
            render_texture_view: None,
            depth_texture: None,
            depth_texture_view: None,
            
            // Viewport size
            viewport_width: 400,
            viewport_height: 300,
            
            // Texture bridge (for WGPU integration)
            rendered_image: None,
            
            // Initialization state
            wgpu_initialized: false,
            wgpu_initializing: false,
        }
    }



    /// Initialize WGPU (async version)
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        }).await.ok_or("Failed to find an appropriate adapter")?;
        
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        ).await?;
        
        let device = Arc::new(device);
        let queue = Arc::new(queue);
        
        // Create render textures
        self.create_render_targets(&device)?;
        
        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/simple_3d.wgsl").into()),
        });
        
        // Create uniform buffer
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        // Create bind group layout
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        
        // Create bind group
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });
        
        // Create render pipeline layout
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        // Create render pipeline
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        
        self.device = Some(device);
        self.queue = Some(queue);
        self.render_pipeline = Some(render_pipeline);
        self.uniform_buffer = Some(uniform_buffer);
        self.uniform_bind_group = Some(uniform_bind_group);
        
        Ok(())
    }

    /// Create render target textures
    fn create_render_targets(&mut self, device: &wgpu::Device) -> Result<(), Box<dyn std::error::Error>> {
        // Create color texture
        let render_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Render Texture"),
            size: wgpu::Extent3d {
                width: self.viewport_width,
                height: self.viewport_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        
        let render_texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        // Create depth texture
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: self.viewport_width,
                height: self.viewport_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        self.render_texture = Some(render_texture);
        self.render_texture_view = Some(render_texture_view);
        self.depth_texture = Some(depth_texture);
        self.depth_texture_view = Some(depth_texture_view);
        
        Ok(())
    }

    /// Update the elements to display
    pub fn update_elements(&mut self, elements: Vec<Element>) {
        self.elements = elements;
        self.update_geometry();
    }
        
    /// Update geometry buffers
    fn update_geometry(&mut self) {
        if let Some(device) = &self.device {
            let (vertices, indices) = self.elements_to_gpu_geometry();
            
            self.vertex_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }));
            
            self.index_buffer = Some(device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            }));
            
            self.num_indices = indices.len() as u32;
        }
    }

    /// Render the 3D scene
    pub fn render(&mut self, ui: &mut egui::Ui) {
        // Wrap everything in a scrollable area
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.heading("3D WGPU Viewer");
            
            // Compact camera controls
            ui.horizontal(|ui| {
                ui.label("Distance:");
                if ui.add(egui::Slider::new(&mut self.camera_distance, 1.0..=50.0)).changed() {
                    ui.ctx().request_repaint();
                }
                ui.label("Rotation X:");
                if ui.add(egui::Slider::new(&mut self.camera_rotation_x, -180.0..=180.0)).changed() {
                    ui.ctx().request_repaint();
                }
                ui.label("Rotation Y:");
                if ui.add(egui::Slider::new(&mut self.camera_rotation_y, -180.0..=180.0)).changed() {
                    ui.ctx().request_repaint();
                }
            });
            
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_wireframe, "Wireframe");
                ui.checkbox(&mut self.show_normals, "Show Normals");
                
                if ui.button("Re-render").clicked() {
                    ui.ctx().request_repaint();
                }
                
                if !self.wgpu_initialized && !self.wgpu_initializing {
                    if ui.button("Initialize 3D").clicked() {
                        self.wgpu_initializing = true;
                        ui.ctx().request_repaint();
                        
                        let rt = pollster::block_on(async {
                            self.initialize().await
                        });
                        
                        match rt {
                            Ok(_) => {
                                self.wgpu_initializing = false;
                                self.wgpu_initialized = true;
                                ui.ctx().request_repaint();
                            }
                            Err(e) => {
                                eprintln!("Failed to initialize WGPU: {}", e);
                                self.wgpu_initializing = false;
                            }
                        }
                    }
                } else if self.wgpu_initializing {
                    ui.label("Initializing WGPU...");
                }
            });
            
            ui.separator();
            
            // 3D Viewport with smaller size
            let (response, _painter) = ui.allocate_painter(
                egui::vec2(400.0, 250.0), // Reduced height
                egui::Sense::drag(),
            );
            
            // Update viewport size if needed
            let new_width = response.rect.width() as u32;
            let new_height = response.rect.height() as u32;
            if new_width != self.viewport_width || new_height != self.viewport_height {
                self.viewport_width = new_width;
                self.viewport_height = new_height;
            }
            
                    // Draw 3D scene
        self.draw_3d_scene(ui);
        
        // Debug info
        ui.label(format!("WGPU Initialized: {}", self.wgpu_initialized));
        ui.label(format!("Elements loaded: {}", self.elements.len()));
        if let Some(image) = &self.rendered_image {
            ui.label(format!("Image available: {}x{}", image.size[0], image.size[1]));
        } else {
            ui.label("No image available");
        }
            
            ui.separator();
            
            // Compact statistics
            ui.horizontal(|ui| {
                ui.label(format!("Elements: {}", self.elements.len()));
                ui.label(format!("Triangles: {}", self.elements.len() * 12));
            });
            
            // Collapsible element details
            ui.collapsing("Element Details", |ui| {
                for element in &self.elements {
                    ui.group(|ui| {
                        ui.label(format!("Element: {}", element.id));
                        ui.label(format!("Type: {:?}", element.element_type));
                        ui.label(format!("Position: ({:.2}, {:.2}, {:.2})", 
                            element.position.x, element.position.y, element.position.z));
                        ui.label(format!("Dimensions: ({:.2}, {:.2}, {:.2})", 
                            element.dimensions.x, element.dimensions.y, element.dimensions.z));
                        ui.label(format!("Rotation: {:.1}°", element.rotation_degrees));
                        
                        if self.show_normals {
                            ui.label(format!("Normal: ({:.2}, {:.2}, {:.2})", 
                                element.normal.x, element.normal.y, element.normal.z));
                        }
                    });
                }
            });
        });
    }

    /// Draw the 3D scene
    fn draw_3d_scene(&mut self, ui: &mut egui::Ui) {
        if self.elements.is_empty() {
            ui.label("No elements loaded");
            return;
        }
        
        // Check if WGPU is initialized
        if !self.wgpu_initialized {
            // Show initialization button
            ui.label("WGPU not initialized\nClick 'Initialize 3D' to start rendering");
            return;
        }
        
        // Try WGPU rendering
        if let Err(e) = self.render_with_wgpu() {
            // Show error details
            ui.label(format!("WGPU Error:\n{}", e));
        } else {
            // Display the rendered texture
            self.display_rendered_texture(ui);
        }
    }



    /// Convert elements to GPU geometry
    fn elements_to_gpu_geometry(&self) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;
        
        for element in &self.elements {
            let triangles = GeometryUtils::element_to_triangles(element);
            
            for triangle in triangles {
                // Calculate normal for this triangle
                let normal = self.calculate_triangle_normal(&triangle);
                
                for vertex in triangle {
                    vertices.push(Vertex {
                        position: [vertex.x as f32, vertex.y as f32, vertex.z as f32],
                        normal: [normal.x as f32, normal.y as f32, normal.z as f32],
                    });
                }
                
                indices.push(index_offset);
                indices.push(index_offset + 1);
                indices.push(index_offset + 2);
                index_offset += 3;
            }
        }
        
        (vertices, indices)
    }

    /// Calculate normal for a triangle
    fn calculate_triangle_normal(&self, triangle: &[nalgebra::Vector3<f64>; 3]) -> nalgebra::Vector3<f64> {
        let v0 = triangle[0];
        let v1 = triangle[1];
        let v2 = triangle[2];
        
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        
        edge1.cross(&edge2).normalize()
    }

    /// Render with WGPU
    fn render_with_wgpu(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.device.is_none() {
            return Err("WGPU device not initialized - need to implement async initialization".into());
        }
        
        if let (Some(device), Some(queue), Some(render_pipeline), Some(uniform_buffer), Some(uniform_bind_group)) = 
            (&self.device, &self.queue, &self.render_pipeline, &self.uniform_buffer, &self.uniform_bind_group) {
            
            // Calculate view-projection matrix
            let uniforms = self.calculate_uniforms();
            
            // Update uniform buffer
            queue.write_buffer(uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            
            // Create command encoder
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
            
            // Begin render pass
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: self.render_texture_view.as_ref().unwrap(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                        view: self.depth_texture_view.as_ref().unwrap(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Store,
                        }),
                        stencil_ops: None,
                    }),
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                
                render_pass.set_pipeline(render_pipeline);
                render_pass.set_bind_group(0, uniform_bind_group, &[]);
                
                if let (Some(vertex_buffer), Some(index_buffer)) = (self.vertex_buffer.as_ref(), self.index_buffer.as_ref()) {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
                }
            }
            
            // Submit commands
            queue.submit(std::iter::once(encoder.finish()));
            
            Ok(())
        } else {
            Err("WGPU not initialized".into())
        }
    }

    /// Display the rendered texture
    fn display_rendered_texture(&mut self, ui: &mut egui::Ui) {
        if let (Some(device), Some(queue), Some(texture)) = 
            (&self.device, &self.queue, &self.render_texture) {
            
            // Copy texture to CPU buffer
            let bytes_per_row = self.viewport_width * 4;
            let aligned_bytes_per_row = ((bytes_per_row + 255) / 256) * 256; // Align to 256 bytes
            let buffer_size = aligned_bytes_per_row * self.viewport_height;
            let buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Texture Copy Buffer"),
                size: buffer_size as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            
            // Copy texture to buffer
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Texture Copy Encoder"),
            });
            
            // Calculate aligned bytes per row
            let bytes_per_row = self.viewport_width * 4;
            let aligned_bytes_per_row = ((bytes_per_row + 255) / 256) * 256; // Align to 256 bytes
            
            encoder.copy_texture_to_buffer(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::ImageCopyBuffer {
                    buffer: &buffer,
                    layout: wgpu::ImageDataLayout {
                        offset: 0,
                        bytes_per_row: Some(aligned_bytes_per_row),
                        rows_per_image: Some(self.viewport_height),
                    },
                },
                wgpu::Extent3d {
                    width: self.viewport_width,
                    height: self.viewport_height,
                    depth_or_array_layers: 1,
                },
            );
            
            queue.submit(std::iter::once(encoder.finish()));
            
            // Map buffer and read data
            let buffer_slice = buffer.slice(..);
            let (tx, rx) = std::sync::mpsc::channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                let _ = tx.send(result);
            });
            
            device.poll(wgpu::Maintain::Wait);
            
            if let Ok(Ok(())) = rx.recv() {
                let data = buffer_slice.get_mapped_range();
                let pixels: Vec<u8> = data.iter().copied().collect();
                drop(data);
                buffer.unmap();
                
                // Convert aligned data to unaligned for egui
                let mut unaligned_pixels = Vec::new();
                let aligned_bytes_per_row = ((self.viewport_width * 4 + 255) / 256) * 256;
                let actual_bytes_per_row = self.viewport_width * 4;
                
                for row in 0..self.viewport_height {
                    let row_start = (row * aligned_bytes_per_row) as usize;
                    let row_end = row_start + actual_bytes_per_row as usize;
                    unaligned_pixels.extend_from_slice(&pixels[row_start..row_end]);
                }
                
                // Convert to egui image
                let image = egui::ColorImage::from_rgba_unmultiplied(
                    [self.viewport_width as usize, self.viewport_height as usize],
                    &unaligned_pixels,
                );
                
                // Store the image for display
                self.rendered_image = Some(image);
            } else {
                // Fallback if texture copy failed
                ui.label("Texture copy failed");
            }
        } else {
            ui.label("No texture available");
        }
        
        // Display the rendered image if available
        if let Some(image) = &self.rendered_image {
            // Load the texture and get the handle
            let texture_handle = ui.ctx().load_texture("wgpu_rendered_texture", image.clone(), Default::default());
            
            // Display the image using the texture handle
            ui.image(&texture_handle);
        }
    }

    /// Calculate uniforms for 3D rendering
    fn calculate_uniforms(&self) -> Uniforms {
        let aspect_ratio = self.viewport_width as f32 / self.viewport_height as f32;
        
        // Simple perspective projection
        let fov = 45.0_f32.to_radians();
        let near = 0.1;
        let far = 100.0;
        
        let projection = self.perspective_matrix(fov, aspect_ratio, near, far);
        let view = self.view_matrix();
        let view_projection = self.multiply_matrices(projection, view);
        
        // Camera position
        let camera_pos = self.calculate_camera_position();
        
        Uniforms {
            view_projection,
            camera_position: camera_pos,
            _padding: 0.0,
        }
    }

    /// Calculate perspective projection matrix
    fn perspective_matrix(&self, fov: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
        let f = 1.0 / (fov / 2.0).tan();
        
        [
            [f / aspect, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, (far + near) / (near - far), (2.0 * far * near) / (near - far)],
            [0.0, 0.0, -1.0, 0.0],
        ]
    }

    /// Calculate view matrix (orbit camera)
    fn view_matrix(&self) -> [[f32; 4]; 4] {
        let camera_pos = self.calculate_camera_position();
        
        // Simple look-at matrix
        let target = [0.0, 0.0, 0.0];
        let up = [0.0, 1.0, 0.0];
        
        // Forward vector
        let forward = [
            target[0] - camera_pos[0],
            target[1] - camera_pos[1],
            target[2] - camera_pos[2],
        ];
        let forward_len = (forward[0] * forward[0] + forward[1] * forward[1] + forward[2] * forward[2]).sqrt();
        let forward = [forward[0] / forward_len, forward[1] / forward_len, forward[2] / forward_len];
        
        // Right vector
        let right = [
            forward[1] * up[2] - forward[2] * up[1],
            forward[2] * up[0] - forward[0] * up[2],
            forward[0] * up[1] - forward[1] * up[0],
        ];
        let right_len = (right[0] * right[0] + right[1] * right[1] + right[2] * right[2]).sqrt();
        let right = [right[0] / right_len, right[1] / right_len, right[2] / right_len];
        
        // Up vector
        let up_corrected = [
            right[1] * forward[2] - right[2] * forward[1],
            right[2] * forward[0] - right[0] * forward[2],
            right[0] * forward[1] - right[1] * forward[0],
        ];
        
        [
            [right[0], up_corrected[0], -forward[0], 0.0],
            [right[1], up_corrected[1], -forward[1], 0.0],
            [right[2], up_corrected[2], -forward[2], 0.0],
            [-right[0] * camera_pos[0] - right[1] * camera_pos[1] - right[2] * camera_pos[2],
             -up_corrected[0] * camera_pos[0] - up_corrected[1] * camera_pos[1] - up_corrected[2] * camera_pos[2],
             forward[0] * camera_pos[0] + forward[1] * camera_pos[1] + forward[2] * camera_pos[2], 1.0],
        ]
    }

    /// Calculate camera position from angles and distance
    fn calculate_camera_position(&self) -> [f32; 3] {
        let rot_x = self.camera_rotation_x.to_radians();
        let rot_y = self.camera_rotation_y.to_radians();
        
        let x = self.camera_distance * rot_y.sin() * rot_x.cos();
        let y = self.camera_distance * rot_x.sin();
        let z = self.camera_distance * rot_y.cos() * rot_x.cos();
        
        [x, y, z]
    }

    /// Multiply two 4x4 matrices
    fn multiply_matrices(&self, a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
} 