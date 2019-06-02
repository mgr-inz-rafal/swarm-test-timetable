extern crate piston_window;
use piston_window::*;
use std::collections::HashMap;

const SCREEN_SIZE_NATIVE: [u32; 2] = [1920, 1080];

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum TextureId {
    Background,
    Test,
}

struct TextureDef {
    id: TextureId,
    path: &'static str,
}

const TEXTURE_REPOSITORY: [TextureDef; 2] = [
    TextureDef {
        id: TextureId::Background,
        path: "images/backgrounds/darlington.jpg",
    },
    TextureDef {
        id: TextureId::Test,
        path: "images/test_image.png",
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
            clear([0.0; 4], g);
            image(
                texture_depot.get(&TextureId::Background).unwrap(),
                c.transform,
                g,
            );

            let c1 = c.trans(300.0, 200.0);

            let bert_image = Image::new_color([1.0, 0.5, 0.5, 0.5]);
            let bert_tex = texture_depot.get(&TextureId::Test).unwrap();

            bert_image.draw(bert_tex, &c.draw_state, c1.transform, g);
        });

    }
}
