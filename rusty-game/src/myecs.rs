use std::cell::Cell;
use ecs::*;
use ecs::system::*;
use anim::*;

/*
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Animator<'a> {
    pub spriteAnimator: SpriteAnimator<'a>
}
*/

components! {
    struct MyComponents {
        //#[hot] animator: Animator<'a>,
    }
}

#[derive(Default)]
pub struct DeltaService {
    pub time: Cell<f32>,
}

#[derive(Default)]
pub struct MyServices {
    pub delta: DeltaService,
}

impl ServiceManager for MyServices {
}

pub struct MotionProcess;

impl System for MotionProcess { type Components = MyComponents; type Services = MyServices; }

impl EntityProcess for MotionProcess {
    fn process(&mut self, entities: EntityIter<MyComponents>, data: &mut DataHelper<MyComponents, MyServices>) {
        /*
        for e in entities {
            let mut position = data.position[e];
            let velocity = data.velocity[e];
            position.x += velocity.dx * data.services.delta.time.get();
            position.y += velocity.dy * data.services.delta.time.get();
            data.position[e] = position;
        }
        */
    }
}

systems! {
    struct MySystems<MyComponents, MyServices> {
        active: {
            /*
            motion: EntitySystem<MotionProcess> = EntitySystem::new(
                MotionProcess,
                aspect!(<MyComponents> all: [position, velocity])
            ),
            */
        },
        passive: {}
    }
}