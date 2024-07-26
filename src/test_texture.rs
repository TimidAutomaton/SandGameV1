#[derive(Debug)]
pub struct TestTexture {
    pub bind_group: wgpu::BindGroup,
    pub dimensions: (usize, usize),
    pub data: Vec<u8>,
}


impl TestTexture {
    pub fn new(texture_map: &str, device: &wgpu::Device, queue: &wgpu::Queue, texture_bind_group_layout: &wgpu::BindGroupLayout) -> Self{

        use std::env;
        use std::ffi::OsStr;
        let file = OsStr::new(texture_map);
        let mut dir = env::current_dir().unwrap();
        dir.push("assets");
        dir.set_file_name(file);
        

        let fullpath = "./assets/".to_owned() + texture_map;

        let diffuse_image = image::open(fullpath).unwrap();
        let diffuse_rgba = match diffuse_image {
            image::DynamicImage::ImageRgba8(diffuse_image) => diffuse_image,
            x => x.to_rgba8()
        };


        let new_width = diffuse_rgba.width() as usize;
        let new_height = diffuse_rgba.height() as usize;
        let data = diffuse_rgba.clone();
        let data = data.into_raw();

        let dimensions = (new_width, new_height);

        let texture_size = wgpu::Extent3d {
            width: dimensions.0 as u32,
            height: dimensions.1 as u32,
            depth_or_array_layers: 1,
        };


        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            }
        );

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &diffuse_rgba,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0 as u32),
                rows_per_image: Some(dimensions.1 as u32),
            },
            texture_size,
        );


        // We don't need to configure the texture view much, so let's
        // let wgpu define it.
        let diffuse_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });


        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                //label: Some("texture_bind_group"),
                label: Some(texture_map),
            }
        );


        Self{
            bind_group,
            dimensions,
            data,
        }
        
    }

    pub fn make_texture_from_raw_bytes(name: &str, raw_bytes: &Vec<u8>, texture_dimensions: [u32; 2], device: &wgpu::Device, queue: &wgpu::Queue, texture_bind_group_layout: &wgpu::BindGroupLayout) -> Self{

        let mut data = vec![0; raw_bytes.len()];

        data.copy_from_slice(&raw_bytes[..]);

        let texture_size = wgpu::Extent3d {
            width: texture_dimensions[0],
            height: texture_dimensions[1],
            depth_or_array_layers: 1,
        };


        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size,
                mip_level_count: 1, // We'll talk about this a little later
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
                view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            }
        );

        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            &raw_bytes,
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * texture_dimensions[0]),
                rows_per_image: Some(texture_dimensions[1]),
            },
            texture_size,
        );

        // We don't need to configure the texture view much, so let's
        // let wgpu define it.
        let diffuse_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });


        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&diffuse_view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                    }
                ],
                label: Some("name"),
            }
        );

        let mut empty_list = Vec::new();
        empty_list.push(([0.0, 0.0], [1.0, 0.0], [0.0, 1.0], [1.0, 1.0]));

        Self{
            bind_group,
            dimensions: (texture_dimensions[0] as usize, texture_dimensions[1] as usize),
            data,
        }
        
    }
}