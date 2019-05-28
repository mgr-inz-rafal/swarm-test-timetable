
extern crate piston_window;
use piston_window::*;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new(
        "Timetable: Demo of the Swarm library by mgr. inż. Rafał",
        [1920, 1080],
    )
    .exit_on_esc(true)
    .fullscreen(true)
    .opengl(opengl)
    .build()
    .unwrap();

    let test_image: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        "images/test_image.png",
        Flip::None,
        &TextureSettings::new(),
    )
    .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0; 4], g);
            image(&test_image, c.transform, g);
        });
    }
}
