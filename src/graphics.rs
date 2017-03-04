use std::collections::HashMap;
use std::path::Path;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::image::*;
use sdl2::render::*;
use anim::SpriteAnimator;

pub struct Graphics<'a> {
    pub textures: HashMap<String, Texture>,
    pub renderer: Renderer<'a>,
}

impl<'a> Graphics<'a> {
    pub fn new(renderer: Renderer<'a>) -> Graphics<'a> {
        return Graphics {
            textures: HashMap::new(),
            renderer: renderer,
        };
    }

    pub fn load_texture(&mut self, texture_path: String) {
        let image = self.renderer.load_texture(Path::new(&texture_path)).unwrap();
        self.textures.insert(texture_path, image);
    }

    pub fn render(&mut self,
                  texture_path: &String,
                  animator: &SpriteAnimator,
                  x: i32,
                  y: i32,
                  zoom: u32,
                  dt: f32) {
        animator.update_frame_index(dt);
        let ref playback = animator.playback.borrow();
        let ref current_frame = animator.sheet.anims[&playback.current_anim]
            .get(playback.current_frame_index as usize)
            .unwrap()
            .frame;
        let source_rect = Rect::new(current_frame.x,
                                    current_frame.y,
                                    current_frame.w,
                                    current_frame.h);
        let mut dest_rect = Rect::new(current_frame.x,
                                      current_frame.y,
                                      current_frame.w * zoom,
                                      current_frame.h * zoom);
        dest_rect.center_on(Point::new(x, y));
        let ref texture = self.textures.get(texture_path).unwrap();
        self.renderer.copy(texture, Some(source_rect), Some(dest_rect)).expect("Render failed");
        self.renderer.present();
        self.renderer.clear();
    }
}
