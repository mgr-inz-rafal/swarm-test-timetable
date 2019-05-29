extern crate piston_window;
use piston_window::*;
use std::collections::HashMap;
use std::hash::Hash;

const SCREEN_SIZE_NATIVE: [u32; 2] = [1920, 1080];

#[derive(Eq, PartialEq, Hash)]
enum Textures {
    Background,
}

fn load_textures(depot: &mut HashMap<Textures, G2dTexture>, context: &mut G2dTextureContext) {
    let test_image: G2dTexture = Texture::from_path(
        context,
        "images/backgrounds/darlington.jpg",
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();

    depot.insert(Textures::Background, test_image);
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
            clear([1.0; 4], g);
            image(texture_depot.get(&Textures::Background).unwrap(), c.transform, g);
        });
    }
}
