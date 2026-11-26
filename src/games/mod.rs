pub mod air_hockey;
pub mod beacon;
pub mod caro;
pub mod echo;
pub mod fuse_yard;
pub mod hill;
pub mod pong;
pub mod snake;
pub mod stones;
mod table;
pub mod tiles;
pub mod ur;
use macroquad::prelude::Color;
pub fn color(slot: usize) -> Color {
    Color::from_hex([0xadf56e, 0x79bdff, 0xff927e, 0xffdb74][slot])
}
use crate::core::game::{Config, MiniGame};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Era {
    Ancient,
    Traditional,
    Classic,
    Modern,
}
impl Era {
    pub const ALL: [Self; 4] = [
        Self::Ancient,
        Self::Traditional,
        Self::Classic,
        Self::Modern,
    ];
    pub fn label(self, en: bool) -> &'static str {
        match (self, en) {
            (Self::Ancient, false) => "Cổ đại",
            (Self::Traditional, false) => "Dân gian",
            (Self::Classic, false) => "Cổ điển",
            (Self::Modern, false) => "Hiện đại",
            (Self::Ancient, true) => "Ancient",
            (Self::Traditional, true) => "Traditional",
            (Self::Classic, true) => "Classic",
            (Self::Modern, true) => "Modern",
        }
    }
}
pub struct Entry {
    pub id: &'static str,
    pub title: &'static str,
    pub modes: &'static [usize],
    pub era: Era,
    pub difficulty: u8,
    pub art: &'static [u8],
    pub music: &'static [u8],
    pub rules: [&'static [&'static str]; 2],
    factory: fn() -> Box<dyn MiniGame>,
}
impl Entry {
    pub fn start(&self, players: usize, seed: u64) -> Box<dyn MiniGame> {
        assert!(self.modes.contains(&players));
        let mut g = (self.factory)();
        g.init(players, Config { seed });
        g
    }
}
pub const CATALOG: &[Entry] = &[
    Entry {
        id: "pong",
        title: pong::TITLE,
        modes: &[2],
        era: Era::Classic,
        difficulty: 1,
        art: include_bytes!("../../assets/art/pong.png"),
        music: include_bytes!("../../assets/audio/pong.wav"),
        rules: [
            &[
                "Lên / xuống để di chuyển vợt.",
                "Đánh bằng mép vợt để đổi góc bóng.",
                "Đưa bóng qua vạch cuối sân. Ai đạt 7 điểm trước sẽ thắng.",
            ],
            &[
                "Move your paddle up or down.",
                "Strike near the tip to change the angle.",
                "Pass the far baseline to score. First to 7 wins.",
            ],
        ],
        factory: || Box::new(pong::Pong::default()),
    },
    Entry {
        id: "snake",
        title: snake::TITLE,
        modes: &[2],
        era: Era::Classic,
        difficulty: 2,
        art: include_bytes!("../../assets/art/snake.png"),
        music: include_bytes!("../../assets/audio/snake.wav"),
        rules: [
            &[
                "Rẽ bằng bốn hướng. Rắn luôn tiến và không thể quay đầu ngay.",
                "Ăn mồi +2. Đụng tường hoặc thân rắn sẽ bị loại.",
                "Sống sót +5 và thắng. Cùng bị loại thì hòa. Hết 90 giây xét điểm.",
            ],
            &[
                "Steer in four directions. No instant reversal.",
                "Fruit gives 2 points. Walls or bodies eliminate you.",
                "Sole survivor wins (+5). Double crash draws. At 90s highest score wins.",
            ],
        ],
        factory: || Box::new(snake::Snake::default()),
    },
    Entry {
        id: "fuse-yard",
        title: fuse_yard::TITLE,
        modes: &[4],
        era: Era::Modern,
        difficulty: 2,
        art: include_bytes!("../../assets/art/fuse-yard.png"),
        music: include_bytes!("../../assets/audio/fuse-yard.wav"),
        rules: [
            &[
                "A đặt lõi. B đổi trục ngang / dọc cho lõi tiếp theo.",
                "Sau 1,6 giây, xung đi 3 ô mỗi phía, phá kiện (+1) và kích lõi khác.",
                "Người cuối cùng thắng (+5). Hết 100 giây xét điểm người còn sống.",
            ],
            &[
                "A drops a core. B switches its horizontal / vertical axis.",
                "After 1.6s the pulse travels 3 tiles, breaks crates (+1) and chains cores.",
                "Last survivor wins (+5). At 100s compare surviving scores.",
            ],
        ],
        factory: || Box::new(fuse_yard::FuseYard::default()),
    },
    Entry{ id:"ur",title:"Ur Sprint",modes:&[1, 2],era:Era::Ancient,difficulty:2,art:include_bytes!("../../assets/art/ur.png"),music:include_bytes!("../../assets/audio/ur.wav"),rules:[&["A gieo bốn hạt nhị phân. Dùng hướng để chọn quân hợp lệ, A để đi.","Chặng 5–12 dùng chung. Bắt quân đối phương, trừ ô hoa 8. Ô hoa cho thêm lượt.","Đưa đủ 3 quân về ô 15 bằng đúng số bước để thắng. 1P đấu máy."],&["A rolls four binary dice. Use directions to select a legal runner; A moves.","Spaces 5–12 are shared. Capture rivals except on rosette 8. Rosettes grant another turn.","Bring three runners exactly to 15. Solo plays against the CPU."]],factory:||Box::new(ur::Ur::default()) },
    Entry{ id:"caro",title:"Caro",modes:&[1, 2],era:Era::Classic,difficulty:2,art:include_bytes!("../../assets/art/caro.png"),music:include_bytes!("../../assets/audio/caro.wav"),rules:[&["Dùng bốn hướng để chọn giao điểm. A đặt quân.","Năm quân liên tiếp ngang, dọc hoặc chéo sẽ thắng. Bàn đầy thì hòa.","Chơi một mình đấu máy hoặc hai người thay phiên."],&["Move the cursor with directions. A places a stone.","Make five or more in a row horizontally, vertically or diagonally. A full board draws.","Play solo against the CPU or alternate between two seats."]],factory:||Box::new(caro::Caro::default()) },
    Entry{ id:"stones",title:"Bốc sỏi",modes:&[1, 2, 3, 4],era:Era::Traditional,difficulty:1,art:include_bytes!("../../assets/art/stones.png"),music:include_bytes!("../../assets/audio/stones.wav"),rules:[&["Khởi đầu với 21 viên sỏi. Mỗi lượt lấy 1, 2 hoặc 3 viên.","Trái/phải đổi số lượng, A xác nhận. Lượt chuyển theo thứ tự P1 → P2…","Người lấy viên cuối thắng. 1P thi với máy."],&["Start with 21 stones. Take 1, 2 or 3 on your turn.","Left/right changes the count; A confirms. Seats take turns in order.","Take the final stone to win. Solo plays against the CPU."]],factory:||Box::new(stones::Stones::default()) },
    Entry{ id:"tiles",title:"15 Tiles",modes:&[1],era:Era::Classic,difficulty:2,art:include_bytes!("../../assets/art/tiles.png"),music:include_bytes!("../../assets/audio/tiles.wav"),rules:[&["Di chuyển ô trống bằng bốn phím hướng.","Xếp các số từ 1 tới 15 theo hàng ngang. Ô trống ở góc dưới bên phải.","Mọi bàn đều xáo từ trạng thái đích. Ít lượt hơn được nhiều điểm hơn."],&["Move the empty square with the four directions.","Arrange 1 through 15 in reading order with the gap at bottom right.","Every shuffle is solvable. Fewer moves earn a higher score."]],factory:||Box::new(tiles::Tiles::default()) },
    Entry{ id:"echo",title:"Echo Steps",modes:&[1, 2, 3, 4],era:Era::Modern,difficulty:2,art:include_bytes!("../../assets/art/echo.png"),music:include_bytes!("../../assets/audio/echo.wav"),rules:[&["Xem các mũi tên sáng lần lượt, rồi bấm lại đúng thứ tự.","Mỗi bước đúng +1. Sai thì đến lượt tiếp theo.","Qua 5 cấp, ai nhớ đúng nhiều nhất thắng. Có thể chuyền máy."],&["Watch the arrows light up, then repeat the sequence.","Each correct direction scores 1. A mistake ends your turn.","Highest score over five levels wins. Pass the device between seats."]],factory:||Box::new(echo::Echo::default()) },
    Entry{ id:"beacon",title:"Beacon Rush",modes:&[1, 2, 3, 4],era:Era::Modern,difficulty:1,art:include_bytes!("../../assets/art/beacon.png"),music:include_bytes!("../../assets/audio/beacon.wav"),rules:[&["Mỗi người dùng nút A riêng. Chờ đèn chuyển từ đỏ sang xanh rồi bấm.","Bấm sớm hoặc quá 1,2 giây được 0 ở lượt đó.","Thi 5 lượt. Phản xạ nhanh được nhiều điểm hơn."],&["Each seat uses its own A. Wait for the light to turn green, then tap.","A false start or a reaction later than 1.2 seconds scores zero for that heat.","Five heats. Faster valid reactions earn more points."]],factory:||Box::new(beacon::Beacon::default()) },
    Entry{ id:"hill",title:"Neon Hill",modes:&[2, 3, 4],era:Era::Modern,difficulty:2,art:include_bytes!("../../assets/art/hill.png"),music:include_bytes!("../../assets/audio/hill.wav"),rules:[&["Dùng bốn hướng để chạy, A để tăng tốc ngắn.","Đứng một mình trong vòng được 1 điểm mỗi nửa giây. Có tranh chấp thì dừng tính.","Vòng đổi chỗ mỗi 6 giây. Sau 36 giây, người nhiều điểm nhất thắng."],&["Move in four directions; A gives a short sprint.","Occupy the circle alone for 1 point per half-second. Contested time earns nothing.","The hill moves every six seconds. Highest score at 36 seconds wins."]],factory:||Box::new(hill::Hill::default()) },
    Entry {
        id: "air-hockey",
        title: "Air Hockey",
        modes: &[2],
        era: Era::Classic,
        difficulty: 1,
        art: include_bytes!("../../assets/art/air-hockey.png"),
        music: include_bytes!("../../assets/audio/air-hockey.wav"),
        rules: [
            &["Di chuyển tay gạt bằng bốn hướng trong nửa sân của mình.",
              "Đón đĩa rồi đẩy về khe thành đối thủ. Tay gạt đang di chuyển đánh mạnh hơn.",
              "Ghi 5 bàn trước để thắng."],
            &["Move your striker in four directions inside your half.",
              "Strike the puck into the goal opening. A moving striker adds speed.",
              "First to five goals wins."],
        ],
        factory: || Box::new(air_hockey::AirHockey::default()),
    },
];
