use game_hub::core::clock::{Clock, STEP};
#[test]
fn a_stall_cannot_replay_seconds_of_input() {
    let mut clock = Clock::default();
    assert_eq!(clock.ticks(3.0), 6);
    assert_eq!(clock.ticks(STEP), 1);
}

#[test]
fn fractional_frames_accumulate_over_one_second() {
    let mut clock = Clock::default();
    let ticks: u32 = (0..120).map(|_| u32::from(clock.ticks(STEP / 2.0))).sum();
    assert_eq!(ticks, 60);
}
