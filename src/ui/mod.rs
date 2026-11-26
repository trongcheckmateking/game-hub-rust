mod draw;
mod hub;
mod settings;
use crate::{
    core::{
        audio::Audio,
        clock::{Clock, STEP},
        game::{MiniGame, ResultState},
        input::{names, Input},
        save::{record, Store},
        scene::Scene,
    },
    games::{color, CATALOG},
};
use draw::{bi, box_fill, camera, fitted, label, Nav, DIM, INK, PANEL, PAPER};
use hub::{Lobby, LobbyAction};
use macroquad::prelude::*;
use settings::Preferences;
fn seed() -> u64 {
    (macroquad::miniquad::date::now() * 1_000_000.).round() as u64
}
fn start(i: usize, n: usize) -> Box<dyn MiniGame> {
    CATALOG[i].start(n, seed())
}
pub async fn run() {
    let normal = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/DejaVuSans.ttf"))
        .expect("bundled font");
    let mono = load_ttf_font_from_bytes(include_bytes!("../../assets/fonts/DejaVuSansMono.ttf"))
        .expect("bundled mono");
    let art: Vec<_> = CATALOG
        .iter()
        .map(|g| Texture2D::from_file_with_format(g.art, Some(ImageFormat::Png)))
        .collect();
    let store = Store::local();
    let mut prefs = Preferences::default();
    let mut saves = store.load().unwrap_or_else(|e| {
        prefs.note = e.to_string();
        Default::default()
    });
    set_fullscreen(saves.settings.fullscreen);
    let mut audio = Audio::load(CATALOG.iter().map(|e| (e.id, e.music))).await;
    let mut devices = Input::default();
    let mut clock = Clock::default();
    let mut scene = Scene::Hub;
    let mut return_to = Scene::Hub;
    let mut idx = 0;
    let mut n = CATALOG[0].modes[0];
    let mut game = start(0, n);
    let mut nav = Nav::new();
    let mut lobby = Lobby::default();
    let mut transition = 0.0_f32;
    let mut notice = String::new();
    loop {
        #[cfg(target_arch = "wasm32")]
        crate::platform::web::scene(match scene {Scene::Hub=>0,Scene::Tutorial=>1,Scene::Playing=>2,Scene::Paused=>3,Scene::Settings=>4,Scene::End=>5},n,saves.settings.english);
        let before = scene;
        let en = saves.settings.english;
        let lang = |v, e| bi(en, v, e);
        let font = if saves.settings.retro { &mono } else { &normal };
        let captured = (scene == Scene::Settings && prefs.capturing())
            || (scene == Scene::Hub && lobby.editing);
        devices.poll(n, &saves.settings.keys);
        let key = |k| !captured && is_key_pressed(k);
        if key(KeyCode::M) {
            saves.settings.muted = !saves.settings.muted;
            if let Err(e) = store.settings(&saves.settings) {
                notice = e.to_string();
            }
        }
        if key(KeyCode::F11) {
            saves.settings.fullscreen = !saves.settings.fullscreen;
            set_fullscreen(saves.settings.fullscreen);
            if let Err(e) = store.settings(&saves.settings) {
                notice = e.to_string();
            }
        }
        nav.begin(
            !captured
                && (devices.menu[0] || devices.menu[2] || key(KeyCode::Up) || key(KeyCode::Left)),
            !captured
                && (devices.menu[1]
                    || devices.menu[3]
                    || key(KeyCode::Down)
                    || key(KeyCode::Right)
                    || key(KeyCode::Tab)),
            !captured && (devices.menu[4] || key(KeyCode::Enter)),
        );
        let exit = key(KeyCode::Backspace);
        let esc = key(KeyCode::Escape) || devices.menu[5];
        clear_background(INK);
        camera(Vec2::ZERO);
        box_fill(Rect::new(0., 0., 1120., 65.), PANEL);
        draw_rectangle(44., 28., 25., 7., draw::LIME);
        draw_rectangle(53., 19., 7., 25., draw::LIME);
        label(&mono, "GAME HUB", 85., 44., 29, PAPER);
        label(
            font,
            &format!(
                "{}  ·  {} {}",
                lang("Cùng một máy", "Local offline"),
                devices.count(),
                lang("tay cầm", "gamepads")
            ),
            665.,
            42.,
            19,
            DIM,
        );
        match scene {
            Scene::Hub => match lobby.show(font, &mut nav, &art, &saves.highscores, en) {
                LobbyAction::Choose(i, players) => {
                    idx = i;
                    n = players;
                    scene = Scene::Tutorial;
                }
                LobbyAction::Settings => {
                    return_to = Scene::Hub;
                    scene = Scene::Settings;
                }
                LobbyAction::Quit => break,
                LobbyAction::None => {}
            },
            Scene::Tutorial => {
                let e = &CATALOG[idx];
                if e.modes.len() > 1 {
                    for (j, &mode) in e.modes.iter().enumerate() {
                        if nav.button(
                            font,
                            Rect::new(721. + j as f32 * 74., 108., 67., 40.),
                            &format!("{mode}P"),
                            n == mode,
                        ) {
                            n = mode;
                        }
                    }
                }
                label(font, e.title, 46., 139., 46, PAPER);
                label(
                    font,
                    &format!("{}P  /  {}", n, lang("Hướng dẫn", "How to play")),
                    49.,
                    179.,
                    23,
                    draw::LIME,
                );
                for (i, line) in e.rules[usize::from(en)].iter().enumerate() {
                    fitted(
                        font,
                        line,
                        Rect::new(39., 200. + 44. * i as f32, 1040., 38.),
                        23,
                        PAPER,
                    );
                }
                label(
                    font,
                    lang("Phím theo vị trí", "Controls by seat"),
                    48.,
                    376.,
                    28,
                    PAPER,
                );
                for p in 0..n {
                    let map: Vec<_> = (0..6)
                        .map(|a| names(p, a, n, &saves.settings.keys).join("/"))
                        .collect();
                    let k = format!(
                        "P{}   {}  {}  {}  {}       A  {}      B  {}",
                        p + 1,
                        map[0],
                        map[1],
                        map[2],
                        map[3],
                        map[4],
                        map[5]
                    );
                    fitted(
                        font,
                        &k,
                        Rect::new(40., 388. + p as f32 * 35., 1030., 32.),
                        19,
                        color(p),
                    );
                }
                label(
                    font,
                    lang(
                        "Tay cầm  ·  D-pad / cần trái   ·   A/× và B/○",
                        "Gamepad  ·  D-pad / left stick   ·   A/× and B/○",
                    ),
                    49.,
                    564.,
                    18,
                    DIM,
                );
                if nav.button(
                    font,
                    Rect::new(47., 598., 245., 53.),
                    lang("Vào trận →", "Start round →"),
                    true,
                ) {
                    game = start(idx, n);
                    scene = Scene::Playing;
                }
                if nav.button(
                    font,
                    Rect::new(309., 598., 170., 53.),
                    lang("Quay lại", "Back"),
                    false,
                ) || esc
                    || exit
                {
                    scene = Scene::Hub;
                }
            }
            Scene::Playing => {
                if exit {
                    scene = Scene::Hub;
                } else if key(KeyCode::Escape) || devices.start {
                    game.pause();
                    scene = Scene::Paused;
                } else if key(KeyCode::R) {
                    game = start(idx, n);
                    clock = Clock::default();
                    devices.clear();
                }
                if scene == Scene::Playing {
                    for _ in 0..clock.ticks(get_frame_time()) {
                        game.handle_input(&devices.take()[..n]);
                        game.update(STEP);
                        for fx in std::mem::take(&mut game.round_mut().cues) {
                            audio.cue(fx, &saves.settings);
                        }
                        if game.is_game_over() {
                            record(&mut saves.highscores, CATALOG[idx].id, n, game.get_score());
                            if let Err(e) = store.scores(&saves.highscores) {
                                notice = e.to_string();
                            }
                            audio.effect(
                                if matches!(game.round().result,ResultState::Draw|ResultState::Defeat) {
                                    "lose"
                                } else {
                                    "win"
                                },
                                &saves.settings,
                            );
                            scene = Scene::End;
                            break;
                        }
                    }
                }
                camera(vec2(80., 107.));
                game.render();
                camera(Vec2::ZERO);
                label(font, CATALOG[idx].title, 81., 95., 24, PAPER);
                for (p, s) in game.get_score().iter().enumerate() {
                    label(
                        font,
                        &format!("P{} · {}", p + 1, s),
                        448. + p as f32 * 134.,
                        94.,
                        19,
                        color(p),
                    );
                }
                label(
                    font,
                    lang(
                        "Esc tạm dừng     R chơi lại     Backspace về hub",
                        "Esc pause     R restart     Backspace return to hub",
                    ),
                    80.,
                    683.,
                    17,
                    DIM,
                );
            }
            Scene::Paused | Scene::End => {
                let pause = scene == Scene::Paused;
                camera(vec2(80., 107.));
                game.render();
                camera(Vec2::ZERO);
                draw_rectangle(0., 65., 1120., 655., Color::new(0.025, 0.035, 0.03, 0.83));
                box_fill(Rect::new(278., 135., 564., 455.), PANEL);
                let title = if pause {
                    lang("Tạm dừng", "Paused").to_owned()
                } else {
                    match game.round().result {
                        ResultState::Winner(i) => {
                            format!("P{} {}", i + 1, lang("chiến thắng!", "wins!"))
                        }
                        ResultState::Defeat => lang("Thử lại nhé!", "Try again!").into(),
                        _ => lang("Hòa!", "Draw!").into(),
                    }
                };
                fitted(font, &title, Rect::new(303., 160., 500., 60.), 42, PAPER);
                let scores = game
                    .get_score()
                    .iter()
                    .enumerate()
                    .map(|(p, s)| format!("P{}  {}", p + 1, s))
                    .collect::<Vec<_>>()
                    .join("     ");
                fitted(
                    font,
                    &scores,
                    Rect::new(306., 227., 485., 40.),
                    22,
                    draw::LIME,
                );
                let mut y = 292.;
                if pause {
                    if nav.button(
                        font,
                        Rect::new(308., y, 500., 46.),
                        lang("Tiếp tục", "Resume"),
                        true,
                    ) || esc
                        || devices.start
                    {
                        game.resume();
                        scene = Scene::Playing;
                    }
                    y += 61.;
                }
                if nav.button(
                    font,
                    Rect::new(308., y, 500., 46.),
                    lang("Chơi lại", "Play again"),
                    !pause,
                ) || key(KeyCode::R)
                {
                    game = start(idx, n);
                    scene = Scene::Playing;
                }
                y += 61.;
                if pause {
                    if nav.button(
                        font,
                        Rect::new(308., y, 500., 46.),
                        lang("Cài đặt", "Settings"),
                        false,
                    ) {
                        return_to = Scene::Paused;
                        scene = Scene::Settings;
                    }
                    y += 61.;
                }
                if nav.button(
                    font,
                    Rect::new(308., y, 500., 46.),
                    lang("Về hub", "Quit to hub"),
                    false,
                ) || exit
                    || (!pause && esc)
                {
                    scene = Scene::Hub;
                }
            }
            Scene::Settings => {
                if prefs.show(font, &mut nav, &mut saves, &store)
                    || (!captured && (devices.menu[5] || exit))
                {
                    scene = return_to;
                }
            }
        }
        nav.finish();
        if !notice.is_empty() {
            fitted(font, &notice, Rect::new(38., 684., 1044., 32.), 14, ORANGE);
        }
        if before != scene {
            transition = 0.14;
            nav.reset();
            devices.clear();
            clock = Clock::default();
            if before != Scene::Playing || scene != Scene::End {
                audio.effect("click", &saves.settings);
            }
        }
        let in_match = matches!(scene, Scene::Playing | Scene::Paused | Scene::End)
            || (scene == Scene::Settings && return_to == Scene::Paused);
        audio.music(
            if in_match { CATALOG[idx].id } else { "hub" },
            &saves.settings,
        );
        if transition > 0. {
            draw_rectangle(
                0.,
                0.,
                1120.,
                720.,
                Color::new(0.04, 0.06, 0.05, (transition / 0.14).min(1.) * 0.65),
            );
            transition -= get_frame_time();
        }
        if scene != Scene::Playing {
            devices.clear();
        }
        next_frame().await;
    }
}
