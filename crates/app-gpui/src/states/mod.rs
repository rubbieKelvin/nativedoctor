use gpui::{App, AppContext, Entity, Global};

#[derive(Default, Clone, Copy)]
pub struct AppState {
    pub count: u32,
}

#[derive(Clone)]
struct GlobalAppState(Entity<AppState>);

impl Global for GlobalAppState {}

impl AppState {
    pub fn global(cx: &mut App) -> Entity<Self> {
        cx.try_global::<GlobalAppState>()
            .map(|global| global.0.clone())
            .unwrap_or_else(|| {
                let entity = cx.new(|_| Self::default());
                cx.set_global(GlobalAppState(entity.clone()));
                entity
            })
    }

    pub fn try_global(cx: &App) -> Option<Entity<Self>> {
        cx.try_global::<GlobalAppState>()
            .map(|global| global.0.clone())
    }

    // Convenience methods for common operations
    pub fn increment_counter(cx: &mut App) {
        let state = Self::global(cx);
        state.update(cx, |state, cx| {
            // state.counter += 1;
            cx.notify(); // Important: notify observers of changes
        });
    }

    pub fn decrement_counter(cx: &mut App) {
        let state = Self::global(cx);
        state.update(cx, |state, cx| {
            // state.counter -= 1;
            cx.notify();
        });
    }

    pub fn reset_counter(cx: &mut App) {
        let state = Self::global(cx);
        state.update(cx, |state, cx| {
            // state.counter = 0;
            cx.notify();
        });
    }
}
