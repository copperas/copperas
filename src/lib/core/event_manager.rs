use std::sync::mpsc::{ Receiver, Sender };

use winit::{Event, EventsLoop, DeviceEvent, WindowEvent};

use lib::core::config::Controlls;
use lib::core::message_bus::{
    message::Message,
    message_data::MessageData,
    address::Address
};

pub struct EventManager {
    events_loop: EventsLoop,
    sender:      Sender<Message>,
    receiver:    Receiver<Message>
}

impl EventManager {
    pub fn new(config: &Controlls, snr: (Sender<Message>, Receiver<Message>)) -> Self {
        let (sender, receiver) = snr;
        Self { events_loop: EventsLoop::new(), sender: sender, receiver: receiver }
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
    sender: Sender<Message>
}

impl DeviceEventPorcessor {
    pub fn process(&self, event: DeviceEvent) {}
}

struct WindowEventProcessor {
    sender: Sender<Message>
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
        let message = Message::new(title, data, Address::Core, Address::EventManager);
        &self.sender.send(message).unwrap();
    }
}

pub enum CloseWindow { Close }
