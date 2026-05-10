use smithay::{
    wayland::{
        compositor::{
            CompositorState,
            CompositorHandler,
            CompositorClientState, 
        },

        shell::{
            xdg::XdgShellState,
        }
    },

    reexports::{
        wayland_server::{
            backend::ClientData,
            Client,

            protocol::{
                wl_surface::WlSurface,
            },
        },
    },
};

use std::collections::HashMap;
use crate::wayland::surface::Surface;

pub struct Mosaic {
    pub running: bool,
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,

    pub surfaces: HashMap<WlSurface, Surface>,
    pub needs_repaint: bool,
}

pub struct ClientState {
    pub compositor_state: CompositorClientState,
}

impl ClientData for ClientState {}

impl CompositorHandler for Mosaic {
    fn compositor_state(&mut self) -> &mut CompositorState {
        &mut self.compositor_state
    }

    fn client_compositor_state<'a>(&self, client: &'a Client) -> &'a CompositorClientState {
        &client
            .get_data::<ClientState>()
            .unwrap()
            .compositor_state
    } 

    fn commit(&mut self, wl_surface: &WlSurface) {
        if let Some(surface) = self.surfaces.get_mut(wl_surface) {
            surface.mapped = true;

            println!("Updated existing surface!");
        } else {
            let surface = Surface {
                wl_surface: wl_surface.clone(),
                width: 0,
                height: 0,
                mapped: true,
            };
            
            self.surfaces.insert(wl_surface.clone(), surface);

            println!("Created new surface on commit");
        }

        self.needs_repaint = true;
    }
}
