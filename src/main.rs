mod state; 

use smithay::{
    reexports::{
        calloop::EventLoop,
        wayland_server::{
            Display, 
            backend::ClientData,
        },
    },
    wayland::{
        socket::ListeningSocketSource, 
        compositor::{CompositorState, CompositorHandler}, 
    },
};
use smithay::delegate_compositor;

use std::sync::Arc;

use crate::state::Mosaic;

struct ClientState;

impl ClientData for ClientState {}

impl CompositorHandler for Mosaic {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // the Wayland display is actually the Wayland server
    let display: Display<Mosaic> = Display::new()?;
    // display handle stuff idk man 
    let mut display_handle = display.handle();

    let compositor_state = CompositorState::new(&display_handle());

    // this is the state. the state is the brain of the compositor.
    // essentially, what this is is everything the compositor knows
    // now, as in, what windows have been opened in the current moment,
    // what keys have been pressed, what the user has clicked with their
    // mouse, etc. 
    let mut state = Mosaic {
        running: true,
    }; 

    // this is the event loop. this is the heart of the compositor.
    // this is a loop which runs, but stays inactive when nothing is
    // given to it. Once a user "throws" input to it, it reacts to it
    let mut event_loop: EventLoop<Mosaic> = EventLoop::try_new()?;
    let handle = event_loop.handle();

    // this is a Unix socket file. it allows stuff to go through it
    // tbh idk what this does bro :wilted_rose: :sob: :pray: 
    let socket = ListeningSocketSource::new_auto()?;
    
    // register socket into loop 
    handle.insert_source(
        socket,
        move |client_stream, _, state| {
            display_handle.insert_client(
                client_stream,
                Arc::new(ClientState),
            );
        }
    )?;

    event_loop.run(None, &mut state, |_| {})?;

    Ok(())
}

delegate_compositor!(Mosaic);
