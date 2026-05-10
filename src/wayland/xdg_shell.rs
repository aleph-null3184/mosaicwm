use smithay::{
    delegate_xdg_shell,

    wayland::shell::xdg::{
        XdgShellHandler,
        XdgShellState,
        ToplevelSurface,
        PopupSurface,
        PositionerState,
    },

    reexports::{
        wayland_server::{
            protocol::wl_seat::WlSeat,
        },
    },

    utils::Serial,
};

use crate::state::Mosaic;

impl XdgShellHandler for Mosaic {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState {
        &mut self.xdg_shell_state
    }

    fn new_toplevel(
        &mut self,
        surface: ToplevelSurface,
    ) {
        println!("New toplevel window");

        // store window

        // map window 
        
        // assign geometry
    }

    fn new_popup(
        &mut self,
        surface: PopupSurface,
        positioner: PositionerState,
    ) {
        println!("New popup!");
    }

    fn reposition_request(&mut self, surface: PopupSurface, positioner: PositionerState, token: u32) {
        surface.with_pending_state(|state| {
            let geometry = positioner.get_geometry();
            state.geometry = geometry;
            state.positioner = positioner; 
        });
        self.unconstrain_popup(&surface);
        surface.send_repositioned(token);
    }

    fn grab(
        &mut self,
        surface: PopupSurface,
        seat: WlSeat,
        serial: Serial
    ) {
        println!("Popup grab");
    }
}

delegate_xdg_shell!(Mosaic);
