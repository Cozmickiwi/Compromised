use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, System, WriteStorage, ReadStorage},
    input::{InputHandler, StringBindings}, winit::VirtualKeyCode,
};

use crate::compromised::{
    Compromised, AREA_HEIGHT, AREA_WIDTH, CAR_HEIGHT, CAR_WIDTH, Carwell,
};

pub struct MovementSystem {
    jump_active: bool,
    jump_decent: bool,
    //jump_pos: f32,
    jump_start: f32,
}

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem {
            jump_active: false,
            jump_decent: false,
            //jump_pos: 0.0,
            jump_start: 0.0,
        }
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
        let space_down = input.key_is_down(VirtualKeyCode::Space);
        for(_car, transform) in (&carwell, &mut transforms).join() {
            if transform.translation().x > 0.0 {
                if a_down {
                    transform.prepend_translation_x(-2.0);
                }
            } 
            if transform.translation().x < 100.0 {
                if d_down {
                    transform.prepend_translation_x(2.0);
                }
            }
            if space_down && self.jump_active == false {
                self.jump_active = true;
                self.jump_start = transform.translation().y;
                transform.prepend_translation_y(1.0);
                //self.jump_pos = transform.translation().y;

            }
            if self.jump_active {
                let speed = (((self.jump_start + 13.0) - transform.translation().y) + 1.0) / 13.0;
                //println!("{speed}");
                if transform.translation().y >= self.jump_start + 13.0 {
                    self.jump_decent = true;
                    transform.prepend_translation_y(-0.75 - speed);
                } else if self.jump_decent {
                    transform.prepend_translation_y(-0.75 - speed);
                    if transform.translation().y <= self.jump_start {
                        transform.set_translation_y(self.jump_start);
                        self.jump_decent = false;
                        self.jump_active = false;
                    }
                } else {
                    transform.prepend_translation_y(0.75 + speed);
                }
            }
        }
    }
}
