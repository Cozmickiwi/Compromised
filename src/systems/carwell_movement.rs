use std::fmt::Debug;

use amethyst::{
    core::{math::Vector3, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, WriteStorage, DenseVecStorage},
    input::{InputHandler, StringBindings},
    winit::VirtualKeyCode,
};

use crate::compromised::{Carwell, Compromised, AREA_HEIGHT, AREA_WIDTH, CAR_HEIGHT, CAR_WIDTH, Block, BLOCK_WIDTH};

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
        ReadStorage<'s, Block>,
        Read<'s, InputHandler<StringBindings>>,
    );
    fn run(&mut self, (mut transforms, carwell, block, input): Self::SystemData) {
        let d_down = input.key_is_down(VirtualKeyCode::D);
        let a_down = input.key_is_down(VirtualKeyCode::A);
        let space_down = input.key_is_down(VirtualKeyCode::Space);
        let mut carwell_xpos: f32 = 0.0;
        let mut collision = false;
        for (_car, transform) in (&carwell, &mut transforms).join() {
            carwell_xpos = transform.translation().x;
            if transform.translation().x > 0.0 {
                if a_down {
                    transform.prepend_translation_x(-2.0);
                    if transform.scale().x > 0.0 {
                        let new_scale =
                            Vector3::new(-transform.scale().x, transform.scale().y, 1.0);
                        transform.set_scale(new_scale);
                    }
                }
            }
            if transform.translation().x < 100.0 {
                if d_down {
                    transform.prepend_translation_x(2.0);
                    if transform.scale().x < 0.0 {
                        let new_scale =
                            Vector3::new(-transform.scale().x, transform.scale().y, 1.0);
                        transform.set_scale(new_scale);
                    }
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
        for (blc, transform) in (&block, &transforms).join() {
            if carwell_xpos + (CAR_WIDTH / 2.0) >= transform.translation().x - (BLOCK_WIDTH / 2.0) 
                && carwell_xpos - (CAR_WIDTH / 2.0) <= transform.translation().x + (BLOCK_WIDTH / 2.0) {
                println!("Collision!!");
            }
        }
    }
}
