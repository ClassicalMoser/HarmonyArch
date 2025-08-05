//! Infrastructure layer - External concerns like file I/O and rendering

pub mod geometry;
pub mod renderers;
pub mod file_io;
pub mod wgpu_renderer;

pub use renderers::{StlRenderer, SvgRenderer};
pub use geometry::GeometryUtils;
pub use file_io::FileWriter;
pub use wgpu_renderer::WgpuRenderer;