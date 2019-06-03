extern crate piston_window;
use piston_window::*;
use std::collections::HashMap;

const SCREEN_SIZE_NATIVE: [u32; 2] = [1920, 1080];
const TILE_WIDTH: u32 = 44;
const TILE_HEIGHT: u32 = 54;
const TILE_SPACING: u32 = (50 - TILE_WIDTH) / 2;
const TILES_PER_ROW: u32 = 36;
const BOARD_MARGIN: u32 = (SCREEN_SIZE_NATIVE[0] - (TILE_WIDTH + TILE_SPACING) * TILES_PER_ROW) / 2;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum TextureId {
    Background,
    Test,
    TileBlank,
    TileA,
    TileR,
    TileW,
}

struct TextureDef {
    id: TextureId,
    path: &'static str,
}

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

fn main() {
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

    let mut ctx = window.create_texture_context();
    let mut texture_depot = HashMap::new();
    load_textures(&mut texture_depot, &mut ctx);

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

            // Paint tiles

            let c1 = c.trans(300.0, 200.0);

            let bert_image = Image::new_color([1.0, 0.5, 0.5, 0.5]);
            let bert_tex = texture_depot.get(&TextureId::Test).unwrap();

            bert_image.draw(bert_tex, &c.draw_state, c1.transform, g);
        });

    }
}
