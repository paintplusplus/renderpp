pub struct WgpuSurface<'a> {}

impl WgpuSurface {
    pub fn new() -> Self {
        WgpuSurface {}
    }
}

pub fn create_surface() -> WgpuSurface {
    WgpuSurface::new()
}
