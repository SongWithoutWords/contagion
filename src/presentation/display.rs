use crate::core::vector::*;
use crate::core::matrix::*;
use crate::core::geo::polygon::*;
use crate::simulation::state::*;

use glium::Surface;
use glium::texture::texture2d::Texture2d;
use enum_map::EnumMap;
use crate::presentation::ui::gui::*;
use crate::presentation::ui::gui::GuiType;
use crate::presentation::ui::glium_text;
use crate::presentation::ui::glium_text::FontTexture;
use crate::presentation::ui::gui::Component;

// Enum ordered by draw order
#[derive(Copy, Clone, Debug, Enum)]
pub enum SpriteType {
    SelectionHighlight,
    Dead,
    Civilian,
    Zombie,
    Cop,
}

pub type Textures = EnumMap<SpriteType, Texture2d>;

pub fn load_textures(window: &glium_sdl2::SDL2Facade) -> Textures {
    use crate::presentation::graphics::renderer::load_texture;
    enum_map! {
        SpriteType::Cop                => load_texture(&window, "src/assets/police.png"),
        SpriteType::Zombie             => load_texture(&window, "src/assets/zombie.png"),
        SpriteType::Civilian           => load_texture(&window, "src/assets/citizen.png"),
        SpriteType::Dead               => load_texture(&window, "src/assets/dead_zombie.png"),
        SpriteType::SelectionHighlight => load_texture(&window, "src/assets/selection_highlight.png"),
    }
}

pub struct Programs {
    sprite_program: glium::Program,
    shadow_program: glium::Program,
    gui_program: glium::Program,
    shape_program: glium::Program
}
pub fn load_programs(window: &glium_sdl2::SDL2Facade) -> Programs {
    Programs {
        sprite_program: glium::Program::from_source(
            window,
            include_str!("graphics/sprite.vs.glsl"),
            include_str!("graphics/sprite.fs.glsl"), None).unwrap(),
        shadow_program: glium::Program::from_source(
            window,
            include_str!("graphics/shadow.vs.glsl"),
            include_str!("graphics/shadow.fs.glsl"), None).unwrap(),
        gui_program: glium::Program::from_source(
            window,
            include_str!("graphics/gui.vs.glsl"),
            include_str!("graphics/gui.fs.glsl"), None).unwrap(),
        shape_program: glium::Program::from_source(
            window,
            include_str!("graphics/shape.vs.glsl"),
            include_str!("graphics/shape.fs.glsl"), None).unwrap(),
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

#[derive(Copy, Clone)]
struct ColorVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(ColorVertex, position, tex_coords, color);

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

// TODO: fix
fn push_gui_vertices(buffer: &mut Vec<Vertex>, ui: &Gui) {
    let top_left  =  ui.top_left;
    let top_right =  ui.top_right;
    let bot_left  = ui.bot_left;
    let bot_right =  ui.bot_right;

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

// TODO: Assumes building is a rectangle and not an arbitrary polygon, consider generalizing
fn push_building_vertices(buffer: &mut Vec<ColorVertex>, building: &Polygon, color: [f32; 3]) {
    let top_left = building.get(0);
    let top_right = building.get(1);
    let bot_right = building.get(2);
    let bot_left = building.get(3);

    let vertex0 = ColorVertex {
        position: top_left.as_f32_array(),
        tex_coords: [0.0, 1.0],
        color
    };
    let vertex1 = ColorVertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0],
        color
    };
    let vertex2 = ColorVertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0],
        color
    };
    let vertex3 = ColorVertex {
        position: bot_right.as_f32_array(),
        tex_coords: [1.0, 0.0],
        color
    };
    buffer.push(vertex0);
    buffer.push(vertex1);
    buffer.push(vertex2);
    buffer.push(vertex1);
    buffer.push(vertex3);
    buffer.push(vertex2);
}

fn draw_sprites<U>(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    vertices: &Vec<Vertex>,
    program: &glium::Program,
    params: &glium::DrawParameters,
    uniforms: &U)
    where U: glium::uniforms::Uniforms
{
    frame.draw(
        &glium::VertexBuffer::new(window, vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        program,
        uniforms,
        params).unwrap();
}

fn draw_color_sprites<U>(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    vertices: &Vec<ColorVertex>,
    program: &glium::Program,
    params: &glium::DrawParameters,
    uniforms: &U)
    where U: glium::uniforms::Uniforms
{
    frame.draw(
        &glium::VertexBuffer::new(window, vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        program,
        uniforms,
        params).unwrap();
}

pub fn display(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    programs: &Programs,
    textures: &Textures,
    params: &glium::DrawParameters,
    state: &State, camera_frame: Mat4,
    ui: &mut Component,
    font: &FontTexture,
    ) {

    frame.clear_color(0.2, 0.2, 0.2, 1.0);

    let mut vertex_buffers = enum_map!{_ => vec!()};
    let mut vertex_buffers_gui = enum_map!{_ => vec!()};
    let mut vertex_buffers_building = vec!();

    // Compute the vertices in world coordinates of all entities
    for entity in &state.entities {
        let sprite_type = match entity.behaviour {
            Behaviour::Cop{..} => SpriteType::Cop,
            Behaviour::Dead => SpriteType::Dead,
            Behaviour::Human => SpriteType::Civilian,
            Behaviour::Zombie => SpriteType::Zombie,
        };
        push_sprite_vertices(&mut vertex_buffers[sprite_type], entity);
    }

    // Compute vertices for selection highlights
    let mut selection_count = 0;
    {
        for i in 0..state.is_selected.len() {
            if state.is_selected[i] {
                push_sprite_vertices(&mut vertex_buffers[SpriteType::SelectionHighlight], &state.entities[i]);
                // add more selection GUI to right
                selection_count += 1;
            }
        }
    }

    // Computer vertices for GUI
    let offset = 0.1;
    for component in &mut ui.components {
        if component.id == GuiType::Selected  {
            for i in 0..selection_count {
                // TODO: make selectetGUI move a bit to right when multiple entities are selected
//                component.move_pos( Vector2{x: offset * (selection_count as f64), y: 0.0});
                push_gui_vertices(&mut vertex_buffers_gui[GuiType::Selected], component);
            }
        } else {
            push_gui_vertices(&mut vertex_buffers_gui[component.id], component);
        }
    }

    // Compute vertices for buildings
    for building in &state.buildings {
        let color = [0.1, 0.1, 0.1];
        push_building_vertices(&mut vertex_buffers_building, building, color);
    }

    let camera_frame = camera_frame.as_f32_array();

    // Render shadows
    use crate::presentation::display::SpriteType::*;
    for sprite_type in &[Cop, Civilian, Dead, Zombie] {

        let uniforms = uniform! {
            matrix: camera_frame,
            tex: &textures[*sprite_type],
            height: match sprite_type { Dead => 0.5, _ => 1.0 } as f32
        };
        draw_sprites(
            frame,
            window,
            &vertex_buffers[*sprite_type],
            &programs.shadow_program,
            params,
            &uniforms);
    }

    // Render sprites
    for (sprite_type, vertex_buffer) in &vertex_buffers {

        let uniforms = uniform! {
            matrix: camera_frame,
            tex: &textures[sprite_type],
        };
        draw_sprites(
            frame,
            window,
            &vertex_buffer,
            &programs.sprite_program,
            params,
            &uniforms);
    }

    {
        let uniforms = uniform! {
            matrix: camera_frame
        };
        draw_color_sprites(
            frame,
            window,
            &vertex_buffers_building,
            &programs.shape_program,
            params,
            &uniforms);
    }

    // Render GUI
    for (_gui_type, vertex_buffer) in &vertex_buffers_gui {
        if _gui_type == GuiType::Selected {
            let uniforms = uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32],
                    ],
                    tex: &textures[SpriteType::Cop],
                };
            draw_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        } else {
            let uniforms = uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32],
                    ]
                };
            draw_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.gui_program,
                params,
                &uniforms);
        }
    }

    // Render Text
    let system = glium_text::TextSystem::new(window);
    let text = glium_text::TextDisplay::new(&system, font, "This is a demo");
    let color = [1.0, 1.0, 0.0, 1.0f32];
    let text_width = text.get_width();
    let (w, h) = frame.get_dimensions();
    let matrix = [
        [1.0 / text_width, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / text_width, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-1.0, 0.85, 0.0, 1.0f32],
    ];
    glium_text::draw(&text, &system, frame, matrix, color);
}
