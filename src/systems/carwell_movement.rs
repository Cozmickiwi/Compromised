use std::fmt::Debug;

use amethyst::{
    core::{math::Vector3, Transform},
    derive::SystemDesc,
    ecs::{DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    winit::VirtualKeyCode,
};

use crate::compromised::{
    Block, Carwell, Compromised, AREA_HEIGHT, AREA_WIDTH, BLOCK_WIDTH, CAR_HEIGHT, CAR_WIDTH, BLOCK_HEIGHT,
};

pub struct MovementSystem {
    jump_active: bool,
    jump_decent: bool,
    jump_start: f32,
    falling: bool,
    on_block: bool,
    block_width: f32,
    block_x_pos: f32,
    block_y_pos: f32,
}

impl MovementSystem {
    pub fn new() -> Self {
        MovementSystem {
            jump_active: false,
            jump_decent: false,
            jump_start: 0.0,
            falling: false,
            on_block: false,
            block_width: 0.0,
            block_x_pos: 0.0,
            block_y_pos: 0.0,
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
        let mut carwell_ypos: f32 = 0.0;
        let mut collision = false;
        for (_car, transform) in (&carwell, &transforms).join() {
            carwell_xpos = transform.translation().x;
            carwell_ypos = transform.translation().y - (CAR_HEIGHT / 2.0);
        }
        for (_blc, transform) in (&block, &transforms).join() {
            if carwell_xpos + (CAR_WIDTH / 2.0) >= transform.translation().x - (BLOCK_WIDTH / 2.0)
                && carwell_xpos - (CAR_WIDTH / 2.0)
                    <= transform.translation().x + (BLOCK_WIDTH / 2.0)
                && carwell_ypos < transform.translation().y + (BLOCK_HEIGHT / 2.0)
            {
                collision = true;
                (
                    self.block_width,
                    self.block_x_pos,
                    self.block_y_pos,
                ) = (BLOCK_WIDTH, transform.translation().x, transform.translation().y);
                if carwell_ypos + 2.0 >= transform.translation().y + (BLOCK_HEIGHT / 2.0) {
                    self.falling = false;
                    self.on_block = true;
                    collision = false;
                }
            }
        }
        for (_car, transform) in (&carwell, &mut transforms).join() {
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
                if d_down && !collision {
                    transform.prepend_translation_x(2.0);
                    if transform.scale().x < 0.0 {
                        let new_scale =
                            Vector3::new(-transform.scale().x, transform.scale().y, 1.0);
                        transform.set_scale(new_scale);
                    }
                }
            }
            if space_down && !self.jump_active && !self.falling {
                self.jump_active = true;
                self.jump_start = transform.translation().y;
                transform.prepend_translation_y(1.0);
            }
            if self.jump_active && !self.falling {
                let speed = (((self.jump_start + 13.0) - transform.translation().y) + 1.0) / 13.0;
                if transform.translation().y >= self.jump_start + 13.0 {
                    self.falling = true;
                    self.jump_active = false;
                } else {
                    transform.prepend_translation_y(0.75 + speed);
                } /* else if self.jump_decent {
                      transform.prepend_translation_y(-0.75 - speed);
                      if transform.translation().y <= self.jump_start {
                          transform.set_translation_y(self.jump_start);
                          self.jump_decent = false;
                          self.jump_active = false;
                      }
                  } else {
                      transform.prepend_translation_y(0.75 + speed);
                  }*/
            } else if self.falling {
                if !fall(transform) {
                    self.falling = false;
                }
            }
            if self.on_block {
                if transform.translation().x + (CAR_WIDTH / 2.0) < self.block_x_pos - (BLOCK_WIDTH / 2.0) {
                    self.falling = true;
                    self.on_block = false;
                }
            }
        }
    }
}

pub fn fall<'a>(t: &'a mut Transform) -> bool {
    t.prepend_translation_y(-1.0);
    if t.translation().y <= 15.0 {
        t.set_translation_y(15.0);
        return false;
    }
    return true;
}
