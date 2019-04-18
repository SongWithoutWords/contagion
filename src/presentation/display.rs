use enum_map::EnumMap;
use glium::Surface;
use glium::texture::texture2d::Texture2d;
use lerp::*;

use crate::core::geo::polygon::*;
use crate::core::matrix::*;
use crate::core::scalar::*;
use crate::core::vector::*;
use crate::presentation::graphics::font::FontPkg;
use crate::presentation::ui::glium_text;
use crate::presentation::ui::glium_text::FontTexture;
use crate::presentation::ui::gui::*;
use crate::presentation::ui::gui::Component;
use crate::presentation::ui::gui::GuiType;
use crate::simulation::control::*;
use crate::simulation::state::*;
use crate::simulation::update::EntityCounts;
use crate::simulation::game_state::GameState;

// Enum ordered by draw order
#[derive(Copy, Clone, Debug, Enum, PartialEq)]
pub enum SpriteType {
    SelectionHighlight,
    Dead,
    BulletCasing,
    Civilian,
    Zombie,
    Cop,
    Soldier,
    BulletInAir,
    Menu,
    MenuWindow,
    Button,
    InstructionMenu,
    CopIcon,
    ZombieWorldIcon,
    CopWorldIcon,
    CivilianWorldIcon,
    BuildingOne,
    ZombieFist,
    ZombieTorso,
    ZombieClawRight,
    ZombieClawLeft,
    ZombieIconHighlight,
    CopIconHighlight,
    CivilianIconHighlight,
}

// pub type Textures = EnumMap<SpriteType, Texture2d>;
pub struct Textures {
    sprite_textures: EnumMap<SpriteType, Texture2d>,
    background_texture: Texture2d,
    outside_border_texture: Texture2d,
    left_fence_texture: Texture2d,
    top_fence_texture: Texture2d,
    wallpaper: Texture2d,
    victory: Texture2d,
    loss: Texture2d,
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
            SpriteType::Soldier
                => load_texture(window, "assets/images/old/soldier.png"),
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
                => load_texture(window, "assets/images/ui/zombie_world_icon_new.png"),
            SpriteType::CopWorldIcon
                => load_texture(window, "assets/images/ui/cop_world_icon_new.png"),
            SpriteType::CivilianWorldIcon
                => load_texture(window, "assets/images/ui/civilian_world_icon_new.png"),
            SpriteType::BuildingOne
                => load_texture(window, "assets/images/building/building_one.png"),
            SpriteType::ZombieFist
                => load_texture(window, "assets/images/zombie/zombie_claw_right.png"),
            SpriteType::ZombieTorso
                => load_texture(window, "assets/images/zombie/zombie_torso.png"),
            SpriteType::ZombieClawRight
                => load_texture(window, "assets/images/zombie/zombie_claw_right.png"),
            SpriteType::ZombieClawLeft
                => load_texture(window, "assets/images/zombie/zombie_claw_left.png"),
            SpriteType::ZombieIconHighlight
                => load_texture(window, "assets/images/ui/zombie_highlight.png"),
            SpriteType::CopIconHighlight
                => load_texture(window, "assets/images/ui/cop_highlight.png"),
            SpriteType::CivilianIconHighlight
                => load_texture(window, "assets/images/ui/civilian_highlight.png"),
        },
        background_texture: load_texture(&window, "assets/images/dirt.jpg"),
        wallpaper: load_texture(&window, "assets/images/contagion_wallpaper.png"),
        victory: load_texture(&window, "assets/images/homer.png"),
        loss: load_texture(&window, "assets/images/lisa.png"),
        outside_border_texture: load_texture(&window, "assets/images/grass.jpg"),
        left_fence_texture: load_texture(&window, "assets/images/wall_left.jpg"),
        top_fence_texture: load_texture(&window, "assets/images/wall.jpg"),
    }
}

pub struct Programs {
    background_program: glium::Program,
    sprite_program: glium::Program,
    shadow_program: glium::Program,
    gui_program: glium::Program,
    shape_program: glium::Program,
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

    let top_left = position - right + up;
    let top_right = position + right + up;
    let bot_left = position - right - up;
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
        tex_coords: [0.0, 1.0],
    };
    let vertex1 = Vertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0],
    };
    let vertex2 = Vertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0],
    };
    let vertex3 = Vertex {
        position: bot_right.as_f32_array(),
        tex_coords: [1.0, 0.0],
    };
    buffer.push(vertex0);
    buffer.push(vertex1);
    buffer.push(vertex2);
    buffer.push(vertex1);
    buffer.push(vertex3);
    buffer.push(vertex2);
}

fn push_gui_vertices(buffer: &mut Vec<ColorVertex>, ui: &Gui) {
    let top_left = ui.top_left;
    let top_right = ui.top_right;
    let bot_left = ui.bot_left;
    let bot_right = ui.bot_right;
    let mut color = [0.0, 0.0, 0.0, 1.0];

    match ui.id {
        GuiType::SelectionDrag => { color = [0.105, 0.214, 0.124, 0.3] }
        GuiType::Button { .. } => { color = [0.6, 0.7, 0.8, 0.0] }
        GuiType::Window => { color = [0.0, 0.0, 0.0, 0.7] }
        GuiType::Menu { .. } => { color = [0.6, 0.7, 0.8, 0.0] }
        _ => (),
    };

    let vertex0 = ColorVertex {
        position: top_left.as_f32_array(),
        tex_coords: [0.0, 1.0],
        color,
    };
    let vertex1 = ColorVertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0],
        color,
    };
    let vertex2 = ColorVertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0],
        color,
    };
    let vertex3 = ColorVertex {
        position: bot_right.as_f32_array(),
        tex_coords: [1.0, 0.0],
        color,
    };
    buffer.push(vertex0);
    buffer.push(vertex1);
    buffer.push(vertex2);
    buffer.push(vertex1);
    buffer.push(vertex3);
    buffer.push(vertex2);
}

// right if hand is true, left if hand is false
fn push_hand_vertices(buffer: &mut Vec<Vertex>, sprite: &Sprite, hand: bool) {
    let position = sprite.position;
    let up = sprite.radius * sprite.facing;
    let right = up.right(); //vector2(up.y, -up.x);

    let mut top_left = position - right + up;
    let mut top_right = position + up;
    let mut bot_left = position - right;
    let mut bot_right = position;

    if hand {
        top_left = position + up;
        top_right = position + right + up;
        bot_left = position;
        bot_right = position + right;
    }

    // 0      1
    // +------+
    // |    / |
    // |  /   |
    // |/     |
    // +------+
    // 2      3

    let vertex0 = Vertex {
        position: top_left.as_f32_array(),
        tex_coords: [0.0, 1.0],
    };
    let vertex1 = Vertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0],
    };
    let vertex2 = Vertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0],
    };
    let vertex3 = Vertex {
        position: bot_right.as_f32_array(),
        tex_coords: [1.0, 0.0],
    };
    buffer.push(vertex0);
    buffer.push(vertex1);
    buffer.push(vertex2);
    buffer.push(vertex1);
    buffer.push(vertex3);
    buffer.push(vertex2);
}

fn push_health_bar_vertices(buffer: &mut Vec<ColorVertex>, sprite: &Sprite, health: Scalar) {
    let position = Vector2 { x: sprite.position.x, y: sprite.position.y + 1.0 };
    let up = 0.5;
    let down = 0.2;

    let top_left = Vector2 { x: position.x - up, y: position.y + up };
    let top_right = top_left.lerp(Vector2 { x: position.x + up, y: position.y + up }, health);
    let bot_left = Vector2 { x: position.x - up, y: position.y + down };
    let bot_right = bot_left.lerp(Vector2 { x: position.x + up, y: position.y + down }, health);

    let color = vector4(1.0, 0.0, 0.0, 1.0).lerp(vector4(0.0, 1.0, 0.0, 1.0), health).as_f32_array();

    let vertex0 = ColorVertex {
        position: top_left.as_f32_array(),
        tex_coords: [0.0, 1.0],
        color,
    };
    let vertex1 = ColorVertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0],
        color,
    };
    let vertex2 = ColorVertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0],
        color,
    };
    let vertex3 = ColorVertex {
        position: bot_right.as_f32_array(),
        tex_coords: [1.0, 0.0],
        color,
    };
    buffer.push(vertex0);
    buffer.push(vertex1);
    buffer.push(vertex2);
    buffer.push(vertex1);
    buffer.push(vertex3);
    buffer.push(vertex2);
}

fn push_building_vertices(buffer: &mut Vec<ColorVertex>, building: &Polygon, color: [f32; 4]) {
    let bounds = building.bounding_box();
    let width = bounds.1.x - bounds.0.x;
    let height = bounds.1.y - bounds.0.y;

    for triangle in building.triangles() {
        for point in triangle.0 {
            buffer.push(ColorVertex {
                position: point.as_f32_array(),
                tex_coords: [
                    ((point.x - bounds.0.x) / width) as f32,
                    ((point.y - bounds.0.y) / height) as f32
                ],
                color,
            })
        }
    }
}

fn push_path_vertices(buffer: &mut Vec<ColorVertex>, point1: Vector2, point2: Vector2, color: [f32; 4]) {
    let lambda = 0.03;

    // case 1, point1 is bottom left and point2 is top right or vice versa
    let mut top_left = Vector2 { x: point1.x - lambda, y: point1.y + lambda };
    let mut top_right = Vector2 { x: point2.x - lambda, y: point2.y + lambda };
    let mut bot_right = Vector2 { x: point2.x + lambda, y: point2.y - lambda };
    let mut bot_left = Vector2 { x: point1.x + lambda, y: point1.y - lambda };

    // case 2, point1 is top left and point2 is bottom right or vice versa
    if (point1.x <= point2.x && point1.y >= point2.y) || (point2.x <= point1.x && point2.y >= point1.y) {
        top_left = Vector2 { x: point1.x - lambda, y: point1.y - lambda };
        top_right = Vector2 { x: point1.x + lambda, y: point1.y + lambda };
        bot_right = Vector2 { x: point2.x - lambda, y: point2.y - lambda };
        bot_left = Vector2 { x: point2.x + lambda, y: point2.y + lambda };
    }

    let vertex0 = ColorVertex {
        position: top_left.as_f32_array(),
        tex_coords: [0.0, 1.0],
        color,
    };
    let vertex1 = ColorVertex {
        position: top_right.as_f32_array(),
        tex_coords: [1.0, 1.0],
        color,
    };
    let vertex2 = ColorVertex {
        position: bot_left.as_f32_array(),
        tex_coords: [0.0, 0.0],
        color,
    };
    let vertex3 = ColorVertex {
        position: bot_right.as_f32_array(),
        tex_coords: [1.0, 0.0],
        color,
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

fn draw_outside_background(
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
        position: [-extent, extent],
    };
    let top_right = VertexPosition {
        position: [extent, extent],
    };
    let bot_left = VertexPosition {
        position: [-extent, -extent],
    };
    let bot_right = VertexPosition {
        position: [extent, -extent],
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
        tex: &textures.outside_border_texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.background_program,
        &uniforms,
        params).unwrap();
}

fn draw_left_fence(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    let top_left = VertexPosition {
        position: [-24.5, 116.0],
    };
    let top_right = VertexPosition {
        position: [-26.0, 116.0],
    };
    let bot_left = VertexPosition {
        position: [-24.5, -26.0],
    };
    let bot_right = VertexPosition {
        position: [-26.0, -26.0],
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
        tex: &textures.left_fence_texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.background_program,
        &uniforms,
        params).unwrap();
}

fn draw_right_fence(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    let top_left = VertexPosition {
        position: [114.5, 116.0],
    };
    let top_right = VertexPosition {
        position: [116.0, 116.0],
    };
    let bot_left = VertexPosition {
        position: [114.5, -26.0],
    };
    let bot_right = VertexPosition {
        position: [116.0, -26.0],
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
        tex: &textures.left_fence_texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.background_program,
        &uniforms,
        params).unwrap();
}

fn draw_top_fence(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    let top_left = VertexPosition {
        position: [-26.0, 116.0],
    };
    let top_right = VertexPosition {
        position: [116.0, 116.0],
    };
    let bot_left = VertexPosition {
        position: [-26.0, 114.5],
    };
    let bot_right = VertexPosition {
        position: [116.0, 114.5],
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
        tex: &textures.top_fence_texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.background_program,
        &uniforms,
        params).unwrap();
}

fn draw_lower_fence(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    let top_left = VertexPosition {
        position: [-26.0, -24.5],
    };
    let top_right = VertexPosition {
        position: [116.0, -24.5],
    };
    let bot_left = VertexPosition {
        position: [-26.0, -26.0],
    };
    let bot_right = VertexPosition {
        position: [116.0, -26.0],
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
        tex: &textures.top_fence_texture,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.background_program,
        &uniforms,
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
    // let extent = 1e2;

    let top_left = VertexPosition {
        position: [-25.0, 115.0],
    };
    let top_right = VertexPosition {
        position: [115.0, 115.0],
    };
    let bot_left = VertexPosition {
        position: [-25.0, -25.0],
    };
    let bot_right = VertexPosition {
        position: [115.0, -25.0],
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


fn draw_main_menu_background(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    // This is about as large as you can get without introducing artifacts
    // due to floating point imprecision
    // let extent = 1e2
    let top_left = Vertex {
        position: [-11.3, 11.3],
        tex_coords: [0.0, 1.0],
    };
    let top_right = Vertex {
        position: [11.3, 11.3],
        tex_coords: [1.0, 1.0],
    };
    let bot_left = Vertex {
        position: [-11.3, -11.3],
        tex_coords: [0.0, 0.0],
    };
    let bot_right = Vertex {
        position: [11.3, -11.3],
        tex_coords: [1.0, 0.0],
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
        tex: &textures.wallpaper,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.sprite_program,
        &uniforms,
        params).unwrap();
}

fn draw_loss_menu_background(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    // This is about as large as you can get without introducing artifacts
    // due to floating point imprecision
    // let extent = 1e2
    let top_left = Vertex {
        position: [-11.3, 11.3],
        tex_coords: [0.0, 1.0],
    };
    let top_right = Vertex {
        position: [11.3, 11.3],
        tex_coords: [1.0, 1.0],
    };
    let bot_left = Vertex {
        position: [-11.3, -11.3],
        tex_coords: [0.0, 0.0],
    };
    let bot_right = Vertex {
        position: [11.3, -11.3],
        tex_coords: [1.0, 0.0],
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
        tex: &textures.loss,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.sprite_program,
        &uniforms,
        params).unwrap();
}

fn draw_victory_menu_background(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    textures: &Textures,
    programs: &Programs,
    camera_frame: [[f32; 4]; 4],
    params: &glium::DrawParameters)
{
    // This is about as large as you can get without introducing artifacts
    // due to floating point imprecision
    // let extent = 1e2
    let top_left = Vertex {
        position: [-11.3, 11.3],
        tex_coords: [0.0, 1.0],
    };
    let top_right = Vertex {
        position: [11.3, 11.3],
        tex_coords: [1.0, 1.0],
    };
    let bot_left = Vertex {
        position: [-11.3, -11.3],
        tex_coords: [0.0, 0.0],
    };
    let bot_right = Vertex {
        position: [11.3, -11.3],
        tex_coords: [1.0, 0.0],
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
        tex: &textures.victory,
    };
    frame.draw(
        &glium::VertexBuffer::new(window, &vertices).unwrap(),
        &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
        &programs.sprite_program,
        &uniforms,
        params).unwrap();
}

pub fn display(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    programs: &Programs,
    textures: &Textures,
    params: &glium::DrawParameters,
    state: &State,
    entity_counts: &EntityCounts,
    camera_frame: Mat4,
    ui: &mut Component,
    fonts: &FontPkg,
    control: &Control,
    gamestate: &mut GameState,
) {
    let font = fonts.get("Consola").unwrap();

    frame.clear_color(0.2, 0.2, 0.2, 1.0);

    let camera_frame = camera_frame.as_f32_array();
    draw_outside_background(frame, window, textures, programs, camera_frame, params);
    draw_background(frame, window, textures, programs, camera_frame, params);


    let mut vertex_buffers = enum_map! {_ => vec!()};
    let mut vertex_buffers_green_hp = vec!();
    let mut vertex_buffers_gui = enum_map! {_ => vec!()};
    let mut vertex_buffers_building = vec!();
    let mut vertex_buffers_path = vec!();
    let mut text_buffers = vec!();

    let mut _menu_buttons: Vec<(Vector2, Vector2, Vector2, Vector2)> = vec![];

    // Compute the vertices in world coordinates of all projectiles
    for p in &state.projectiles {
        let sprite_type = match p.kind {
            ProjectileKind::Bullet => SpriteType::BulletInAir,
            ProjectileKind::Casing => SpriteType::BulletCasing,
            ProjectileKind::Fist { .. } => SpriteType::ZombieFist
        };
        let mut rad = BULLET_RADIUS;
        match p.kind {
            ProjectileKind::Fist { .. } => {
                rad = FIST_RADIUS;
            }
            _ => ()
        }
        let sprite = Sprite {
            position: p.position,
            facing: p.velocity.normalize(),
            radius: rad,
        };
        push_sprite_vertices(&mut vertex_buffers[sprite_type], &sprite);
    }

    // Compute the vertices in world coordinates of all entities
    for entity in &state.entities {
        let sprite = Sprite {
            position: entity.position,
            facing: entity.get_facing_normal(),
            radius: 0.5,
        };
        let sprite_type = match &entity.dead_or_alive {
            DeadOrAlive::Dead => SpriteType::Dead,
            DeadOrAlive::Alive { zombie_or_human, .. } => match zombie_or_human {
                ZombieOrHuman::Zombie { state: _, left_hand_status, right_hand_status } => {
                    match left_hand_status {
                        HandStatus::Normal => {
                            push_hand_vertices(&mut vertex_buffers[SpriteType::ZombieClawLeft], &sprite, false);
                        }
                        _ => ()
                    }
                    match right_hand_status {
                        HandStatus::Normal => {
                            push_hand_vertices(&mut vertex_buffers[SpriteType::ZombieClawRight], &sprite, true);
                        }
                        _ => ()
                    }
                    SpriteType::ZombieTorso
                }
                ZombieOrHuman::Human { human, .. } => match human {
                    Human::Cop { cop_type, .. } => match cop_type {
                        CopType::Normal => SpriteType::Cop,
                        CopType::Soldier => SpriteType::Soldier,
                    },
                    Human::Civilian { .. } => SpriteType::Civilian
                }
            }
        };
        push_sprite_vertices(&mut vertex_buffers[sprite_type], &sprite);

        let health = match &entity.dead_or_alive {
            DeadOrAlive::Alive { health, .. } => *health,
            _ => ENTITY_HEALTH_MIN
        };

        // Display a health bar only for entities that are wounded but not dead
        if ENTITY_HEALTH_MIN < health && health < ENTITY_HEALTH_MAX {
            push_health_bar_vertices(&mut vertex_buffers_green_hp, &sprite, health);
        }
    }

    // Compute vertices for selection highlights
    let mut selection_count = 0;
    {
        for i in &state.selection {

            // TODO: Ian M - I think there's a bug where cops that become infected maintain their selection highlight

            let entity = &state.entities[*i];
            let sprite = Sprite {
                position: entity.position,
                facing: entity.get_facing_normal(),
                radius: 0.5,
            };
            // If tutorial mode is on and tutorial for selecting flag is true
            if gamestate.tutorial && gamestate.tut_01 {
                gamestate.tut_passed = true;
            }
            push_sprite_vertices(&mut vertex_buffers[SpriteType::SelectionHighlight], &sprite);

            // add more selection GUI to right
            selection_count += 1;
        }
    }

    // Computer vertices for GUI

    //let offset = 0.1;
    for component in &mut ui.components {
        match &component.id {

            GuiType::ZombieHighlight => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::ZombieIconHighlight], component);
            }
            GuiType::CopHighlight => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CopIconHighlight], component);
            }
            GuiType::CivilianHighlight => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CivilianIconHighlight], component);
            }
            GuiType::ZombieUI => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::ZombieWorldIcon], component);
            }
            GuiType::CivilianUI => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CivilianWorldIcon], component);
            }
            GuiType::CopUI => {
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CopWorldIcon], component);
            }
            GuiType::Selected => {
                if selection_count < 1 {} else {
                    // might be useful later...
                    // component.move_pos(Vector2 { x: offset * ((0) as f64), y: 0.0 });

                    // draw the cop UI icon (is used to show the selected cops)
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::CopIcon], component);
                }
            }
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
            GuiType::Menu { _window_gui, _buttons_gui, .. } => {
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
            }
            _ => (),
        };
    }

    //  push_gui_vertices(&mut vertex_buffers_gui[SpriteType::SelectionHighlight], component);

    // Compute vertices for buildings
    for building in &state.buildings {
        let color = [0.1, 0.1, 0.1, 1.0];
        push_building_vertices(&mut vertex_buffers_building, building, color);
    }

    // Compute vertices for cop paths
    for entity in &state.entities {
        match &entity.dead_or_alive {
            DeadOrAlive::Alive {
                zombie_or_human: ZombieOrHuman::Human {
                    human: Human::Cop { state_stack, .. },
                    ..
                },
                ..
            } => {
                let path = match state_stack.last() {
                    Some(CopState::Moving { path, .. }) => path,
                    Some(CopState::AttackingZombie { path, .. }) => path,
                    _ => &None
                };
                match path {
                    Some(path) => {
                        let path_vec = path.to_vec();
                        let color = [0.0, 0.0, 0.0, 1.0];
                        for i in 0..(path_vec.len() - 1) {
                            push_path_vertices(&mut vertex_buffers_path, path_vec[i], path_vec[i + 1], color);
                        }
                        // Tutorial purposes: right clicking
                        // if tutorial is enabled and tutorial 1 is over and tutorial 2 is enabled
                        if gamestate.tutorial && !gamestate.tut_01 && gamestate.tut_02 {
                            gamestate.tut_passed = true;
                        }
                    }
                    None => ()
                }
            }
            _ => ()
        };
    }


    // Render paths
    {
        let uniforms = uniform! {
            matrix: camera_frame
        };
        draw_color_sprites(
            frame,
            window,
            &vertex_buffers_path,
            &programs.shape_program,
            params,
            &uniforms);
    }

    // Render buildings
    {
        let uniforms = uniform! {
            matrix: camera_frame,
            tex: &textures.sprite_textures[SpriteType::BuildingOne],
        };
        draw_color_sprites(
            frame,
            window,
            &vertex_buffers_building,
            &programs.sprite_program,
            params,
            &uniforms);
    }
    // Draw Fence border textures
    draw_left_fence(frame, window, textures, programs, camera_frame, params);
    draw_right_fence(frame, window, textures, programs, camera_frame, params);
    draw_top_fence(frame, window, textures, programs, camera_frame, params);
    draw_lower_fence(frame, window, textures, programs, camera_frame, params);


    // Render shadows
    use crate::presentation::display::SpriteType::*;
    for sprite_type in &[Cop, Civilian, Dead, ZombieTorso, ZombieClawRight, ZombieClawLeft] {
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

    // Render green HP
    let uniforms = uniform! {
        matrix: camera_frame,
    };
    draw_color_sprites(
        frame,
        window,
        &vertex_buffers_green_hp,
        &programs.gui_program,
        params,
        &uniforms);

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
        } else if _gui_type == SpriteType::MenuWindow {
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
        } else if _gui_type == SpriteType::CopIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_cop_num(window, selection_count, frame, &font.medres());
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }
        else if _gui_type == SpriteType::CopIconHighlight {
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
        else if _gui_type == SpriteType::ZombieIconHighlight {
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
       else if _gui_type == SpriteType::CivilianIconHighlight {
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

        else if _gui_type == SpriteType::ZombieWorldIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_remaining_zombie_num(
                window,
                entity_counts.zombies,
                frame,
                &font.lowres());

            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        } else if _gui_type == SpriteType::CivilianWorldIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };
            // Draw the text showing the number of cops next to the UI cop icon
            draw_remaining_civilian_num(window, entity_counts.civilians, frame, &font.lowres());
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        } else if _gui_type == SpriteType::CopWorldIcon {
            let uniforms = uniform! {
                    matrix: mat_gui,
                    tex: &textures.sprite_textures[_gui_type],
                };

            // Draw the text showing the number of cops next to the UI cop icon
            draw_remaining_cop_num(window, entity_counts.cops, frame, &font.lowres());
            draw_color_sprites(
                frame,
                window,
                &vertex_buffer,
                &programs.sprite_program,
                params,
                &uniforms);
        }
    }

    // Render Summary Text
    if gamestate.summary_text {
        let mat = Mat4::init_id_matrix();
        let system = glium_text::TextSystem::new(window);
        let text_to_display = "People are turning into zombies!";
        let text_display = format!("{}", text_to_display);
        let mut color = [1.0, 1.0, 1.0, 1.0 as f32];
        if gamestate.fade_wait > 120 {
            gamestate.fade_alpha = gamestate.fade_alpha - (1.0 / (gamestate.fade_max as f32));
            color = [1.0, 1.0, 1.0, gamestate.fade_alpha as f32];
        }
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);

        let text_width = text.get_width() as f64;
        let (w, h) = frame.get_dimensions();
        let _text_offset = 1.0 / text_width;
        let scale_factor = Vector4 { x: 1.0 / text_width, y: 1.0 * (w as f64) / (h as f64) / text_width, z: 1.0, w: 1.0 };
        let translation_offset = Vector4 { x: -0.8 , y: 0.7, z: 0.0, w: 0.0 };
        let mut matrix = mat.scale(scale_factor).translation(translation_offset);
        glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);

        let text_to_display = "Eradicate all the zombies.";
        let text_display = format!("{}", text_to_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
        let translation_offset = Vector4 {x: 0.0, y: -0.1, z: 0.0, w: 0.0};
        matrix = matrix.translation(translation_offset);
        glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);
    }

    // Render Tutorial Text
    if gamestate.tutorial {
        let mat = Mat4::init_id_matrix();
        let system = glium_text::TextSystem::new(window);
        let mut scale_width = 1.0;
        let mut x_offset = -0.5;
        let (w, h) = frame.get_dimensions();
        let color = [1.0, 1.0, 1.0, 1.0 as f32];
        // if tutorial 1 is not over
        if gamestate.tut_01 {
            let mut text_to_display = "Find and select a cop or drag to select multiple";
            if gamestate.tut_passed == true {
                text_to_display = "good!";
                scale_width = 0.2;
                x_offset = -0.1;
                if tutorial_text_fade(gamestate) {
                    gamestate.tut_01 = false;
                }
            }
            let text_display = format!("{}", text_to_display);
            let str_slice: &str = &text_display[..];
            let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
            let text_width = text.get_width() as f64;
            let text_offset = scale_width / text_width;
            let scale_factor = Vector4 { x: text_offset, y: scale_width * (w as f64) / (h as f64) / text_width, z: 1.0, w: 1.0 };
            let translation_offset = Vector4 { x: x_offset , y: -0.5, z: 0.0, w: 0.0 };
            let matrix = mat.scale(scale_factor).translation(translation_offset);
            glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);
        }
        // if tutorial 1 is over and tutorial 2 started
        if !gamestate.tut_01 && gamestate.tut_02 {
            let mut text_to_display = "Right click to move or target zombies";
            if gamestate.tut_passed == true {
                text_to_display = "Good!";
                scale_width = 0.2;
                x_offset = -0.1;
                if tutorial_text_fade(gamestate) {
                    gamestate.tut_02 = false;
                }
            }
            let text_display = format!("{}", text_to_display);
            let str_slice: &str = &text_display[..];
            let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
            let text_width = text.get_width() as f64;
            let text_offset = scale_width / text_width;
            let scale_factor = Vector4 { x: text_offset, y: scale_width * (w as f64) / (h as f64) / text_width, z: 1.0, w: 1.0 };
            let translation_offset = Vector4 { x: x_offset , y: -0.5, z: 0.0, w: 0.0 };
            let matrix = mat.scale(scale_factor).translation(translation_offset);
            glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);
        }
        // if tutorial 2 is done and tutorial 3 is enabled
        if !gamestate.tut_02 && gamestate.tut_03 {
            let mut text_to_display = "press 'space bar' to unpause and pause the game";
            if gamestate.tut_passed == true {
                text_to_display = "Good!";
                scale_width = 0.2;
                x_offset = -0.1;
                if tutorial_text_fade(gamestate) {
                    gamestate.tut_03 = false;
                }
            }
            let text_display = format!("{}", text_to_display);
            let str_slice: &str = &text_display[..];
            let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
            let text_width = text.get_width() as f64;
            let text_offset = scale_width / text_width;
            let scale_factor = Vector4 { x: text_offset, y: scale_width * (w as f64) / (h as f64) / text_width, z: 1.0, w: 1.0 };
            let translation_offset = Vector4 { x: x_offset , y: -0.5, z: 0.0, w: 0.0 };
            let matrix = mat.scale(scale_factor).translation(translation_offset);
            glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);
        }
    }

    // Render Menu Button Text
    let mat = Mat4::init_id_matrix();
    for i in 0..text_buffers.len() {
        let system = glium_text::TextSystem::new(window);
        let mut text_to_display = "".to_string();
        let button = &mut text_buffers[i];
        let mut color = [1.0, 1.0, 1.0, 1.0f32];
        match &mut button.id {
            GuiType::Button { ref mut text, highlight } => {
                text_to_display = text.clone();
                if *highlight { color = [0.1, 0.1, 0.1, 1.0f32] }
                if text_to_display == "Instruction" {}
            }
            _ => ()
        }
        let text_display = format!("{}", text_to_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
        let text_width = text.get_width() as f64;
        let text_height = 0.06;
        let dimensions = _menu_buttons[i];
        let button_width = dimensions.1.x - dimensions.0.x;
        let x_align = dimensions.0.x;
        let y_align = (dimensions.0.y) - 0.07;
        let menu_matrix = mat.translation(Vector4 { x: x_align, y: y_align, z: 0.0, w: 0.0 })
            .scale(Vector4 { x: button_width / text_width, y: text_height, z: 1.0, w: 1.0 }).as_f32_array();
        glium_text::draw(&text, &system, frame, menu_matrix, color);
    }
}

fn tutorial_text_fade(gamestate: &mut GameState) -> bool {
    if gamestate.tut_time == 0 {
        gamestate.tut_time = gamestate.tut_time_curr;
    }
    if (gamestate.tut_time_curr - gamestate.tut_time) > 60 {
        gamestate.tut_passed = false;
        gamestate.tut_time = 0;
        return true
    } else {
        return false
    }
}

// Draw the selected cop number next to the UI Cop Icon
fn draw_cop_num(window: &glium_sdl2::SDL2Facade, cop_num: i32, frame: &mut glium::Frame, font: &FontTexture) {
    let system = glium_text::TextSystem::new(window);
    let cop_num_str: String = cop_num.to_string();
    let cop_num_display = format!("x{}", cop_num_str);
    let str_slice: &str = &cop_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.05, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();
    let matrix = [
        [1.0 / font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [-0.88, -0.88, 0.0, 1.0f32],
    ];

    if cop_num > 0 {
        glium_text::draw(&text, &system, frame, matrix, color);
    }
}


// Draw the remaining number of zombies in the world (number)
fn draw_remaining_zombie_num(window: &glium_sdl2::SDL2Facade, zombie_num: usize, frame: &mut glium::Frame, font: &FontTexture) {
    let system = glium_text::TextSystem::new(window);
    let zombie_num_str: String = zombie_num.to_string();
    let mut zombie_num_display = format!("0{}", zombie_num_str);
    if zombie_num > 99 {
        zombie_num_display = format!("{}", zombie_num_str);
    } else if zombie_num < 10 {
        zombie_num_display = format!("0{}", zombie_num_display);
    }
    let str_slice: &str = &zombie_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();

    let matrix = [
        [1.0 / font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.8755, 0.83, 0.0, 1.0f32],
    ];

    glium_text::draw(&text, &system, frame, matrix, color);
}

// Draw the remaining number of civilians in the world (number)
fn draw_remaining_civilian_num(window: &glium_sdl2::SDL2Facade, civilian_num: usize, frame: &mut glium::Frame, font: &FontTexture) {
    let system = glium_text::TextSystem::new(window);
    let civilian_num_str: String = civilian_num.to_string();
    let mut civilian_num_display = format!("0{}", civilian_num_str);
    if civilian_num > 99 {
        civilian_num_display = format!("{}", civilian_num_str);
    } else if civilian_num < 10 {
        civilian_num_display = format!("00{}", civilian_num_str);
    }
    let str_slice: &str = &civilian_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();

    let matrix = [
        [1.0 / font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.7255, 0.83, 0.0, 1.0f32],
    ];

    glium_text::draw(&text, &system, frame, matrix, color);
}

// Draw the remaining number of civilians in the world (number)
fn draw_remaining_cop_num(window: &glium_sdl2::SDL2Facade, cop_num: usize, frame: &mut glium::Frame, font: &FontTexture) {
    let system = glium_text::TextSystem::new(window);
    let cop_num_str: String = cop_num.to_string();
    let mut cop_num_display = format!("0{}", cop_num_str);
    if cop_num > 99 {
        cop_num_display = format!("{}", cop_num_str);
    } else if cop_num < 10 {
        cop_num_display = format!("00{}", cop_num_str);
    }
    let str_slice: &str = &cop_num_display[..];
    let text = glium_text::TextDisplay::new(&system, font, str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let font_scale_down = 40.0;
    let (w, h) = frame.get_dimensions();

    let matrix = [
        [1.0 / font_scale_down, 0.0, 0.0, 0.0],
        [0.0, 1.0 * (w as f32) / (h as f32) / font_scale_down, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.5755, 0.83, 0.0, 1.0f32],
    ];

    glium_text::draw(&text, &system, frame, matrix, color);
}

pub fn display_main_menu(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    programs: &Programs,
    textures: &Textures,
    params: &glium::DrawParameters,
    camera_frame: [[f32; 4]; 4],
    ui: &mut Component,
    fonts: &FontPkg,
) {
    let font = fonts.get("Consola").unwrap();
    frame.clear_color(0.160, 0.160, 0.160, 1.0);

    let mat = Mat4::init_id_matrix();
    draw_main_menu_background(frame, window, textures, programs, camera_frame, params);

    let mut vertex_buffers_gui = enum_map! {_ => vec!()};
    let mut text_buffers = vec!();
    let mut _menu_buttons: Vec<(Vector2, Vector2, Vector2, Vector2)> = vec![];

    // Compute vertices for GUI
    for component in &mut ui.components {
        match &component.id {
            GuiType::Button { .. } => {
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
            GuiType::Menu { _window_gui, _buttons_gui, .. } => {
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
                        let cloned = &_buttons_gui[i];
                        text_buffers.push(cloned.clone());

                        _menu_buttons.push(button_dimensions);
                        push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], &_buttons_gui[i]);
                    }
                } else if ui.active_window == ActiveWindow::Instruction {
                    push_gui_vertices(&mut vertex_buffers_gui[SpriteType::MenuWindow], _window_gui);
                }
            }
            _ => (),
        };
    }

    // Render GUI
    let mat_gui = mat.as_f32_array();
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


    // Render Menu button Text
    for i in 0..text_buffers.len() {
        let system = glium_text::TextSystem::new(window);
        let mut text_to_display = "".to_string();
        let button = text_buffers[i].clone();
        let mut color = [1.0, 1.0, 1.0, 1.0f32];
        match button.id.clone() {
            GuiType::Button { text, highlight } => {
                text_to_display = text;
                if highlight { color = [0.1, 0.1, 0.1, 1.0f32] }
            }
            GuiType::Menu { _buttons_gui, text, highlight, .. } => {
                text_to_display = text;
                if highlight { color = [0.1, 0.1, 0.1, 1.0f32] }
            }
            _ => ()
        }
        let text_display = format!("{}", text_to_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
        let text_width = text.get_width() as f64;
        let text_height = 0.06;
        let dimensions = _menu_buttons[i];
        let button_width = dimensions.1.x - dimensions.0.x;
        let x_align = dimensions.0.x;
        let y_align = (dimensions.0.y) - 0.07;

        let menu_matrix = mat.translation(Vector4 { x: x_align, y: y_align, z: 0.0, w: 0.0 })
            .scale(Vector4 { x: button_width / text_width, y: text_height, z: 1.0, w: 1.0 }).as_f32_array();
        glium_text::draw(&text, &system, frame, menu_matrix, color);
    }
}

fn compute_score(entity_counts: &EntityCounts) -> f64 {
    (100 * (entity_counts.cops + entity_counts.civilians)) as f64
        / entity_counts.total() as f64
}

pub fn display_loss_screen(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    programs: &Programs,
    _textures: &Textures,
    params: &glium::DrawParameters,
    camera_frame: [[f32; 4]; 4],
    ui: &mut Component,
    entity_counts: &EntityCounts,
    fonts: &FontPkg,
) {
    let font = fonts.get("Consola").unwrap();
    //  frame.clear_color(0.0, 0.0, 0.0, 1.0);


    let mat = Mat4::init_id_matrix();
    draw_loss_menu_background(frame, window, _textures, programs, camera_frame, params);


    let mut vertex_buffers_gui = enum_map! {_ => vec!()};
    let mut text_buffers = vec!();
    let mut _menu_buttons: Vec<(Vector2, Vector2, Vector2, Vector2)> = vec![];

    // Compute vertices for GUI
    for component in &mut ui.components {
        match &component.id {
            GuiType::Button { .. } => {
                let button_dimensions = component.get_dimension();
                text_buffers.push(Box::new(component.clone()));
                _menu_buttons.push(button_dimensions);
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], component);
            }
            GuiType::Score => (),
            GuiType::Timer => (),
            GuiType::Window => (),
            GuiType::Menu { .. } => (),
            _ => (),
        };
    }

    // Render GUI
    let mat_gui = mat.as_f32_array();
    for (_gui_type, vertex_buffer) in &vertex_buffers_gui {
        if _gui_type == SpriteType::Button {
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


    // Render Button Text
    for i in 0..text_buffers.len() {
        let system = glium_text::TextSystem::new(window);
        let mut text_to_display = "".to_string();
        let button = text_buffers[i].clone();
        let mut color = [1.0, 1.0, 1.0, 1.0f32];
        match button.id.clone() {
            GuiType::Button { text, highlight } => {
                text_to_display = text;
                if highlight { color = [0.1, 0.1, 0.1, 1.0f32]; }
            }
            GuiType::Menu { text, .. } => {
                text_to_display = text;
            }
            _ => ()
        }
        let text_display = format!("{}", text_to_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
        let text_width = (text.get_width() as f64) - 0.02;
        let text_height = 0.06;
        let dimensions = _menu_buttons[i];
        let button_width = dimensions.1.x - dimensions.0.x;
        let x_align = dimensions.0.x;
        let y_align = (dimensions.0.y) - 0.07;

        let menu_matrix = mat.translation(Vector4 { x: x_align, y: y_align, z: 0.0, w: 0.0 })
            .scale(Vector4 { x: button_width / text_width, y: text_height, z: 1.0, w: 1.0 }).as_f32_array();
        glium_text::draw(&text, &system, frame, menu_matrix, color);
    }

    // Render Entities
    let mat = Mat4::init_id_matrix();
    let score = compute_score(entity_counts);

    let system = glium_text::TextSystem::new(window);
    let text_1_loss = "Humanity Perished...".to_string();
    let text_display = format!("{}", text_1_loss);
    let str_slice: &str = &text_display[..];
    let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
    let color = [1.0, 1.0, 0.0, 1.0f32];
    let _font_scale_down = 1.5;
    let text_width = text.get_width() as f64;
    let (w, h) = frame.get_dimensions();
    let _text_offset = 1.0 / text_width;
    let scale_factor = Vector4 { x: 2.0 / text_width, y: 2.0 * (w as f64) / (h as f64) / text_width, z: 1.0, w: 1.0 };
    let translation_offset = Vector4 { x: -1.0, y: 0.7, z: 0.0, w: 0.0 };
    let mut matrix = mat.scale(scale_factor).translation(translation_offset);
    glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);

    // Score
    let text_display = format!("Score: {}", score);
    let str_slice: &str = &text_display[..];
    let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
    let color = [1.0, 1.0, 0.0, 1.0f32];
    let scale_factor = Vector4 { x: 0.5, y: 0.5, z: 1.0, w: 1.0 };
    let translate_offset = Vector4 { x: 0.1, y: -0.2, z: 0.0, w: 0.0 };
    matrix = matrix.scale(scale_factor).translation(translate_offset);
    glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);

    // Stats
    let text_display = format!("Cops: {}, Civilians: {}, Zombies: {}",
                               entity_counts.cops, entity_counts.civilians, entity_counts.zombies);

    let str_slice: &str = &text_display[..];
    let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
    let color = [1.0, 1.0, 0.0, 1.0f32];
    let translate_offset = Vector4 { x: 0.0, y: -0.2, z: 0.0, w: 0.0 };
    matrix = matrix.translation(translate_offset);
    glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);
}

pub fn display_victory_screen(
    frame: &mut glium::Frame,
    window: &glium_sdl2::SDL2Facade,
    programs: &Programs,
    _textures: &Textures,
    params: &glium::DrawParameters,
    camera_frame: [[f32; 4]; 4],
    ui: &mut Component,
    // state: &State,
    entity_counts: &EntityCounts,
    fonts: &FontPkg,
) {
    let font = fonts.get("Consola").unwrap();
    // background color
    //   frame.clear_color(0.0, 0.0, 0.0, 1.0);

    // initialize identity matrix
    let mat = Mat4::init_id_matrix();
    draw_victory_menu_background(frame, window, _textures, programs, camera_frame, params);


    let mut vertex_buffers_gui = enum_map! {_ => vec!()};
    let mut text_buffers = vec!();
    let mut menu_buttons: Vec<(Vector2, Vector2, Vector2, Vector2)> = vec![];

    // Compute vertices for GUI
    for component in &mut ui.components {
        match &component.id {
            GuiType::Button { .. } => {
                let button_dimensions = component.get_dimension();
                text_buffers.push(Box::new(component.clone()));
                menu_buttons.push(button_dimensions);
                push_gui_vertices(&mut vertex_buffers_gui[SpriteType::Button], component);
            }
            GuiType::Score => (),
            GuiType::Timer => (),
            GuiType::Window => (),
            GuiType::Menu { .. } => (),
            _ => (),
        };
    }

    // Render GUI
    let mat_gui = mat.as_f32_array();
    for (_gui_type, vertex_buffer) in &vertex_buffers_gui {
        if _gui_type == SpriteType::Button {
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

    // Render Button Text
    for i in 0..text_buffers.len() {
        let system = glium_text::TextSystem::new(window);
        let mut text_to_display = "".to_string();
        let button = text_buffers[i].clone();
        let mut color = [1.0, 1.0, 1.0, 1.0f32];
        match button.id.clone() {
            GuiType::Button { text, highlight } => {
                text_to_display = text;
                if highlight { color = [0.1, 0.1, 0.1, 1.0f32]; }
            }
            _ => ()
        }
        let text_display = format!("{}", text_to_display);
        let str_slice: &str = &text_display[..];
        let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
        let text_width = (text.get_width() as f64) - 0.02;
        let text_height = 0.06;
        let dimensions = menu_buttons[i];
        let button_width = dimensions.1.x - dimensions.0.x;
        let x_align = dimensions.0.x;
        let y_align = (dimensions.0.y) - 0.07;

        let menu_matrix = mat.translation(Vector4 { x: x_align, y: y_align, z: 0.0, w: 0.0 })
            .scale(Vector4 { x: button_width / text_width, y: text_height, z: 1.0, w: 1.0 }).as_f32_array();
        glium_text::draw(&text, &system, frame, menu_matrix, color);
    }

    let score = compute_score(entity_counts);

    let system = glium_text::TextSystem::new(window);
    let text_1_win = "Humanity Prevailed!".to_string();
    let text_display = format!("{}", text_1_win);
    let str_slice: &str = &text_display[..];
    let text = glium_text::TextDisplay::new(&system, font.highres(), str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let _font_scale_down = 1.5;
    let text_width = text.get_width() as f64;
    let (w, h) = frame.get_dimensions();
    let _text_offset = 1.0 / text_width;
    let scale_factor = Vector4 { x: 2.0 / text_width, y: 2.0 * (w as f64) / (h as f64) / text_width, z: 1.0, w: 1.0 };
    let translation_offset = Vector4 { x: -1.0, y: 0.8, z: 0.0, w: 0.0 };
    let mut matrix = mat.scale(scale_factor).translation(translation_offset);
    glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);

    // Score
    let text_display = format!("Score: {}", score);
    let str_slice: &str = &text_display[..];
    let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let scale_factor = Vector4 { x: 0.5, y: 0.5, z: 1.0, w: 1.0 };
    let translate_offset = Vector4 { x: 0.1, y: -0.2, z: 0.0, w: 0.0 };
    matrix = matrix.scale(scale_factor).translation(translate_offset);
    glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);

    // Stats
    let text_display = format!("Cops: {}, Civilians: {}, Zombies: {}",
                               entity_counts.cops, entity_counts.civilians, entity_counts.zombies);
    let str_slice: &str = &text_display[..];
    let text = glium_text::TextDisplay::new(&system, font.medres(), str_slice);
    let color = [0.0, 0.0, 0.0, 1.0f32];
    let translate_offset = Vector4 { x: 0.0, y: -0.2, z: 0.0, w: 0.0 };
    matrix = matrix.translation(translate_offset);
    glium_text::draw(&text, &system, frame, matrix.as_f32_array(), color);
}
