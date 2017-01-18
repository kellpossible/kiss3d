use std::sync::mpsc::Receiver;
use std::rc::Rc;
use std::cell::RefCell;
use glutin;

/// An event.
pub struct Event<'a> {
    /// The event timestamp.
    pub timestamp: f64,
    /// The exact glfw event value. This can be modified to fool the other event handlers.
    pub value:     glutin::Event,
    /// Set this to `true` to prevent the window or the camera from handling the event.
    pub inhibited: bool,
    inhibitor:     &'a RefCell<Vec<glutin::Event>>
}

impl<'a> Drop for Event<'a> {
    #[inline]
    fn drop(&mut self) {
        if !self.inhibited {
            self.inhibitor.borrow_mut().push(self.value.clone())
        }
    }
}

impl<'a> Event<'a> {
    #[inline]
    fn new(timestamp: f64,
           value:     glutin::Event,
           inhibitor: &RefCell<Vec<glutin::Event>>)
           -> Event {
        Event {
            timestamp: timestamp,
            value:     value,
            inhibited: false,
            inhibitor: inhibitor
        }
    }
}

/// An iterator through events.
pub struct Events<'a> {
    stream:    glfw::FlushedMessages<'a, (f64, glutin::Event)>,
    inhibitor: &'a RefCell<Vec<glutin::Event>>
}

impl<'a> Events<'a> {
    #[inline]
    fn new(stream:    glfw::FlushedMessages<'a, (f64, glutin::Event)>,
           inhibitor: &'a RefCell<Vec<glutin::Event>>)
           -> Events<'a> {
        Events {
            stream:    stream,
            inhibitor: inhibitor
        }
    }
}


impl<'a> Iterator for Events<'a> {
    type Item = Event<'a>;

    #[inline]
    fn next(&mut self) -> Option<Event<'a>> {
        match self.stream.next() {
            None         => None,
            Some((t, e)) => Some(Event::new(t, e, self.inhibitor))
        }
    }
}

/// A stand-alone object that provides an iterator though glfw events.
///
/// It is not lifetime-bound to the main window.
pub struct EventManager {
    events:    Rc<Receiver<(f64, glutin::Event)>>,
    inhibitor: Rc<RefCell<Vec<glutin::Event>>>
}

impl EventManager {
    /// Creates a new event manager.
    #[inline]
    pub fn new(events:    Rc<Receiver<(f64, glutin::Event)>>,
               inhibitor: Rc<RefCell<Vec<glutin::Event>>>)
               -> EventManager {
        EventManager {
            events:    events,
            inhibitor: inhibitor
        }
    }

    /// Gets an iterator to the glfw events already collected.
    #[inline]
    pub fn iter(&mut self) -> Events {
        Events::new(glfw::flush_messages(&*self.events), &*self.inhibitor)
    }
}
