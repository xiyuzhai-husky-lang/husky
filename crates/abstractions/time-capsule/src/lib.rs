use crate::state::IsTimeCapsuleState;

pub mod action;
pub mod capsule;
pub mod event;
pub mod state;

#[test]
fn time_capsule_works() {
    use crate::{
        action::IsTimeCapsuleAction,
        event::{IsTimeCapsuleEvent, IsTimeCapsuleEventBuffer},
    };
    use expect_test::expect;

    #[derive(Default)]
    struct Counter(i32);

    impl std::fmt::Debug for Counter {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl IsTimeCapsuleState for Counter {
        type Event = CounterEvent;

        fn redo(&mut self, event: &Self::Event) {
            self.0 += event.0
        }

        fn undo(&mut self, event: &Self::Event) {
            self.0 -= event.0
        }
    }

    #[derive(Default)]
    struct CounterEvent(i32);

    impl std::fmt::Debug for CounterEvent {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl IsTimeCapsuleEvent for CounterEvent {
        type Buffer = CounterEventBuffer;

        type State = Counter;
    }

    #[derive(Default)]
    struct CounterEventBuffer(i32);

    impl IsTimeCapsuleEventBuffer for CounterEventBuffer {
        type Event = CounterEvent;

        fn finish(self) -> Option<Self::Event> {
            (self.0 != 0).then_some(CounterEvent(self.0))
        }
    }

    #[derive(Debug)]
    enum CounterAction {
        Incr,
        Decr,
    }

    impl IsTimeCapsuleAction for CounterAction {
        type Event = CounterEvent;

        type Outcome = ();

        fn add_to_event_buffer(&self, event: &mut CounterEventBuffer) {
            match self {
                CounterAction::Incr => event.0 += 1,
                CounterAction::Decr => event.0 -= 1,
            }
        }

        fn exec(self, state: &mut Counter) -> Self::Outcome {
            match self {
                CounterAction::Incr => state.0 += 1,
                CounterAction::Decr => state.0 -= 1,
            }
        }
    }

    let mut capsule: capsule::TimeCapsule<Counter> = Default::default();
    expect![[r#"
        TimeCapsule {
            state: 0,
            events: [],
            num_of_active_events: 0,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.undo();
    expect![[r#"
        TimeCapsule {
            state: 0,
            events: [],
            num_of_active_events: 0,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.redo();
    expect![[r#"
        TimeCapsule {
            state: 0,
            events: [],
            num_of_active_events: 0,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.add_event(|event_builder| {
        event_builder.add_action(CounterAction::Incr);
    });
    assert_eq!(capsule.state.0, 1);
    expect![[r#"
        TimeCapsule {
            state: 1,
            events: [
                1,
            ],
            num_of_active_events: 1,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.undo();
    assert_eq!(capsule.state.0, 0);
    expect![[r#"
        TimeCapsule {
            state: 0,
            events: [
                1,
            ],
            num_of_active_events: 0,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.redo();
    assert_eq!(capsule.state.0, 1);
    expect![[r#"
        TimeCapsule {
            state: 1,
            events: [
                1,
            ],
            num_of_active_events: 1,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.add_event(|event_builder| {
        event_builder.add_action(CounterAction::Decr);
    });
    assert_eq!(capsule.state.0, 0);
    expect![[r#"
        TimeCapsule {
            state: 0,
            events: [
                1,
                -1,
            ],
            num_of_active_events: 2,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.undo();
    assert_eq!(capsule.state.0, 1);
    expect![[r#"
        TimeCapsule {
            state: 1,
            events: [
                1,
                -1,
            ],
            num_of_active_events: 1,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.undo();
    assert_eq!(capsule.state.0, 0);
    expect![[r#"
        TimeCapsule {
            state: 0,
            events: [
                1,
                -1,
            ],
            num_of_active_events: 0,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.redo();
    assert_eq!(capsule.state.0, 1);
    expect![[r#"
        TimeCapsule {
            state: 1,
            events: [
                1,
                -1,
            ],
            num_of_active_events: 1,
        }
    "#]]
    .assert_debug_eq(&capsule);
    capsule.add_event(|event_builder| {
        event_builder.add_action(CounterAction::Incr);
    });
    assert_eq!(capsule.state.0, 2);
    expect![[r#"
        TimeCapsule {
            state: 2,
            events: [
                1,
                1,
            ],
            num_of_active_events: 2,
        }
    "#]]
    .assert_debug_eq(&capsule);
}
