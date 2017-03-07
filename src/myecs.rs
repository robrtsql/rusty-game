use std::cell::Cell;
use std::cell::RefCell;
use ecs::*;
use ecs::system::*;
use anim::*;
use graphics::*;

components! {
    struct MyComponents {
        #[hot] sprite_animator: SpriteAnimator,
    }
}

#[derive(Default)]
pub struct GraphicsService {
    pub dt: Cell<f32>,
    pub graphics: Graphics,
}

#[derive(Default)]
pub struct MyServices {
    pub graphics: GraphicsService,
}

impl ServiceManager for MyServices {
}

pub struct HelloWorldProcess;

impl System for HelloWorldProcess { type Components = MyComponents; type Services = MyServices; }

impl EntityProcess for HelloWorldProcess {
    fn process(&mut self, entities: EntityIter<MyComponents>, data: &mut DataHelper<MyComponents, MyServices>) {
        for e in entities {
            data.services.graphics.render(&data.animator, 100, 100, 2, data.services.graphics.dt.get());
        }
    }
}

systems! {
    struct MySystems<MyComponents, MyServices> {
        active: {
            motion: EntitySystem<HelloWorldProcess> = EntitySystem::new(
                HelloWorldProcess,
                aspect!(<MyComponents> all: [sprite_animator])
            ),
        },
        passive: {}
    }
}