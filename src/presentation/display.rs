use crate::core::vector::*;
use crate::core::scalar::*;
use crate::core::matrix::*;
use crate::simulation::state::*;

use glium::Surface;
use glium::texture::texture2d::Texture2d;

pub struct Textures {
    pub zombies: Texture2d,
    pub dead_zombie: Texture2d,
    pub police: Texture2d,
    pub selection_highlight: Texture2d,
    pub citizen: Texture2d,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn push_sprite_vertices(buffer: &mut Vec<Vertex>, entity: &Entity) {

    let position = entity.position;
    let up = entity.get_facing_normal();
    let right = vector2(up.y, -up.x);

    let top_left  = position - 0.5 * right + 0.5 * up;
    let top_right = position + 0.5 * right + 0.5 * up;
    let bot_left  = position - 0.5 * right - 0.5 * up;
    let bot_right = position + 0.5 * right - 0.5 * up;

    // 0      1
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 2      3

    let vertex0 = Vertex {
        position: top_left.as_f32_array(),
        tex_coords: [0.0, 1.0]
    };
    let vertex1 = Vertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0]
    };
    let vertex2 = Vertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0]
    };
    let vertex3 = Vertex {
        position: bot_right.as_f32_array(),
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
    camera_frame: Mat4,
    texture: &Texture2d,
    params: &glium::DrawParameters,
) {
    let camera_frame = camera_frame.as_f32_array();
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
    state: &State, camera_frame: Mat4) {

    frame.clear_color(0.2, 0.2, 0.2, 1.0);

    let mut cop_vertices = Vec::new();
    let mut dead_vertices = Vec::new();
    let mut human_vertices = Vec::new();
    let mut zombie_vertices = Vec::new();

    // Compute the vertices in world coordinates of all entities
    for entity in &state.entities {
        match entity.behaviour {
            Behaviour::Cop{..} => push_sprite_vertices(&mut cop_vertices, entity),
            Behaviour::Dead => push_sprite_vertices(&mut dead_vertices, entity),
            Behaviour::Human => push_sprite_vertices(&mut human_vertices, entity),
            Behaviour::Zombie => push_sprite_vertices(&mut zombie_vertices, entity),
        }
    }

    // Make the draw calls
    draw_sprites(frame, window, &cop_vertices, program, camera_frame, &textures.police, params);
    draw_sprites(frame, window, &dead_vertices, program, camera_frame, &textures.dead_zombie, params);
    draw_sprites(frame, window, &human_vertices, program, camera_frame, &textures.citizen, params);
    draw_sprites(frame, window, &zombie_vertices, program, camera_frame, &textures.zombies, params);

    {
        let mut selection_highlight_vertices = Vec::new();
        for i in 0..state.is_selected.len() {
            if state.is_selected[i] {
                push_sprite_vertices(&mut selection_highlight_vertices, &state.entities[i]);
            }
        }
        draw_sprites(frame, window, &selection_highlight_vertices, program, camera_frame, &textures.selection_highlight, params);
}

}
