use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;

pub struct Surface {
    /// handle to the Wayland surface
    pub wl_surface: WlSurface,
    
    /// Current size (may be unknown until first buffer commit)
    pub width: u32,
    pub height: u32,
    
    /// whether this surface has ever commited valid content (via commit())
    pub mapped: bool,
}

impl Surface {
    pub fn new(wl_surface: WlSurface) -> Self {
        Self {
            wl_surface,
            width: 0,
            height: 0,
            mapped: false,
        }
    }

    pub fn apply_commit(&mut self) {
        // read buffer size
        // update damage
        // mark mapped 
        self.mapped = true;
    }

    pub fn is_mapped(&self) -> bool {
        self.mapped 
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height; 
    }
}
