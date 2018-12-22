use std::sync::mpsc::{ Receiver, Sender };

use winit::{Event, EventsLoop, DeviceEvent, WindowEvent};

use lib::core::config::Controlls;

pub struct EventManager {
    events_loop: EventsLoop
}

impl EventManager {
    pub fn new(config: &Controlls) -> Self {
        Self { events_loop: EventsLoop::new() }
    }

    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }

    pub fn manage_events(&mut self, mut running: bool) {
        let a: u64 = 345;
        let wep = self.new_window_event_processor();
        let dep = self.new_device_event_processor();
        &self.events_loop.poll_events(|event| {
            // println!("{:?}", event);
            match event {
                Event::DeviceEvent { event, ..}  => dep.process(event),
                Event::WindowEvent { event, .. } => wep.process(event),
                _ => ()
            };
        });
    }

    fn new_device_event_processor(&self) -> DeviceEventPorcessor {
        DeviceEventPorcessor {}
    }

    fn new_window_event_processor(&self) -> WindowEventProcessor {
        WindowEventProcessor {}
    }

}

struct DeviceEventPorcessor;

impl DeviceEventPorcessor {
    pub fn process(&self, event: DeviceEvent) {}
}

struct WindowEventProcessor {}

impl WindowEventProcessor {
    pub fn process(&self, event: WindowEvent) {
        println!("{:?}", event);
        match event {
            WindowEvent::CloseRequested => self.close_requested(),
            _ => ()
        }
    }

    fn close_requested(&self) {
        let title   = String::from("Close Requested");
        let data    = EventMessage { data: CloseWindow::Close };
        let message = Message::new(title, data, Address::Core, Address::EventManager);
        &self.sender.send(message).unwrap();
    }
}

pub enum CloseWindow { Close }

pub struct EventMessage {

}
