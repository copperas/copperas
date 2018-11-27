use std::sync::mpsc::SyncSender;

use winit::{Event, EventsLoop, DeviceEvent, WindowEvent};

use lib::core::config::Controlls;
use lib::core::message_bus::{ Message, MessageData};

pub struct EventManager {
    events_loop: EventsLoop,
    sender:      SyncSender<Message>
}

impl EventManager {
    pub fn new(config: &Controlls, sender: SyncSender<Message>) -> Self {
        Self { events_loop: EventsLoop::new(), sender: sender }
    }

    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }

    pub fn manage_events(&mut self) {
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
        DeviceEventPorcessor { sender: self.sender.clone() }
    }

    fn new_window_event_processor(&self) -> WindowEventProcessor {
        WindowEventProcessor { sender: self.sender.clone() }
    }

}

struct DeviceEventPorcessor {
    sender: SyncSender<Message>
}

impl DeviceEventPorcessor {
    pub fn process(&self, event: DeviceEvent) {}
}

struct WindowEventProcessor {
    sender: SyncSender<Message>
}

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
        let data    = Box::new(CloseWindow::Close);
        let message = Message::new(title, data);
        &self.sender.send(message).unwrap();
    }
}

pub enum CloseWindow { Close }
