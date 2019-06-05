extern crate piston_window;
extern crate swarm;

use piston_window::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};
use swarm::{Carrier, Payload, Slot};

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
const EMPTY_PAYLOAD: char = '~';

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum TextureId {
    Background,
    Carrier,
    Test,
    TileBlank,
    TileA,
    TileR,
    TileW,
}

impl TextureId {
    fn from_char(c: char) -> TextureId {
        match c {
            'A' | 'a' => TextureId::TileA,
            'R' | 'r' => TextureId::TileR,
            'W' | 'w' => TextureId::TileW,
            _ => TextureId::TileBlank,
        }
    }
}

struct TextureDef {
    id: TextureId,
    path: &'static str,
}

type MyGameType = swarm::Swarm<TextureId>;

const TEXTURE_REPOSITORY: [TextureDef; 7] = [
    TextureDef {
        id: TextureId::Test,
        path: "images/test_image.png",
    },
    TextureDef {
        id: TextureId::Carrier,
        path: "images/carrier/carrier.png",
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
        id: TextureId::TileA,
        path: "images/tiles/tile_A.png",
    },
    TextureDef {
        id: TextureId::TileR,
        path: "images/tiles/tile_R.png",
    },
    TextureDef {
        id: TextureId::TileW,
        path: "images/tiles/tile_W.png",
    },
];

fn load_textures(depot: &mut HashMap<TextureId, G2dTexture>, context: &mut G2dTextureContext) {
    TEXTURE_REPOSITORY.iter().for_each(|x| {
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

fn char_to_payload(c: char) -> Option<swarm::Payload<TextureId>> {
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
            let mut flip_flop = true;
            let mut payload_being_set = None;
            line.unwrap()
                .chars()
                .filter(|c| !is_tile_delimiter(*c))
                .enumerate()
                .for_each(|(x, c)| {
                    if flip_flop {
                        payload_being_set = char_to_payload(c);
                        flip_flop = false;
                    } else {
                        game.add_slot(Slot::new(
                            (BOARD_LEFT_MARGIN + (TILE_WIDTH + TILE_SPACING) * (x / 2) as u32)
                                as f64,
                            (BOARD_TOP_MARGIN + (TILE_HEIGHT + TILE_SPACING) * y as u32) as f64,
                            payload_being_set,
                            char_to_payload(c),
                            swarm::SlotKind::CLASSIC,
                        ));
                        flip_flop = true;
                    };
                })
        });

    Ok(())
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

    let mut game = swarm::Swarm::new();

    let mut ctx = window.create_texture_context();
    let mut texture_depot = HashMap::new();
    load_textures(&mut texture_depot, &mut ctx);

    load_layout(&mut game, 1)?;

    game.add_carrier(Carrier::new(50.0, 50.0));

    while let Some(e) = window.next() {
        game.tick();
        window.draw_2d(&e, |ctx, g, _| {
            // Clear
            clear([0.0; 4], g);

            // Paint backgrouns
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
            game.get_carriers().iter().for_each(|&c| {
                let pos = c.get_position();
                let context = ctx.trans(pos.x, pos.y);

                let texture = texture_depot.get(&TextureId::Carrier);

                Image::new_color([1.0, 1.0, 1.0, 1.0]).draw(
                    texture.unwrap(),
                    &ctx.draw_state,
                    context.transform,
                    g,
                );
            });
        });
    }

    Ok(())
}
