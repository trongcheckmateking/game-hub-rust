use game_hub::core::clock::{Clock, STEP};
#[test]
fn a_stall_cannot_replay_seconds_of_input() {
    let mut clock = Clock::default();
    assert_eq!(clock.ticks(3.0), 6);
    assert_eq!(clock.ticks(STEP), 1);
}
