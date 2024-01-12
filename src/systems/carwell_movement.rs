use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, System, WriteStorage, ReadStorage},
    input::{InputHandler, StringBindings}, winit::VirtualKeyCode,
};

use crate::compromised::{
    Compromised, AREA_HEIGHT, AREA_WIDTH, CAR_HEIGHT, CAR_WIDTH, Carwell,
};

pub struct MovementSystem;

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem {}
    }
}

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Carwell>,
    Read<'s, InputHandler<StringBindings>>,
    );
    fn run(&mut self, (mut transforms, carwell, input): Self::SystemData) {
        let d_down = input.key_is_down(VirtualKeyCode::D);
        let a_down = input.key_is_down(VirtualKeyCode::A);
        for(_car, transform) in (&carwell, &mut transforms).join() {
            if transform.translation().x > 0.0 {
                if a_down {
                    transform.prepend_translation_x(-1.0);
                }
            } 
            if transform.translation().x < 100.0 {
                if d_down {
                    transform.prepend_translation_x(1.0);
                }
            }
        }
    }
}
