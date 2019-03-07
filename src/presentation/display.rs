use crate::core::scalar::*;
use crate::core::vector::*;
use crate::core::matrix::*;
use crate::core::geo::polygon::*;
use crate::simulation::state::*;
use crate::simulation::control::*;

use glium::Surface;
use glium::texture::texture2d::Texture2d;
use enum_map::EnumMap;
use crate::presentation::ui::gui::*;
use crate::presentation::ui::gui::GuiType;
use crate::presentation::ui::glium_text;
use crate::presentation::ui::glium_text::FontTexture;
use crate::presentation::ui::gui::Component;

// Enum ordered by draw order
#[derive(Copy, Clone, Debug, Enum, PartialEq)]
pub enum SpriteType {
    SelectionHighlight,
    Dead,
    BulletCasing,
    Civilian,
    Zombie,
    Cop,
    BulletInAir,
    Menu,
    MenuWindow,
    Button,
    InstructionMenu,
    CopIcon,
    ZombieWorldIcon,
    CopWorldIcon,
    CivilianWorldIcon
}

// pub type Textures = EnumMap<SpriteType, Texture2d>;
pub struct Textures {
    sprite_textures: EnumMap<SpriteType, Texture2d>,
    background_texture: Texture2d,

}

pub fn load_textures(window: &glium_sdl2::SDL2Facade) -> Textures {
    use crate::presentation::graphics::renderer::load_texture;

    Textures {
        sprite_textures: enum_map! {
            SpriteType::SelectionHighlight
                => load_texture(window, "assets/images/other/selection_highlight.png"),
            SpriteType::Dead
                => load_texture(window, "assets/images/old/dead_zombie.png"),
            SpriteType::BulletCasing
                => load_texture(window, "assets/images/other/bullet_casing_straight.png"),
            SpriteType::Civilian
                => load_texture(window, "assets/images/old/citizen.png"),
            SpriteType::Zombie
                => load_texture(window, "assets/images/old/zombie.png"),
            SpriteType::Cop
                => load_texture(window, "assets/images/old/police.png"),
            SpriteType::BulletInAir
                => load_texture(window, "assets/images/other/flying_bullet_long.png"),
            SpriteType::Menu
                => load_texture(window, "assets/images/ui/menu_icon.png"),
            SpriteType::MenuWindow
                => load_texture(window, "assets/images/ui/menu_icon.png"),
            SpriteType::Button
                => load_texture(window, "assets/images/other/selection_highlight.png"),
            SpriteType::InstructionMenu
                => load_texture(window, "assets/images/ui/instruction_menu_transparent.png"),
            SpriteType::CopIcon
                => load_texture(window, "assets/images/old/badge_icon.png"),
            SpriteType::ZombieWorldIcon
                => load_texture(window, "assets/images/ui/zombie_world_icon.png"),
            SpriteType::CopWorldIcon
                => load_texture(window, "assets/images/ui/cop_world_icon.png"),
            SpriteType::CivilianWorldIcon
                => load_texture(window, "assets/images/ui/civilian_world_icon.png")
        },
        background_texture: load_texture(&window, "assets/images/background_concrete.png")
    }
}

pub struct Programs {
    background_program: glium::Program,
    sprite_program: glium::Program,
    shadow_program: glium::Program,
    gui_program: glium::Program,
    shape_program: glium::Program
}
pub fn load_programs(window: &glium_sdl2::SDL2Facade) -> Programs {
    Programs {
        background_program: glium::Program::from_source(
            window,
            include_str!("graphics/background.vs.glsl"),
            include_str!("graphics/background.fs.glsl"), None).unwrap(),
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
struct VertexPosition {
    position: [f32; 2],
}
implement_vertex!(VertexPosition, position);

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
    color: [f32; 4],
}
implement_vertex!(ColorVertex, position, tex_coords, color);

struct Sprite {
    position: Vector2,
    facing: Vector2,
    radius: Scalar,
}

fn push_sprite_vertices(buffer: &mut Vec<Vertex>, sprite: &Sprite) {

    let position = sprite.position;
    let up = sprite.radius * sprite.facing;
    let right = up.right(); //vector2(up.y, -up.x);

    let top_left  = position - right + up;
    let top_right = position + right + up;
    let bot_left  = position - right - up;
    let bot_right = position + right - up;

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
fn push_gui_vertices(buffer: &mut Vec<ColorVertex>, ui: &Gui) {
    let top_left  =  ui.top_left;
    let top_right =  ui.top_right;
    let bot_left  = ui.bot_left;
    let bot_right =  ui.bot_right;
    let mut color= [0.0, 0.0, 0.0, 1.0];

    match ui.id {
        GuiType::SelectionDrag => {color = [0.105, 0.214, 0.124, 0.3]},
        GuiType::Button{..} => {color = [0.6, 0.7, 0.8, 0.5]},
        GuiType::Window => {color= [0.0, 0.0, 0.0, 0.7]},
        GuiType::Menu {..} => {color= [0.6, 0.7, 0.8, 0.5]},
        _ => (),
    };

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

// TODO: Assumes building is a rectangle and not an arbitrary polygon, consider generalizing
fn push_building_vertices(buffer: &mut Vec<ColorVertex>, building: &Polygon, color: [f32; 4]) {
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


fn draw_background(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    // This is about as large as you can get without introducing artifacts
    // due to floating point imprecision
    let extent = 1e5;

    let top_left = VertexPosition {
        position: [-extent,  extent],
    };
    let top_right = VertexPosition {
        position: [ extent,  extent],
    };
    let bot_left = VertexPosition {
        position: [-extent, -extent],
    };
    let bot_right = VertexPosition {
        position: [ extent, -extent],
    };

    // tl    tr
    //  +----+
    //  |  / |
    //  | /  |
    //  +----+
    // bl    br

    let vertices = vec!(
        top_left,
        top_right,
        bot_left,
        top_right,
        bot_right,
        bot_left,
    );

    let uniforms = uniform! {
        matrix: camera_frame,
        tex: &textures.background_texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.background_program,
        &uniforms,
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
    control: &Control
) {

    frame.clear_color(0.2, 0.2, 0.2, 1.0);

    let camera_frame = camera_frame.as_f32_array();

    draw_background(frame, window, textures, programs, camera_frame, params);

    let mut vertex_buffers = enum_map!{_ => vec!()};
    let mut vertex_buffers_gui = enum_map!{_ => vec!()};
    let mut vertex_buffers_building = vec!();
    let mut text_buffers = vec!();

    let mut cop_count = 0;

    let mut human_count = 0;
    let mut zombie_count = 0;
    let mut _dead_count = 0;
    let mut _magazine_count = vec!();
    let mut _menu_buttons: Vec<(Vector2, Vector2, Vector2, Vector2)> = vec![];


    // Compute the vertices in world coordinates of all projectiles
    for p in &state.projectiles {
        let sprite_type = match p.kind {
            ProjectileKind::Bullet => SpriteType::BulletInAir,
            ProjectileKind::Casing => SpriteType::BulletCasing,
        };
        let sprite = Sprite {
            position: p.position,
            facing: p.velocity.normalize(),
            radius: BULLET_RADIUS,
        };
        push_sprite_vertices(&mut vertex_buffers[sprite_type], &sprite);
    }

    // Compute the vertices in world coordinates of all entities
    for entity in &state.entities {
        let sprite_type = match entity.behaviour {
            Behaviour::Cop{..} => {cop_count+=1; SpriteType::Cop},
            Behaviour::Dead => {_dead_count+=1; SpriteType::Dead},
            Behaviour::Human => {human_count+=1; SpriteType::Civilian},
            Behaviour::Zombie => {zombie_count+=1; SpriteType::Zombie},
        };
        let sprite = Sprite {
            position: entity.position,
            facing: entity.get_facing_normal(),
            radius: 0.5,
        };
        push_sprite_vertices(&mut vertex_buffers[sprite_type], &sprite);
    }

    // Compute vertices for selection highlights
    let mut selection_count = 0;
    {
        for i in &state.selection {
            match state.entities[*i].behaviour {
                Behaviour::Cop { rounds_in_magazine, .. } => { _magazine_count.push(rounds_in_magazine) },
                _ => ()
            };
            let entity = &state.entities[*i];
            let sprite = Sprite {
                position: entity.position,
                facing: entity.get_facing_normal(),
                radius: 0.5,
            };
            push_sprite_vertices(&mut vertex_buffers[SpriteType::SelectionHighlight], &sprite);

            // add more selection GUI to right
            selection_count += 1;
        }
    }

    // Computer vertices for GUI

    //let offset = 0.1;
    for component in &mut ui.components {
        match &component.id {
            GuiType::ZombieUI => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::ZombieWorldIcon], component);
            },
            GuiType::CivilianUI => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CivilianWorldIcon], component);
            },
            GuiType::CopUI => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CopWorldIcon], component);
            },
            GuiType::Selected => {
                if selection_count < 1 {

                } else {
                    // might be useful later...
                    // component.move_pos(Vector2 { x: offset * ((0) as f64), y: 0.0 });

                    // draw the cop UI icon (is used to show the selected cops)
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CopIcon], component);
                }
            },
            GuiType::SelectionDrag => {
                if control.mouse_drag {
                    let rec_min_x = control.drag_vertex_start.x.min(control.drag_vertex_end.x);
                    let rec_min_y = control.drag_vertex_start.y.min(control.drag_vertex_end.y);
                    let rec_max_x = control.drag_vertex_start.x.max(control.drag_vertex_end.x);
                    let rec_max_y = control.drag_vertex_start.y.max(control.drag_vertex_end.y);
                    component.set_dimension(Vector2 { x: rec_min_x, y: rec_min_y },
                                            Vector2 { x: rec_min_x, y: rec_max_y },
                                            Vector2 { x: rec_max_x, y: rec_min_y },
                                            Vector2 { x: rec_max_x, y: rec_max_y });
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::SelectionHighlight], component);
                }
            }
            GuiType::Score => (),
            GuiType::Timer => (),
            GuiType::Window => (),
            GuiType::Menu{_window_gui, _buttons_gui, ..} => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Menu], component);
                if ui.active_window == ActiveWindow::Menu {
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::MenuWindow], _window_gui);
                    let size = _buttons_gui.len();
                    for i in 0..size {
                        let button_dimensions = _buttons_gui[i].get_dimension();
                        text_buffers.push(_buttons_gui[i].clone());

                        _menu_buttons.push(button_dimensions);
                        push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], &_buttons_gui[i]);
                    }
                } else if ui.active_window == ActiveWindow::Instruction {
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::MenuWindow], _window_gui);
                }
            },
            _ => (),
        };
    }

    //  push_gui_vertices(&mut vertex_buffers_gui[SpriteType::SelectionHighlight], component);

    // Compute vertices for buildings
    for building in &state.buildings {
        let color = [0.1, 0.1, 0.1, 1.0];
        push_building_vertices(&mut vertex_buffers_building, building, color);
    }

    // Render buildings
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


    // Render shadows
    use crate::presentation::display::SpriteType::*;
    for sprite_type in &[Cop, Civilian, Dead, Zombie] {

        let uniforms = uniform! {
            matrix: camera_frame,
            tex: &textures.sprite_textures[*sprite_type],
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
            tex: &textures.sprite_textures[sprite_type],
        };
        draw_sprites(
            frame,
            window,
            &vertex_buffer,
            &programs.sprite_program,
            params,
            &uniforms);
    }

    // Render GUI
    let mat_gui = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];
    for (_gui_type, vertex_buffer) in &vertex_buffers_gui {
        if _gui_type == SpriteType::Cop {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[SpriteType::Cop],
                };
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        } else if _gui_type == SpriteType::SelectionHighlight {
            let uniforms = uniform! {
                    matrix: mat_gui,
                };
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.gui_program,
                params,
                &uniforms);
        } else if _gui_type == SpriteType::Menu {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }
        else if _gui_type == SpriteType::MenuWindow {
            if ui.active_window == ActiveWindow::Menu {
                let uniforms = uniform! {
                        matrix: mat_gui,
                    };
                draw_color_sprites(
                    frame,
                    window,
                    &vertex_buffer,
                    &programs.gui_program,
                    params,
                    &uniforms);
            } else if ui.active_window == ActiveWindow::Instruction {
                let uniforms = uniform! {
                        matrix: mat_gui,
                    };
                draw_color_sprites(
                    frame,
                    window,
                    &vertex_buffer,
                    &programs.gui_program,
                    params,
                    &uniforms);
                let uniforms = uniform! {
                    matrix: mat_gui,
                        tex: &textures.sprite_textures[SpriteType::InstructionMenu]
                    };
                draw_color_sprites(
                    frame,
                    window,
                    &vertex_buffer,
                    &programs.sprite_program,
                    params,
                    &uniforms);
            }
        } else if _gui_type == SpriteType::Button {
            let uniforms = uniform! {
                    matrix: mat_gui,
                };
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.gui_program,
                params,
                &uniforms);
        }
        else if _gui_type == SpriteType::CopIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_cop_num(window, selection_count,frame, &font);
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }
          else if _gui_type == SpriteType::ZombieWorldIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_remaining_zombie_num(window, zombie_count, frame, &font);
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }
          else if _gui_type == SpriteType::CivilianWorldIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_remaining_civilian_num(window, human_count, frame, &font);
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }
          else if _gui_type == SpriteType::CopWorldIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_remaining_cop_num(window, cop_count, frame, &font);
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }



    }

    // Render Menu Text
    for i in 0..text_buffers.len() {
        let system = glium_text::TextSystem::new(window);
        let mut text_to_display = "".to_string();
        let button = text_buffers[i].clone();
        match button.id.clone() {
            GuiType::Button { text } => {
                text_to_display = text;
            }
            _ => ()
        }
//        println!("{:?}", button.id);
//        println!("button dimension: {:?}", button.get_dimension());
        let text_display = format!("{}", text_to_display);
//        println!("{:?}", text_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font, str_slice);
        let color = [1.0, 1.0, 1.0, 1.0f32];
        let text_width=text.get_width();
        let text_height = 0.07;
        let dimensions = _menu_buttons[i];
        let button_width = (dimensions.1.x - dimensions.0.x) as f32;
        let x_align = (dimensions.0.x) as f32;
        let y_align = (dimensions.0.y) as f32;

        let matrix = [
            [button_width / text_width , 0.0, 0.0, 0.0],
            [0.0, text_height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_align, y_align - 0.05, 0.0, 1.0f32],
        ];
        glium_text::draw(&text, &system, frame, matrix, color);
    }

}

// Draw the selected cop number next to the UI Cop Icon
fn draw_cop_num(window: &glium_sdl2::SDL2Facade, cop_num: i32, frame: &mut glium::Frame, font: &FontTexture){
    let system = glium_text::TextSystem::new(window);
    let cop_num_str: String = cop_num.to_string();
    let cop_num_display = format!("x{}", cop_num_str);
    let str_slice: &str = &cop_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.05, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();
    let matrix = [
        [1.0/font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down,0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-0.88, -0.88, 0.0, 1.0f32],
    ];

    if cop_num>0 {
        glium_text::draw(&text, &system, frame, matrix, color);
    }

}


// Draw the remaining number of zombies in the world (number)
fn draw_remaining_zombie_num(window: &glium_sdl2::SDL2Facade, zombie_num: i32, frame: &mut glium::Frame, font: &FontTexture){
    let system = glium_text::TextSystem::new(window);
    let zombie_num_str: String =  zombie_num.to_string();
    let mut zombie_num_display = format!("0{}",  zombie_num_str);
    if zombie_num > 99{
        zombie_num_display = format!("{}",  zombie_num_str);
    }
    let str_slice: &str = &zombie_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();

    let matrix = [
        [1.0/font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down,0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.8755, 0.83, 0.0, 1.0f32],
    ];

    glium_text::draw(&text, &system, frame, matrix, color);

}

// Draw the remaining number of civilians in the world (number)
fn draw_remaining_civilian_num(window: &glium_sdl2::SDL2Facade, civilian_num: i32, frame: &mut glium::Frame, font: &FontTexture){
    let system = glium_text::TextSystem::new(window);
    let civilian_num_str: String =  civilian_num.to_string();
    let mut civilian_num_display = format!("0{}",  civilian_num_str);
    if civilian_num > 99 {
        civilian_num_display = format!("{}",  civilian_num_str);
    }
    let str_slice: &str = &civilian_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();

    let matrix = [
        [1.0/font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down,0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.7255, 0.83, 0.0, 1.0f32],
    ];

    glium_text::draw(&text, &system, frame, matrix, color);

}

// Draw the remaining number of civilians in the world (number)
fn draw_remaining_cop_num(window: &glium_sdl2::SDL2Facade, cop_num: i32, frame: &mut glium::Frame, font: &FontTexture){
    let system = glium_text::TextSystem::new(window);
    let cop_num_str: String =  cop_num.to_string();
    let mut cop_num_display = format!("0{}",  cop_num_str);
    if cop_num > 99 {
        cop_num_display = format!("{}",  cop_num_str);
    }
    else if cop_num < 10 {
        cop_num_display = format!("00{}",  cop_num_str);
    }
    let str_slice: &str = &cop_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();

    let matrix = [
        [1.0/font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down,0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.5755, 0.83, 0.0, 1.0f32],
    ];

    glium_text::draw(&text, &system, frame, matrix, color);

}

pub fn display_main_menu (
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    programs: &Programs,
    textures: &Textures,
    params: &glium::DrawParameters,
    ui: &mut Component,
    font: &FontTexture,
) {

    frame.clear_color(0.160, 0.160, 0.160, 1.0);

    let _camera_frame =[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];

//    draw_background(frame, window, textures, programs, camera_frame, params);

    let mut vertex_buffers_gui = enum_map!{_ => vec!()};
    let mut text_buffers = vec!();
    let mut _menu_buttons: Vec<(Vector2, Vector2, Vector2, Vector2)> = vec![];

    // Compute vertices for GUI
    for component in &mut ui.components {
        match &component.id {
            GuiType::Button {..} => {
                if ui.active_window == ActiveWindow::MainMenu {
                    let button_dimensions = component.get_dimension();
                    text_buffers.push(Box::new(component.clone()));
                    _menu_buttons.push(button_dimensions);
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], component);
                }
            }
            GuiType::Score => (),
            GuiType::Timer => (),
            GuiType::Window => (),
            GuiType::Menu{_window_gui, _buttons_gui, ..} => {
                if ui.active_window == ActiveWindow::MainMenu {
                    let button_dimensions = component.get_dimension();
                    text_buffers.push(Box::new(component.clone()));
                    _menu_buttons.push(button_dimensions);
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], component);
                }
                if ui.active_window == ActiveWindow::Menu {
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::MenuWindow], _window_gui);
                    let size = _buttons_gui.len();
                    for i in 0..size {
                        let button_dimensions = _buttons_gui[i].get_dimension();
                        text_buffers.push(_buttons_gui[i].clone());

                        _menu_buttons.push(button_dimensions);
                        push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], &_buttons_gui[i]);
                    }
                } else if ui.active_window == ActiveWindow::Instruction {
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::MenuWindow], _window_gui);
                }
            },
            _ => (),
        };
    }

    // Render GUI
    let mat_gui = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32],
    ];
    for (_gui_type, vertex_buffer) in &vertex_buffers_gui {
        if _gui_type == SpriteType::MenuWindow {
            if ui.active_window == ActiveWindow::Menu {
                let uniforms = uniform! {
                        matrix: mat_gui,
                    };
                draw_color_sprites(
                    frame,
                    window,
                    &vertex_buffer,
                    &programs.gui_program,
                    params,
                    &uniforms);
            } else if ui.active_window == ActiveWindow::Instruction {
                let uniforms = uniform! {
                        matrix: mat_gui,
                    };
                draw_color_sprites(
                    frame,
                    window,
                    &vertex_buffer,
                    &programs.gui_program,
                    params,
                    &uniforms);
                let uniforms = uniform! {
                    matrix: mat_gui,
                        tex: &textures.sprite_textures[SpriteType::InstructionMenu]
                    };
                draw_color_sprites(
                    frame,
                    window,
                    &vertex_buffer,
                    &programs.sprite_program,
                    params,
                    &uniforms);
            }
        } else if _gui_type == SpriteType::Button {
            let uniforms = uniform! {
                    matrix: mat_gui,
                };
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.gui_program,
                params,
                &uniforms);
        }
    }


    // Render Menu Text
    for i in 0..text_buffers.len() {
        let system = glium_text::TextSystem::new(window);
        let mut text_to_display = "".to_string();
        let button = text_buffers[i].clone();
        match button.id.clone() {
            GuiType::Button { text } => {
                text_to_display = text;
            }
            GuiType::Menu {text,..} => {
                text_to_display = text;
            }
            _ => ()
        }
        let text_display = format!("{}", text_to_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font, str_slice);
        let color = [1.0, 1.0, 1.0, 1.0f32];
        let text_width=text.get_width();
        let text_height = 0.07;
        let dimensions = _menu_buttons[i];
        let button_width = (dimensions.1.x - dimensions.0.x) as f32;
        let x_align = (dimensions.0.x) as f32;
        let y_align = (dimensions.0.y) as f32;

        let matrix = [
            [button_width / text_width , 0.0, 0.0, 0.0],
            [0.0, text_height, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x_align, y_align - 0.05, 0.0, 1.0f32],
        ];
        glium_text::draw(&text, &system, frame, matrix, color);
    }

    // Render Title
    if ui.active_window == ActiveWindow::MainMenu {
        let system = glium_text::TextSystem::new(window);
        let text_display = "Contagion (placeholder)".to_string();
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font, str_slice);
        let color = [1.0, 1.0, 0.0, 1.0f32];
        let _font_scale_down = 1.5;
        let text_width = text.get_width();
        let (w, h) = frame.get_dimensions();
        let _text_offset = 1.0 / text_width;
        let matrix = [
            [2.0 / text_width, 0.0, 0.0, 0.0],
            [0.0, 2.0 * (w as f32) / (h as f32) / text_width, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [-1.0, 0.2, 0.0, 1.0f32],
        ];
        glium_text::draw(&text, &system, frame, matrix, color);
    }
}
