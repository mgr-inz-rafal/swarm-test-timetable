extern crate piston_window;
#[macro_use(make_slot_pit, make_slot_spawner)]
extern crate swarm_it;
extern crate chrono;
extern crate rand;
extern crate time;

use chrono::prelude::*;
use piston_window::*;
use rand::Rng;
use std::char;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};
use std::ops::Add;
use swarm_it::{Carrier, Payload, Slot, SlotKind};
use time::Duration;

const SCREEN_SIZE_NATIVE: [u32; 2] = [1920, 1080];
const TILE_WIDTH: u32 = 44;
const TILE_HEIGHT: u32 = 54;
const TILE_SPACING: u32 = (50 - TILE_WIDTH) / 2;
const TILES_PER_ROW: u32 = 36;
const TILES_PER_COLUMN: u32 = 15;
const BOARD_LEFT_MARGIN: u32 =
    (SCREEN_SIZE_NATIVE[0] - (TILE_WIDTH + TILE_SPACING) * TILES_PER_ROW) / 2;
const BOARD_TOP_MARGIN: u32 =
    (SCREEN_SIZE_NATIVE[1] - (TILE_HEIGHT + TILE_SPACING) * TILES_PER_COLUMN) / 2;
const TILE_DELIMITER: char = '^';
const EMPTY_PAYLOAD: char = ' ';
const CARRIER_ANIM_SPEED: u32 = 8;
const CARRIER_ICON_X_OFFSET: f64 = 0.0;
const CARRIER_ICON_Y_OFFSET: f64 = -50.0;
const MAX_STATION_NAME_LENGTH: usize = 31;
const TIME_DIFFERENCE_MINIMUM: i64 = 13; // Minutes
const TIME_DIFFERENCE_MAXMIMUM: i64 = 90; // Minutes
const MAX_CARRIERS: u8 = 100;
const CARRIER_ACCELERATION: f64 = 0.16;
const CARRIER_MAX_SPEED: f64 = 24.0;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum TextureId {
    Background,
    Carrier01,
    Carrier02,
    Carrier03,
    Carrier04,
    Carrier05,
    Carrier06,
    Carrier07,
    Carrier08,
    Test,
    TileBlank,
    TileQ,
    TileW,
    TileE,
    TileR,
    TileT,
    TileY,
    TileU,
    TileI,
    TileO,
    TileP,
    TileA,
    TileS,
    TileD,
    TileF,
    TileG,
    TileH,
    TileJ,
    TileK,
    TileL,
    TileZ,
    TileX,
    TileC,
    TileV,
    TileB,
    TileN,
    TileM,
    TileEpl,
    TileOpl,
    TileApl,
    TileSpl,
    TileLpl,
    TileZpl,
    TileXpl,
    TileCpl,
    TileNpl,
    Tile0,
    Tile1,
    Tile2,
    Tile3,
    Tile4,
    Tile5,
    Tile6,
    Tile7,
    Tile8,
    Tile9,
    TileColon,
    TileOpenBracket,
    TileCloseBracket,
    TileDash,
    TileDot,
    TileAmp,
    TileApostrophe,
}

impl TextureId {
    fn from_char(c: char) -> TextureId {
        match c {
            'Q' | 'q' => TextureId::TileQ,
            'W' | 'w' => TextureId::TileW,
            'E' | 'e' => TextureId::TileE,
            'R' | 'r' => TextureId::TileR,
            'T' | 't' => TextureId::TileT,
            'Y' | 'y' => TextureId::TileY,
            'U' | 'u' => TextureId::TileU,
            'I' | 'i' => TextureId::TileI,
            'O' | 'o' => TextureId::TileO,
            'P' | 'p' => TextureId::TileP,
            'A' | 'a' => TextureId::TileA,
            'S' | 's' => TextureId::TileS,
            'D' | 'd' => TextureId::TileD,
            'F' | 'f' => TextureId::TileF,
            'G' | 'g' => TextureId::TileG,
            'H' | 'h' => TextureId::TileH,
            'J' | 'j' => TextureId::TileJ,
            'K' | 'k' => TextureId::TileK,
            'L' | 'l' => TextureId::TileL,
            'Z' | 'z' => TextureId::TileZ,
            'X' | 'x' => TextureId::TileX,
            'C' | 'c' => TextureId::TileC,
            'V' | 'v' => TextureId::TileV,
            'B' | 'b' => TextureId::TileB,
            'N' | 'n' => TextureId::TileN,
            'M' | 'm' => TextureId::TileM,
            'Ę' | 'ę' => TextureId::TileEpl,
            'Ó' | 'ó' => TextureId::TileOpl,
            'Ą' | 'ą' => TextureId::TileApl,
            'Ś' | 'ś' => TextureId::TileSpl,
            'Ł' | 'ł' => TextureId::TileLpl,
            'Ż' | 'ż' => TextureId::TileZpl,
            'Ź' | 'ź' => TextureId::TileXpl,
            'Ć' | 'ć' => TextureId::TileCpl,
            'Ń' | 'ń' => TextureId::TileNpl,
            '0' => TextureId::Tile0,
            '1' => TextureId::Tile1,
            '2' => TextureId::Tile2,
            '3' => TextureId::Tile3,
            '4' => TextureId::Tile4,
            '5' => TextureId::Tile5,
            '6' => TextureId::Tile6,
            '7' => TextureId::Tile7,
            '8' => TextureId::Tile8,
            '9' => TextureId::Tile9,
            ':' => TextureId::TileColon,
            '(' => TextureId::TileOpenBracket,
            ')' => TextureId::TileCloseBracket,
            '-' => TextureId::TileDash,
            '.' => TextureId::TileDot,
            '&' => TextureId::TileAmp,
            '\'' => TextureId::TileApostrophe,
            _ => TextureId::TileBlank,
        }
    }
}

struct TextureDef {
    id: TextureId,
    path: &'static str,
}

type MyGameType = swarm_it::Swarm<TextureId>;

const TEXTURE_REPOSITORY: [TextureDef; 63] = [
    TextureDef {
        id: TextureId::Test,
        path: "images/test_image.png",
    },
    TextureDef {
        id: TextureId::Carrier01,
        path: "images/carrier/frame-1.png",
    },
    TextureDef {
        id: TextureId::Carrier02,
        path: "images/carrier/frame-2.png",
    },
    TextureDef {
        id: TextureId::Carrier03,
        path: "images/carrier/frame-3.png",
    },
    TextureDef {
        id: TextureId::Carrier04,
        path: "images/carrier/frame-4.png",
    },
    TextureDef {
        id: TextureId::Carrier05,
        path: "images/carrier/frame-5.png",
    },
    TextureDef {
        id: TextureId::Carrier06,
        path: "images/carrier/frame-6.png",
    },
    TextureDef {
        id: TextureId::Carrier07,
        path: "images/carrier/frame-7.png",
    },
    TextureDef {
        id: TextureId::Carrier08,
        path: "images/carrier/frame-8.png",
    },
    TextureDef {
        id: TextureId::Background,
        path: "images/backgrounds/darlington.jpg",
    },
    TextureDef {
        id: TextureId::TileBlank,
        path: "images/tiles/tile_blank.png",
    },
    TextureDef {
        id: TextureId::TileQ,
        path: "images/tiles/tile_Q.png",
    },
    TextureDef {
        id: TextureId::TileW,
        path: "images/tiles/tile_W.png",
    },
    TextureDef {
        id: TextureId::TileE,
        path: "images/tiles/tile_E.png",
    },
    TextureDef {
        id: TextureId::TileR,
        path: "images/tiles/tile_R.png",
    },
    TextureDef {
        id: TextureId::TileT,
        path: "images/tiles/tile_T.png",
    },
    TextureDef {
        id: TextureId::TileY,
        path: "images/tiles/tile_Y.png",
    },
    TextureDef {
        id: TextureId::TileU,
        path: "images/tiles/tile_U.png",
    },
    TextureDef {
        id: TextureId::TileI,
        path: "images/tiles/tile_I.png",
    },
    TextureDef {
        id: TextureId::TileO,
        path: "images/tiles/tile_O.png",
    },
    TextureDef {
        id: TextureId::TileP,
        path: "images/tiles/tile_P.png",
    },
    TextureDef {
        id: TextureId::TileA,
        path: "images/tiles/tile_A.png",
    },
    TextureDef {
        id: TextureId::TileS,
        path: "images/tiles/tile_S.png",
    },
    TextureDef {
        id: TextureId::TileD,
        path: "images/tiles/tile_D.png",
    },
    TextureDef {
        id: TextureId::TileF,
        path: "images/tiles/tile_F.png",
    },
    TextureDef {
        id: TextureId::TileG,
        path: "images/tiles/tile_G.png",
    },
    TextureDef {
        id: TextureId::TileH,
        path: "images/tiles/tile_H.png",
    },
    TextureDef {
        id: TextureId::TileJ,
        path: "images/tiles/tile_J.png",
    },
    TextureDef {
        id: TextureId::TileK,
        path: "images/tiles/tile_K.png",
    },
    TextureDef {
        id: TextureId::TileL,
        path: "images/tiles/tile_L.png",
    },
    TextureDef {
        id: TextureId::TileZ,
        path: "images/tiles/tile_Z.png",
    },
    TextureDef {
        id: TextureId::TileX,
        path: "images/tiles/tile_X.png",
    },
    TextureDef {
        id: TextureId::TileC,
        path: "images/tiles/tile_C.png",
    },
    TextureDef {
        id: TextureId::TileV,
        path: "images/tiles/tile_V.png",
    },
    TextureDef {
        id: TextureId::TileB,
        path: "images/tiles/tile_B.png",
    },
    TextureDef {
        id: TextureId::TileN,
        path: "images/tiles/tile_N.png",
    },
    TextureDef {
        id: TextureId::TileM,
        path: "images/tiles/tile_M.png",
    },
    TextureDef {
        id: TextureId::TileEpl,
        path: "images/tiles/tile_Epl.png",
    },
    TextureDef {
        id: TextureId::TileOpl,
        path: "images/tiles/tile_Opl.png",
    },
    TextureDef {
        id: TextureId::TileApl,
        path: "images/tiles/tile_Apl.png",
    },
    TextureDef {
        id: TextureId::TileSpl,
        path: "images/tiles/tile_Spl.png",
    },
    TextureDef {
        id: TextureId::TileLpl,
        path: "images/tiles/tile_Lpl.png",
    },
    TextureDef {
        id: TextureId::TileZpl,
        path: "images/tiles/tile_Zpl.png",
    },
    TextureDef {
        id: TextureId::TileXpl,
        path: "images/tiles/tile_Xpl.png",
    },
    TextureDef {
        id: TextureId::TileCpl,
        path: "images/tiles/tile_Cpl.png",
    },
    TextureDef {
        id: TextureId::TileNpl,
        path: "images/tiles/tile_Npl.png",
    },
    TextureDef {
        id: TextureId::TileColon,
        path: "images/tiles/tile_colon.png",
    },
    TextureDef {
        id: TextureId::TileOpenBracket,
        path: "images/tiles/tile_openbracket.png",
    },
    TextureDef {
        id: TextureId::TileCloseBracket,
        path: "images/tiles/tile_closebracket.png",
    },
    TextureDef {
        id: TextureId::TileDash,
        path: "images/tiles/tile_dash.png",
    },
    TextureDef {
        id: TextureId::TileDot,
        path: "images/tiles/tile_dot.png",
    },
    TextureDef {
        id: TextureId::TileAmp,
        path: "images/tiles/tile_amp.png",
    },
    TextureDef {
        id: TextureId::TileApostrophe,
        path: "images/tiles/tile_apostrophe.png",
    },
    TextureDef {
        id: TextureId::Tile0,
        path: "images/tiles/tile_0.png",
    },
    TextureDef {
        id: TextureId::Tile1,
        path: "images/tiles/tile_1.png",
    },
    TextureDef {
        id: TextureId::Tile2,
        path: "images/tiles/tile_2.png",
    },
    TextureDef {
        id: TextureId::Tile3,
        path: "images/tiles/tile_3.png",
    },
    TextureDef {
        id: TextureId::Tile4,
        path: "images/tiles/tile_4.png",
    },
    TextureDef {
        id: TextureId::Tile5,
        path: "images/tiles/tile_5.png",
    },
    TextureDef {
        id: TextureId::Tile6,
        path: "images/tiles/tile_6.png",
    },
    TextureDef {
        id: TextureId::Tile7,
        path: "images/tiles/tile_7.png",
    },
    TextureDef {
        id: TextureId::Tile8,
        path: "images/tiles/tile_8.png",
    },
    TextureDef {
        id: TextureId::Tile9,
        path: "images/tiles/tile_9.png",
    },
];

fn load_textures(depot: &mut HashMap<TextureId, G2dTexture>, context: &mut G2dTextureContext) {
    TEXTURE_REPOSITORY.iter().for_each(|x| {
        println!("{:?}", x.path);
        depot.insert(
            x.id,
            Texture::from_path(context, x.path, Flip::None, &TextureSettings::new()).unwrap(),
        );
    });
}

fn is_tile_delimiter(c: char) -> bool {
    c == TILE_DELIMITER
}

fn is_empty_payload(c: char) -> bool {
    c == EMPTY_PAYLOAD
}

fn char_to_payload(c: char) -> Option<swarm_it::Payload<TextureId>> {
    if is_empty_payload(c) {
        None
    } else {
        Some(Payload::new(TextureId::from_char(c)))
    }
}

fn load_layout(game: &mut MyGameType, id: u32) -> Result<()> {
    let file = format!("layouts/layout{}.txt", id);
    println!("Loading layout from '{}'", file);
    let file = File::open(file)?;
    let mut buffer = BufReader::new(file);
    buffer
        .by_ref()
        .lines()
        .filter(|l| match l {
            Ok(line) => !(line.is_empty() || line.starts_with('#')),
            Err(_) => panic!("How come?"),
        })
        .enumerate()
        .for_each(|(y, line)| {
            let mut setting_source_cargo = true;
            let mut payload_being_set = None;
            line.unwrap()
                .chars()
                .filter(|c| !is_tile_delimiter(*c))
                .enumerate()
                .for_each(|(x, c)| {
                    if setting_source_cargo {
                        payload_being_set = char_to_payload(c);
                        setting_source_cargo = false;
                    } else {
                        game.add_slot(Slot::new(
                            f64::from(
                                BOARD_LEFT_MARGIN + (TILE_WIDTH + TILE_SPACING) * (x / 2) as u32,
                            ),
                            f64::from(BOARD_TOP_MARGIN + (TILE_HEIGHT + TILE_SPACING) * y as u32),
                            payload_being_set,
                            char_to_payload(c),
                            swarm_it::SlotKind::CLASSIC,
                        ));
                        setting_source_cargo = true;
                    };
                })
        });

    Ok(())
}

fn row_start_index(row: u32) -> u32 {
    row * TILES_PER_ROW
}

fn row_end_index(row: u32) -> u32 {
    row_start_index(row) + TILES_PER_ROW
}

fn slot_index(x: u32, y: u32) -> usize {
    (y * TILES_PER_ROW + x) as usize
}

fn fill_row_with_text(game: &mut MyGameType, row: u32, text: &str, target_only: bool) {
    let slots = game.get_slots_mut();
    let start_index = row_start_index(row);
    let end_index = row_end_index(row);
    let mut last_name_index = 0;
    text.chars()
        .enumerate()
        .take(MAX_STATION_NAME_LENGTH)
        .for_each(|(i, v)| {
            if target_only {
                slots[start_index as usize + i].set_target_payload(char_to_payload(v));
            } else {
                slots[start_index as usize + i].set_payloads(char_to_payload(v));
            }
            last_name_index = i;
        });
    for slot in slots
        .iter_mut()
        .take(end_index as usize)
        .skip(start_index as usize + last_name_index + 1)
    {
        if target_only {
            slot.set_target_payload(char_to_payload(EMPTY_PAYLOAD));
        } else {
            slot.set_payloads(char_to_payload(EMPTY_PAYLOAD));
        }
    }
}

fn fill_with_station_names(game: &mut MyGameType, stations: &[String]) {
    for i in 0..TILES_PER_COLUMN {
        fill_row_with_text(game, i, get_random_station_name(stations), false);
    }
}

fn fill_row_departure_time(
    game: &mut MyGameType,
    row: u32,
    time: DateTime<Utc>,
    target_only: bool,
) {
    let slots = game.get_slots_mut();
    let end_index = row_end_index(row);

    if target_only {
        slots[(end_index - 2) as usize].set_target_payload(char_to_payload(
            char::from_digit(time.minute() / 10, 10).unwrap(),
        ));
        slots[(end_index - 1) as usize].set_target_payload(char_to_payload(
            char::from_digit(time.minute() % 10, 10).unwrap(),
        ));

        slots[(end_index - 5) as usize].set_target_payload(char_to_payload(
            char::from_digit(time.hour() / 10, 10).unwrap(),
        ));
        slots[(end_index - 4) as usize].set_target_payload(char_to_payload(
            char::from_digit(time.hour() % 10, 10).unwrap(),
        ));
    } else {
        slots[(end_index - 2) as usize].set_payloads(char_to_payload(
            char::from_digit(time.minute() / 10, 10).unwrap(),
        ));
        slots[(end_index - 1) as usize].set_payloads(char_to_payload(
            char::from_digit(time.minute() % 10, 10).unwrap(),
        ));

        slots[(end_index - 5) as usize].set_payloads(char_to_payload(
            char::from_digit(time.hour() / 10, 10).unwrap(),
        ));
        slots[(end_index - 4) as usize].set_payloads(char_to_payload(
            char::from_digit(time.hour() % 10, 10).unwrap(),
        ));
    }
}

fn increase_departure_time(time: DateTime<Utc>) -> DateTime<Utc> {
    let mut rng = rand::thread_rng();
    time.add(Duration::minutes(
        rng.gen_range(TIME_DIFFERENCE_MINIMUM, TIME_DIFFERENCE_MAXMIMUM),
    ))
}

fn fill_departure_times(game: &mut MyGameType) -> DateTime<Utc> {
    let mut departure_time = Utc::now();
    for i in 0..TILES_PER_COLUMN {
        fill_row_departure_time(game, i, departure_time, false);
        departure_time = increase_departure_time(departure_time);
    }
    departure_time
}

fn fill_time_commas(game: &mut MyGameType) {
    let slots = game.get_slots_mut();
    for i in 0..TILES_PER_COLUMN {
        slots[(row_end_index(i) - 3) as usize].set_payloads(char_to_payload(':'));
    }
}

fn move_all_rows_up(slots: &mut Vec<swarm_it::Slot<TextureId>>) {
    for y in 0..TILES_PER_COLUMN - 1 {
        for x in 0..TILES_PER_ROW {
            let payloads = slots[slot_index(x, y + 1)].get_payloads();
            slots[slot_index(x, y)].set_target_payload(payloads[0]);
        }
    }
}

fn put_next_train_in_last_row(game: &mut MyGameType, time: DateTime<Utc>, new_station: &str) {
    fill_row_with_text(game, TILES_PER_COLUMN - 1, new_station, true);
    fill_row_departure_time(game, TILES_PER_COLUMN - 1, time, true);

    // Take special care about the HH:MM separator
    game.get_slots_mut()[slot_index(TILES_PER_ROW - 3, TILES_PER_COLUMN - 1)]
        .set_payloads(char_to_payload(':'));
}

fn train_departure(
    game: &mut MyGameType,
    last_time: DateTime<Utc>,
    new_station: &str,
) -> DateTime<Utc> {
    let next_time = increase_departure_time(last_time);
    move_all_rows_up(game.get_slots_mut());
    put_next_train_in_last_row(game, next_time, new_station);
    game.slot_data_changed();
    next_time
}

fn load_station_names(list: &mut Vec<String>, file: &str) -> Result<()> {
    println!("Loading stations from '{}'", file);
    let file = File::open(file)?;
    let buffer = BufReader::new(file);
    buffer.lines().for_each(|line| list.push(line.unwrap()));

    println!("{} station(s) loaded", list.len());
    Ok(())
}

fn get_random_station_name(station_names: &[String]) -> &str {
    let mut rng = rand::thread_rng();
    &station_names[rng.gen_range(0, station_names.len())]
}

fn is_facing_left(carrier: &swarm_it::Carrier<TextureId>) -> bool {
    let angle = carrier.get_angle();
    (angle > (std::f64::consts::PI / 2.0)) && (angle < ((3.0 / 2.0) * std::f64::consts::PI))
}

fn add_one_carrier(game: &mut MyGameType, x: f64, y: f64) {
    let index = game.add_carrier(Carrier::new(x, y));
    let carrier = &mut game.get_carriers_mut()[index];
    carrier.set_acceleration(CARRIER_ACCELERATION);
    carrier.set_max_speed(CARRIER_MAX_SPEED);
}

fn main() -> Result<()> {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Timetable: Demo of the Swarm library by mgr. inż. Rafał",
        SCREEN_SIZE_NATIVE,
    )
    .exit_on_esc(true)
    .fullscreen(true)
    .opengl(opengl)
    .build()
    .unwrap();

    let mut allow_next_departure = false;
    let mut game = swarm_it::Swarm::<TextureId>::new();

    let carrier_frames: [TextureId; 8] = [
        TextureId::Carrier01,
        TextureId::Carrier02,
        TextureId::Carrier03,
        TextureId::Carrier04,
        TextureId::Carrier05,
        TextureId::Carrier06,
        TextureId::Carrier07,
        TextureId::Carrier08,
    ];
    let mut carrier_anim_cycle = carrier_frames.iter().cycle();
    let mut carrier_anim_counter = 0;
    let mut carrier_anim_texture = carrier_anim_cycle.next().unwrap();
    let mut station_names = Vec::new();

    let mut ctx = window.create_texture_context();
    let mut texture_depot = HashMap::new();
    load_textures(&mut texture_depot, &mut ctx);
    load_station_names(&mut station_names, "stations/uk.txt")?;
    load_layout(&mut game, 2)?;
    fill_with_station_names(&mut game, &station_names);
    fill_time_commas(&mut game);
    let mut last_time = fill_departure_times(&mut game);
    game.slot_data_changed();
    add_one_carrier(
        &mut game,
        f64::from(SCREEN_SIZE_NATIVE[0] / 2),
        f64::from(SCREEN_SIZE_NATIVE[1] / 2),
    );
    let mut current_carriers_count = game.get_carriers().len() as u8;
    let mut draw_carriers = true;

    game.add_slot(make_slot_pit!(-75.0, f64::from(SCREEN_SIZE_NATIVE[1] / 2)));
    game.add_slot(make_slot_pit!(
        f64::from(SCREEN_SIZE_NATIVE[0] + 75),
        f64::from(SCREEN_SIZE_NATIVE[1] / 2)
    ));
    game.add_slot(make_slot_spawner!(
        -75.0,
        f64::from(SCREEN_SIZE_NATIVE[1] / 2)
    ));
    game.add_slot(make_slot_spawner!(
        f64::from(SCREEN_SIZE_NATIVE[0] + 75),
        f64::from(SCREEN_SIZE_NATIVE[1] / 2)
    ));

    window.set_ups(60);

    while let Some(e) = window.next() {
        e.update(|_| {
            if game.tick() {
                allow_next_departure = true
            }
        });

        e.release(|args| {
            if let piston_window::Button::Keyboard(k) = args {
                match k {
                    piston_window::Key::Space => {
                        if allow_next_departure {
                            last_time = train_departure(
                                &mut game,
                                last_time,
                                get_random_station_name(&station_names),
                            );
                            allow_next_departure = false
                        }
                    }
                    piston_window::Key::H => draw_carriers = !draw_carriers,
                    piston_window::Key::Plus | piston_window::Key::NumPadPlus => {
                        if current_carriers_count < MAX_CARRIERS {
                            current_carriers_count += 1;
                            add_one_carrier(&mut game, f64::from(SCREEN_SIZE_NATIVE[0] / 2), -75.0);
                        }
                    }
                    _ => {}
                }
            }
        });

        e.render(|_| {
            window.draw_2d(&e, |ctx, g, _| {
                // Clear
                clear([0.0; 4], g);

                // Paint background
                image(
                    texture_depot.get(&TextureId::Background).unwrap(),
                    ctx.transform,
                    g,
                );

                // Paint slots
                game.get_slots().iter().for_each(|&s| {
                    let pos = s.get_position();
                    let context = ctx.trans(pos.x, pos.y);

                    let texture;
                    if let Some(p) = s.get_payloads()[0] {
                        texture = texture_depot.get(&p.cargo);
                    } else {
                        texture = texture_depot.get(&TextureId::TileBlank);
                    }

                    Image::new_color([1.0, 1.0, 1.0, 0.85]).draw(
                        texture.unwrap(),
                        &ctx.draw_state,
                        context.transform,
                        g,
                    );
                });

                // Paint carriers
                carrier_anim_counter += 1;
                if carrier_anim_counter == CARRIER_ANIM_SPEED {
                    carrier_anim_counter = 0;
                    carrier_anim_texture = carrier_anim_cycle.next().unwrap();
                }
                game.get_carriers().iter().for_each(|&c| {
                    let pos = c.get_position();
                    let mut context =
                        ctx.trans(pos.x + CARRIER_ICON_X_OFFSET, pos.y + CARRIER_ICON_Y_OFFSET);
                    if is_facing_left(&c) {
                        context = context.flip_h().trans(-40.0, 0.0);
                    }

                    // Paint payload
                    if let Some(p) = c.get_payload() {
                        let texture = texture_depot.get(&p.cargo);
                        let context = ctx.trans(pos.x, pos.y);
                        Image::new_color([1.0, 1.0, 1.0, 0.85]).draw(
                            texture.unwrap(),
                            &context.draw_state,
                            context.transform,
                            g,
                        );
                    }

                    // Paint carrier itself
                    if draw_carriers {
                        let texture = texture_depot.get(&carrier_anim_texture);
                        Image::new_color([1.0, 1.0, 1.0, 1.0]).draw(
                            texture.unwrap(),
                            &ctx.draw_state,
                            context.transform,
                            g,
                        );
                    }
                });
            })
        });
    }

    Ok(())
}
