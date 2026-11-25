use game_hub::core::{
    game::overlaps,
    input::{can_bind, names, Bindings},
    save::{record, Save, Store},
};
use game_hub::games::CATALOG;
#[test]
fn input_aliases_are_unique_for_each_active_party() {
    let keys = Bindings::new();
    for n in 2..=4 {
        let mut seen = std::collections::HashSet::new();
        for p in 0..n {
            for a in 0..6 {
                for k in names(p, a, n, &keys) {
                    assert!(seen.insert(k));
                }
            }
        }
    }
    assert!(names(1, 4, 2, &keys).contains(&"K".to_owned()));
    assert!(!names(1, 4, 4, &keys).contains(&"K".to_owned()));
    assert!(!can_bind(2, 0, "Up", &keys));
    assert!(!can_bind(0, 4, "Escape", &keys));
    assert!(can_bind(0, 4, "Z", &keys));
}
#[test]
fn collision_requires_positive_area() {
    assert!(overlaps((0., 0., 4., 4.), (3., 3., 2., 2.)));
    assert!(!overlaps((0., 0., 4., 4.), (4., 1., 2., 2.)));
}
#[test]
fn backup_round_trip_and_invalid_import_preserves_save() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Store {
        root: tmp.path().into(),
    };
    let mut s = Save::default();
    s.settings.english = true;
    s.settings.keys.insert("0.4".into(), "Z".into());
    record(&mut s.highscores, "snake", 2, &[4, 8]);
    record(&mut s.highscores, "snake", 2, &[1, 0]);
    store.replace(&s).unwrap();
    store.export(&s).unwrap();
    store.replace(&Save::default()).unwrap();
    assert_eq!(store.import().unwrap(), s);
    assert_eq!(store.load().unwrap(), s);
    assert_eq!(s.highscores["snake:2"], 8);
    std::fs::write(store.backup(), b"{").unwrap();
    assert!(store.import().is_err());
    assert_eq!(store.load().unwrap(), s);
}
#[test]
fn unfinished_save_is_recovered() {
    let tmp = tempfile::tempdir().unwrap();
    let store = Store {
        root: tmp.path().into(),
    };
    let mut s = Save::default();
    s.settings.muted = true;
    std::fs::write(
        store.root.join("pending.json"),
        serde_json::to_vec(&s).unwrap(),
    )
    .unwrap();
    assert_eq!(store.load().unwrap(), s);
    assert!(!store.root.join("pending.json").exists());
}
#[test]
fn registered_games_accept_their_lobby_and_freeze_during_pause() {
    use game_hub::core::input::PlayerFrame;
    for e in CATALOG {
        for &n in e.modes {
            let mut game = e.start(n, 1);
            let frames = vec![PlayerFrame::default(); n];
            game.handle_input(&frames);
            game.pause();
            for _ in 0..9000 {
                game.update(1. / 60.);
            }
            assert!(!game.is_game_over());
            assert_eq!(game.get_score(), vec![0; n]);
            game.resume();
            game.update(1. / 60.);
        }
    }
}
