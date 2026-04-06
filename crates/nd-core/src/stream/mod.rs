use std::time::{Duration, Instant};

pub mod events;

/// In-process timeline for one run: stamps monotonic [`Duration`]s and optionally forwards each event (SSE, channels).
pub struct Session {
    id: String,
    start: Instant,
    events: Vec<events::Event>,
    live: Option<Box<dyn FnMut(events::Event) + Send>>,
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

impl Session {
    /// New session: generates an id, records [`events::Event::SessionStarted`] at `elapsed == 0`.
    pub fn new() -> Self {
        Self::with_live_opt(None)
    }

    /// Same as [`Self::new`], plus a hook invoked for every event (including the initial `SessionStarted`).
    ///
    /// The hook receives a clone suitable for streaming; the session also retains a full copy in [`Self::events`].
    pub fn with_live_sink(sink: impl FnMut(events::Event) + Send + 'static) -> Self {
        Self::with_live_opt(Some(Box::new(sink)))
    }

    fn with_live_opt(live: Option<Box<dyn FnMut(events::Event) + Send>>) -> Self {
        let session_id = nanoid::nanoid!();
        let start = Instant::now();

        let mut instance = Self {
            id: session_id,
            start,
            events: Vec::new(),
            live,
        };

        instance.record(events::Event::SessionStarted {
            session_id: instance.id.clone(),
            elapsed: Duration::ZERO,
        });
        return instance;
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
    pub fn finish(mut self) -> Vec<events::Event> {
        let elapsed = self.start.elapsed();
        let end = events::Event::SessionEnded {
            session_id: self.id.clone(),
            elapsed,
        };
        self.record(end);
        self.events
    }

    /// Consume the buffer without appending `SessionEnded` (e.g. after fatal error you already recorded).
    pub fn into_events(self) -> Vec<events::Event> {
        self.events
    }

    fn record(&mut self, event: events::Event) {
        if let Some(cb) = self.live.as_mut() {
            cb(event.clone());
        }
        self.events.push(event);
    }
}
