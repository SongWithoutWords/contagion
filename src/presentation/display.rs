use crate::core::vector::*;
use crate::core::scalar::*;
use crate::simulation::state::*;

use glium::Surface;
use glium::texture::texture2d::Texture2d;

pub struct Textures {
    pub zombies: Texture2d,
    pub police: Texture2d,
    pub citizen: Texture2d,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn push_sprite_vertices(buffer: &mut Vec<Vertex>, position: Vector2) {

    // 0      1
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 2      3

    // TODO: Account for rotation/facing
    let vertex0 = Vertex {
        position: [position.x as f32 - 0.5, position.y as f32 + 0.5],
        tex_coords: [0.0, 1.0]
    };
    let vertex1 = Vertex {
        position: [position.x as f32 + 0.5, position.y as f32 + 0.5],
        tex_coords: [1.0, 1.0]
    };
    let vertex2 = Vertex {
        position: [position.x as f32 - 0.5, position.y as f32 - 0.5],
        tex_coords: [0.0, 0.0]
    };
    let vertex3 = Vertex {
        position: [position.x as f32 + 0.5, position.y as f32 - 0.5],
        tex_coords: [1.0, 0.0]
    };
    buffer.push(vertex0);
    buffer.push(vertex1);
    buffer.push(vertex2);
    buffer.push(vertex1);
    buffer.push(vertex3);
    buffer.push(vertex2);
}

fn draw_sprites(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    vertices: &Vec<Vertex>,
    program: &glium::Program,
    camera_frame: [[f32; 4]; 4],
    texture: &Texture2d,
    params: &glium::DrawParameters,
) {
    let uniforms = uniform! {
        matrix: camera_frame,
        tex: texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        program,
        &uniforms,
        params).unwrap();
}

pub fn display(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    program: &glium::Program,
    textures: &Textures,
    params: &glium::DrawParameters,
    state: &State, camera_frame: [[f32;4];4]) {

    frame.clear_color(0.2, 0.2, 0.2, 1.0);

    let camera_frame = camera_frame;

    let mut cop_vertices = Vec::new();
    let mut human_vertices = Vec::new();
    let mut zombie_vertices = Vec::new();

    // Compute the vertices in world coordinates of all entities
    for entity in &state.entities {

        let position = entity.position;

        match entity.behaviour {
            Behaviour::Cop => push_sprite_vertices(&mut cop_vertices, position),
            Behaviour::Dead =>
            // TODO: Draw a corpse
            // or if that's not what we want for the tone of the game, then don't!
                (),
            Behaviour::Human => push_sprite_vertices(&mut human_vertices, position),
            Behaviour::Zombie => push_sprite_vertices(&mut zombie_vertices, position),
        }
    }

    // Make the draw calls
    draw_sprites(frame, window, &cop_vertices, program, camera_frame, &textures.police, params);
    draw_sprites(frame, window, &human_vertices, program, camera_frame, &textures.citizen, params);
    draw_sprites(frame, window, &zombie_vertices, program, camera_frame, &textures.zombies, params);
}