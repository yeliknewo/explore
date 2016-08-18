pub fn make_square_render() -> ::graphics::spritesheet::Packet {
    let vertices = vec!(
        ::graphics::spritesheet::Vertex::new([0.0, 0.0, 0.0], [1.0, 1.0]),
        ::graphics::spritesheet::Vertex::new([0.0, 1.0, 0.0], [1.0, 0.0]),
        ::graphics::spritesheet::Vertex::new([1.0, 1.0, 0.0], [0.0, 0.0]),
        ::graphics::spritesheet::Vertex::new([1.0, 0.0, 0.0], [0.0, 1.0]),
    );

    let indices = vec!(
        0, 3, 2, 2, 1, 0,
    );

    let rasterizer = ::gfx::state::Rasterizer::new_fill();

    ::graphics::spritesheet::Packet::new(vertices, indices, rasterizer)
}

pub mod p1 {
    pub const SIZE: [f32; 2] = [508.0, 288.0];

    pub const DUCK: [f32; 4] = [365.0, 98.0, 69.0, 71.0];
    pub const FRONT: [f32; 4] = [0.0, 196.0, 66.0, 92.0];
    pub const HURT: [f32; 4] = [438.0, 0.0, 69.0, 92.0];
    pub const JUMP: [f32; 4] = [438.0, 93.0, 67.0, 94.0];
    pub const STAND: [f32; 4] = [67.0, 196.0, 66.0, 92.0];
    pub const WALK01: [f32; 4] = [0.0, 0.0, 72.0, 97.0];
    pub const WALK02: [f32; 4] = [73.0, 0.0, 72.0, 97.0];
    pub const WALK03: [f32; 4] = [146.0, 0.0, 72.0, 97.0];
    pub const WALK04: [f32; 4] = [0.0, 98.0, 72.0, 97.0];
    pub const WALK05: [f32; 4] = [73.0, 98.0, 72.0, 97.0];
    pub const WALK06: [f32; 4] = [146.0, 98.0, 72.0, 97.0];
    pub const WALK07: [f32; 4] = [219.0, 0.0, 72.0, 97.0];
    pub const WALK08: [f32; 4] = [292.0, 0.0, 72.0, 97.0];
    pub const WALK09: [f32; 4] = [219.0, 98.0, 72.0, 97.0];
    pub const WALK10: [f32; 4] = [365.0, 0.0, 72.0, 97.0];
    pub const WALK11: [f32; 4] = [292.0, 98.0, 72.0, 97.0];
}

pub mod tiles {
    pub const SIZE: [f32; 2] = [914.0, 936.0];

    pub const GRASS: [f32; 4] = [648.0, 0.0, 70.0, 70.0];
    pub const GRASS_MID: [f32; 4] = [504.0, 576.0, 70.0, 70.0];
}
