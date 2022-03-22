use std::time::Instant;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;

// handles all the events.
// you can clone it around as much as you want cause it's small

pub fn get_elapsed(start: std::time::Instant) -> f32 {
    start.elapsed().as_secs() as f32 + start.elapsed().subsec_nanos() as f32 / 1_000_000_000.0
}

pub struct EventHandler {
    pub frame_info: FrameInfo,
    events_loop: EventsLoop,
    last_frame_time: Instant,
    start_time: Instant,
    frames_drawn: u32,
}

// information about the current frame
#[derive(Clone, Debug)]



impl EventHandler {
    pub fn new(events_loop: EventsLoop) -> Self {
        Self {
            frame_info: FrameInfo::empty(),
            events_loop,
            last_frame_time: Instant::now(),
            start_time: Instant::now(),
            frames_drawn: 0,
        }
    }
].frames_drawn as f32) / get_elapsed(self.start_time)
    }

    pub fn avg_delta(&self) -> f32 {
        get_elapsed(self.start_time) / (self.frames_drawn as f32)
    }

    pub fn collect_events(&mut self) -> bool {
        // returns whether the program should exit or not
        // clobbers all input from the last frame, mind
        // also assumes the mouse was at the center of the screen last frame

        // TODO: try and replace these variables with pointers to members of
        // self
        let mut done = false;
        let mut keydowns = vec![];
        let mut keyups = vec![];
        let mut all_events = vec![];
        let mut cursor_pos = None;

        self.events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => done = true,
                Event::WindowEvent {
                    event: WindowEvent::CursorMoved { position: p, .. },
                    ..
                } => {
                    cursor_pos = Some(p);
                }
                Event::WindowEvent {
                    event: WindowEvent::KeyboardInput { .. },
                    ..
                } => {
                    if let Some(keyboard_input) = winit_event_to_keycode(&ev) {
                        match keyboard_input {
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: winit::ElementState::Pressed,
                                ..
                            } => keydowns.push(key),
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: winit::ElementState::Released,
                                ..
                            } => keyups.push(key),
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
            all_events.push(ev.clone());
        });
