use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::env::RuntimeEnv;

pub mod events;

/// In-process timeline for one run: stamps monotonic [`Duration`]s and optionally forwards each event (SSE, channels).
pub struct Session {
    id: String,
    start: Instant,
    events: Vec<events::Event>,
    pub runtime: RuntimeEnv,
    live: Option<Box<dyn FnMut(events::Event) + Send>>,
}

impl Session {
    /// New session: generates an id, records [`events::Event::SessionStarted`] at `elapsed == 0`.
    pub fn new(
        runtime: impl FnOnce() -> Result<RuntimeEnv, String>,
        sink: Option<Box<dyn FnMut(events::Event) + Send>>,
    ) -> Result<Self, String> {
        let session_id = nanoid::nanoid!();
        let start = Instant::now();

        let mut instance = Self {
            id: session_id,
            start,
            events: Vec::new(),
            live: sink,
            runtime: runtime()?,
        };

        instance.record(events::Event::SessionStarted {
            session_id: instance.id.clone(),
            elapsed: Duration::ZERO,
        });

        let entries = instance.runtime.entries();
        instance.emit(|e| events::Event::RuntimeVariablesInitialized {
            elapsed: e,
            entries,
        });

        return Ok(instance);
    }

    pub fn session_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    pub fn events(&self) -> &[events::Event] {
        self.events.as_slice()
    }

    /// Build an event with the current session-relative [`Duration`] (preferred over [`Self::push`]).
    pub fn emit(&mut self, f: impl FnOnce(Duration) -> events::Event) {
        let event = f(self.start.elapsed());
        self.record(event);
    }

    /// Append a pre-built event (use when `elapsed` is computed elsewhere; otherwise prefer [`Self::emit`]).
    pub fn push(&mut self, event: events::Event) {
        self.record(event);
    }

    /// Appends [`events::Event::SessionEnded`] and returns the full timeline (including the end marker).
    pub fn finish(&mut self) -> Vec<events::Event> {
        let elapsed = self.start.elapsed();
        let end = events::Event::SessionEnded {
            session_id: self.id.clone(),
            elapsed,
        };
        self.record(end);
        return std::mem::take(&mut self.events);
    }

    /// Consume the buffer without appending `SessionEnded` (e.g. after fatal error you already recorded).
    pub fn into_events(self) -> Vec<events::Event> {
        self.events
    }

    fn record(&mut self, event: events::Event) {
        if let Some(cb) = self.live.as_mut() {
            cb(event.clone());
        }
        return self.events.push(event);
    }

    pub fn reload_runtime(&mut self) {
        self.runtime.clear();
        let entries = self.runtime.entries();

        self.emit(|e| events::Event::RuntimeVariablesInitialized {
            elapsed: e,
            entries,
        });
    }
}

/// [`Session::emit`] and related helpers for a shared session (e.g. Rhai host functions, HTTP runner).
pub trait MutexSession {
    fn id(&self) -> String;
    fn emit(&self, f: impl FnOnce(Duration) -> events::Event);
    fn reload_runtime(&self);
    fn runtime(&self) -> RuntimeEnv;
}

impl MutexSession for Arc<Mutex<Session>> {
    fn id(&self) -> String {
        let session = self.lock().expect("session mutex poisoned");
        return session.id.clone();
    }
    fn emit(&self, f: impl FnOnce(Duration) -> events::Event) {
        let mut session = self.lock().expect("session mutex poisoned");
        session.emit(f);
    }

    fn reload_runtime(&self) {
        let mut session = self.lock().expect("session mutex poisoned");
        session.reload_runtime();
    }

    fn runtime(&self) -> RuntimeEnv {
        let session = self.lock().expect("session mutex poisoned");
        return session.runtime.clone();
    }
}

impl MutexSession for &Arc<Mutex<Session>> {
    fn id(&self) -> String {
        return Arc::clone(self).id();
    }

    fn emit(&self, f: impl FnOnce(Duration) -> events::Event) {
        Arc::clone(self).emit(f);
    }

    fn reload_runtime(&self) {
        Arc::clone(self).reload_runtime();
    }

    fn runtime(&self) -> RuntimeEnv {
        Arc::clone(self).runtime()
    }
}
