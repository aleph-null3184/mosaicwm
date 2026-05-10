mod state; 
mod wayland;

#[allow(unused)]
use smithay::{
    reexports::{
        calloop::EventLoop,
        wayland_server::{
            Display, 
            backend::ClientData,
            Client, 
            
            protocol::{
                wl_surface::WlSurface,
            },
        },
    },
    wayland::{
        socket::ListeningSocketSource, 
        compositor::{
            CompositorState,
            CompositorHandler,
            CompositorClientState,
        }, 
    },
};
use smithay::delegate_compositor;

use std::{
    sync::Arc,
    collections::HashMap,
};

use crate::state::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the Wayland display is actually the Wayland server
    let display: Display<Mosaic> = Display::new()?;
    // display handle stuff idk man 
    let mut display_handle = display.handle();
    
    // this is the event loop. this is the heart of the compositor.
    // this is a loop which runs, but stays inactive when nothing is
    // given to it. Once a user "throws" input to it, it reacts to it
    let mut event_loop: EventLoop<Mosaic> = EventLoop::try_new()?;
    let handle = event_loop.handle();

    let compositor_state = CompositorState::new::<Mosaic>(&display_handle);

    // this is the state. the state is the brain of the compositor.
    // essentially, what this is is everything the compositor knows
    // now, as in, what windows have been opened in the current moment,
    // what keys have been pressed, what the user has clicked with their
    // mouse, etc. 
    let mut state = Mosaic {
        running: true,
        compositor_state,
        
        surfaces: HashMap::new(),
        needs_repaint: false,
    }; 

    // this is a Unix socket file. it allows stuff to go through it
    // tbh idk what this does bro :wilted_rose: :sob: :pray: 
    let socket = ListeningSocketSource::new_auto()?;
    
    // register socket into loop 
    handle.insert_source(
        socket,
        move |client_stream, _, _state| {
            let _ = display_handle.insert_client(
                client_stream,
                Arc::new(ClientState {
                    compositor_state: CompositorClientState::default(),
                }),
            );
        }
    )?;

    event_loop.run(None, &mut state, |_| {})?;

    Ok(())
}

delegate_compositor!(Mosaic);
