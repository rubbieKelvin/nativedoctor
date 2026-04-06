use nd_core::{
    rhai,
    stream::{events, Session},
};

#[test]
fn session_starts_with_id_and_zero_elapsed() {
    let s = Session::new();
    assert!(!s.session_id().is_empty());
    match &s.events()[0] {
        events::Event::SessionStarted {
            session_id,
            elapsed,
        } => {
            assert_eq!(session_id, s.session_id());
            assert!(elapsed.is_zero());
        }
        e => panic!("expected SessionStarted, got {e:?}"),
    }
}

#[test]
fn emit_stamps_elapsed_finish_appends_ended() {
    let mut s = Session::new();
    let sid = s.session_id().to_string();
    s.emit(|e| events::Event::RuntimeLog {
        level: rhai::LogLevel::Info,
        message: "hi".into(),
        script: "x.rhai".into(),
        elapsed: e,
    });
    let out = s.finish();
    assert_eq!(out.len(), 3);
    assert!(matches!(&out[1], events::Event::RuntimeLog { .. }));
    match &out[2] {
        events::Event::SessionEnded {
            session_id,
            elapsed,
        } => {
            assert_eq!(session_id, &sid);
            assert!(!elapsed.is_zero());
        }
        e => panic!("expected SessionEnded, got {e:?}"),
    }
}

#[test]
fn live_sink_sees_every_event() {
    use std::sync::{Arc, Mutex};
    let n = Arc::new(Mutex::new(0usize));
    let w = n.clone();
    let mut s = Session::with_live_sink(move |_| {
        *w.lock().unwrap() += 1;
    });
    s.emit(|e| events::Event::NewStepEncountered {
        name: "a".into(),
        elapsed: e,
    });
    let _ = s.finish();
    assert_eq!(*n.lock().unwrap(), 3);
}
