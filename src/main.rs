extern crate piston_window;
extern crate swarm;

use piston_window::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Result};
use swarm::{Payload, Slot};

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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum TextureId {
    Background,
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

const TEXTURE_REPOSITORY: [TextureDef; 6] = [
    TextureDef {
        id: TextureId::Test,
        path: "images/test_image.png",
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

fn load_layout(game: &mut MyGameType, id: u32) -> Result<()> {
    let file = format!("layouts/layout{}.txt", id);
    println!("Loading layout from '{}'", file);
    let file = File::open(file)?;
    let mut buffer = BufReader::new(file);
    buffer.by_ref().lines().enumerate().for_each(|(y, line)| {
        line.unwrap().chars().enumerate().for_each(|(x, c)| {
            game.add_slot(Slot::new(
                (BOARD_LEFT_MARGIN + (TILE_WIDTH + TILE_SPACING) * x as u32) as f64,
                (BOARD_TOP_MARGIN + (TILE_HEIGHT + TILE_SPACING) * y as u32) as f64,
                Some(Payload::new(TextureId::from_char(c))),
                Some(Payload::new(TextureId::from_char(c))),
                swarm::SlotKind::CLASSIC,
            ))
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

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            // Clear
            clear([0.0; 4], g);

            // Paint backgrouns
            image(
                texture_depot.get(&TextureId::Background).unwrap(),
                c.transform,
                g,
            );

            // Paint slots
            game.get_slots().iter().for_each(|&s| {
                let pos = s.get_position();
                let context = c.trans(pos.x, pos.y);
                Image::new_color([1.0, 1.0, 1.0, 0.85]).draw(
                    texture_depot
                        .get(&s.get_payloads()[0].unwrap().cargo)
                        .unwrap(),
                    &c.draw_state,
                    context.transform,
                    g,
                );
            });
        });
    }

    Ok(())
}
