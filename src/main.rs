use livesplit_core::{Run, Segment, Timer, SharedTimer, HotkeySystem, HotkeyConfig};
use livesplit_hotkey::linux::KeyCode;
use livesplit_core::parking_lot::RwLock;
use std::sync::{Arc};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Starting Repro!");

    // Create a run object that we can use with at least one segment.
    let mut run = Run::new();
    run.set_game_name("Super Mario Odyssey");
    run.set_category_name("Any%");
    run.push_segment(Segment::new("Cap Kingdom"));
    run.push_segment(Segment::new("Bacon Kingdom"));
    run.push_segment(Segment::new("Taco Kingdom"));
    run.push_segment(Segment::new("Ham Kingdom"));
    run.push_segment(Segment::new("Pizza Kingdom"));

    let timer: SharedTimer = Arc::new(RwLock::new(Timer::new(run).expect("Run with at least one segment provided")));

    let hotkey = HotkeySystem::with_config(timer.clone(), HotkeyConfig {
      split: Some(KeyCode::AltR),
      reset: Some(KeyCode::BackSpace),
      undo: Some(KeyCode::Up),
      skip: Some(KeyCode::Down),
      pause: Some(KeyCode::Pause),
      undo_all_pauses: Some(KeyCode::SuperL),
      previous_comparison: Some(KeyCode::Left),
      next_comparison: Some(KeyCode::Right),
      toggle_timing_method: Some(KeyCode::Plus),
    }).expect("hotkeys could not register");
    // Start a new attempt.

    loop {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let in_ms = since_the_epoch.as_secs() * 1000 +
            since_the_epoch.subsec_nanos() as u64 / 1_000_000;

        println!("{:?} {:?} - Split {:?}",
            in_ms,
            timer.clone().read().current_phase(),
            timer.clone().read().current_split_index(),
        );
    }
}
