use bevy::prelude::*;

pub const BUFFER_LEN: usize = 32;

pub struct CircularBuffer<T: Default + Copy>([T; BUFFER_LEN]);

impl<T: Default + Copy> CircularBuffer<T> {

    pub fn new() -> CircularBuffer<T> {
        return CircularBuffer([T::default(); BUFFER_LEN]);
    }

    pub fn new_from(item: T) -> CircularBuffer<T> {
        return CircularBuffer([item; BUFFER_LEN]);
    }

    pub fn get(&self, tick: u16) -> &T {
        let i = tick as usize % BUFFER_LEN;
        &self.0[i]
    }

    pub fn set(&mut self, tick: u16, input: T) {
        let i = tick as usize % BUFFER_LEN;
        self.0[i] = input;
    }
}

#[derive(Component)]
pub struct PosBuffer(pub CircularBuffer<Vec2>);
