use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::math::vec2;
use bevy::{asset::AssetMetaCheck, math::vec3, window::PrimaryWindow};
use derivative::Derivative;
use rand::Rng;

mod solarized {
    #![allow(unused)] 
    use std::sync::LazyLock;
    use bevy::render::color::Color;
    pub static BASE03:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(  0,  43,  54));
    pub static BASE02:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(  7,  54,  66));
    pub static BASE01:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8( 88, 110, 117));
    pub static BASE00:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(101, 123, 131));
    pub static BASE0:         LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(131, 148, 150));
    pub static BASE1:         LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(147, 161, 161));
    pub static BASE2:         LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(238, 232, 213));
    pub static BASE3:         LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(253, 246, 227));
    pub static YELLOW:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(181, 137,   0));
    pub static ORANGE:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(203,  75,  22));
    pub static RED:           LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(220,  50,  47));
    pub static MAGENTA:       LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(211,  54, 130));
    pub static VIOLET:        LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(108, 113, 196));
    pub static BLUE:          LazyLock<Color> = LazyLock::new(|| Color::rgb_u8( 38, 139, 210));
    pub static CYAN:          LazyLock<Color> = LazyLock::new(|| Color::rgb_u8( 42, 161, 152));
    pub static GREEN:         LazyLock<Color> = LazyLock::new(|| Color::rgb_u8(133, 153,   0));
}

#[derive(Component)]
struct ListData {
    list_index: usize,
    index: usize,
}

#[derive(Component, PartialEq, Eq)]
enum Clickable {
    IncreaseValue,
    DecreaseValue,
    Next,
    Previous,
}
#[derive(Bundle)]
struct ContainerBundle {
    list_data: ListData,
    transform: Transform,
    global_transform: GlobalTransform,
}


impl ContainerBundle {
    fn rect() -> (ShapeBundle, Fill, Stroke) {
        let shape = shapes::Rectangle {
            extents: vec2(190.0, 90.0),
            origin: RectangleOrigin::Center
        };
        (   
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Fill::color(*solarized::BLUE),
            Stroke::new(*solarized::BASE2, 10.0),
        )
    }
    fn button(icon: &str, scale: f32, clickable: Clickable) -> (ShapeBundle, Fill, Stroke, BoundingBox, Clickable) {
        let shape = shapes::SvgPathShape {
            svg_doc_size_in_px: vec2(90.0, 90.0),
            svg_path_string: ("M 0,0 h 90 v 90 h -90 z ".to_owned()) + icon
        };
        let mut shape_bundle = ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        };
        shape_bundle.spatial.transform.scale = vec3(scale, scale, 1.0);
        (
            shape_bundle,
            Fill::color(*solarized::VIOLET),
            Stroke::new(*solarized::BASE2, 10.0),
            BoundingBox(Vec2::new(100.0 * scale, 100.0 * scale)),
            clickable
        )
    }
    fn cross_button(scale: f32, clickable: Clickable) -> (ShapeBundle, Fill, Stroke, BoundingBox, Clickable) {
        Self::button("M 20,45 h 50 M 45,20 v 50", scale, clickable)
    }
    fn minus_button(scale: f32, clickable: Clickable) -> (ShapeBundle, Fill, Stroke, BoundingBox, Clickable) {
        Self::button("M 20,45 h 50", scale, clickable)
    }
    fn arrow_button(scale: f32, clickable: Clickable) -> (ShapeBundle, Fill, Stroke, BoundingBox, Clickable) {
        Self::button("M 15,45 h 40 v -10 l 12.5 10 l -12.5 10 v -10", scale, clickable)
    }
    fn spawn_new(list_data: ListData, pos: Vec2, commands: &mut Commands, glist_data: &GlobalListData) {
        let mut increasing_button = Self::cross_button(0.25, Clickable::IncreaseValue);
        increasing_button.0.spatial.transform.translation = vec3(80.0,30.0,10.0);
        let mut decreasing_button = Self::minus_button(0.25, Clickable::DecreaseValue);
        decreasing_button.0.spatial.transform.translation = vec3(80.0,-30.0,10.0);
        let mut next_button = Self::arrow_button(0.25, Clickable::Next);
        next_button.0.spatial.transform.translation = vec3(80.0,5.0,10.0);
        let mut previous_button = Self::arrow_button(0.25, Clickable::Previous);
        previous_button.0.spatial.transform.translation = vec3(-80.0,5.0,10.0);
        previous_button.0.spatial.transform.rotation = Quat::from_axis_angle(Vec3::Z, std::f32::consts::PI);
        let container = Self::rect();
        let text = Text2dBundle {
            text: Text::from_section(
                glist_data.data[list_data.list_index][0].to_string(),
                TextStyle {
                    font_size: 1000.0,
                    color: *solarized::MAGENTA,
                    ..Default::default()
                    // font,
                }
            ),
            transform: Transform::from_scale(vec3(0.05, 0.05, 1.0)).with_translation(vec3(0.0, 0.0, 10.0)),
            ..Default::default()
        };
        let container = commands.spawn(container).id();
        let increasing_button = commands.spawn(increasing_button).id();
        let decreasing_button = commands.spawn(decreasing_button).id();
        let next_button = commands.spawn(next_button).id();
        let previous_button = commands.spawn(previous_button).id();
        let text = commands.spawn(text).id();
        let bundle = ContainerBundle {
            list_data,
            transform: Transform::from_translation(pos.extend(0.0)),
            global_transform: GlobalTransform::default()
        };
        commands.spawn(bundle).push_children(&[increasing_button, decreasing_button, next_button, previous_button, text, container]);
    }
}

fn update_container_values(q: Query<(&ListData, &Children)>, mut q2: Query<&mut Text>, glist_data: Res<GlobalListData>) {
    for (data, children) in q.iter() {
        for child in children.iter() {
            if let Ok(mut t) = q2.get_mut(*child) {
                t.sections[0].value = glist_data.data[data.list_index][data.index].to_string();
            }
        }
    }

}

#[derive(Resource)]
struct GlobalListData {
    data: Vec<VecDeque<i32>>
}
fn main() {
    App::new()
        .insert_resource(GlobalListData {
            data: Vec::new()
        })
        .add_event::<Click>()
        .insert_resource(Time::<bevy::prelude::Fixed>::from_hz(60.0))
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(MousePosition::default())
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_drag, update_cursor_position, mouse_input, update_container_values, handle_clicks, change_button_color_when_pushable))
        .run()
}

fn setup(
    mut commands: Commands, 
    mut window: Query<&mut Window>, 
    mut glist_data: ResMut<GlobalListData>,
) {
    commands.spawn(Camera2dBundle {
        ..Default::default()
    });
    let mut window = window.single_mut();
    window.decorations = false;
    window.resizable = false;
    glist_data.data.push(vec![100, 200, 300, 400, 500].into());
    // glist_data.data.push(vec![1100, 1200, 1300, 1400, 1500].into());

    ContainerBundle::spawn_new(ListData { list_index: 0, index: 0 }, vec2(0.0, 0.0), &mut commands, &glist_data);
    // ContainerBundle::spawn_new(ListData { list_index: 1, index: 0 }, vec2(0.0, 150.0), &mut commands, font.clone(), &glist_data);
}


#[derive(Derivative)]
#[derivative(Default)]
#[derive(Resource)]
struct MousePosition {
    current: Option<Vec2>,
    // previous: Option<Vec2>,
    last_left_press: Option<Vec2>,
    drag: Vec2,
    translation: Vec2,
    #[derivative(Default(value="1.0"))]
    zoom: f32
}


#[derive(Component)]
struct BoundingBox(Vec2);


#[derive(Event)]
struct Click(Vec2);


fn change_button_color_when_pushable(
    q: Query<(&ListData, Entity)>, 
    glist_data: Res<GlobalListData>, 
    mut q2: Query<(&mut Fill, &Clickable, &Parent)>, 
    mut commands: Commands
) {
    for (data, en) in q.iter() {
        let col = if data.index == glist_data.data[data.list_index].len() - 1 {
            *solarized::RED
        } else {*solarized::VIOLET};
        let col2 = if data.index == 0 {
            *solarized::RED
        } else {*solarized::VIOLET};
        for (mut fill, clickable, par) in q2.iter_mut() {
            if en != commands.entity(**par).id() {continue};
            match clickable {
                Clickable::Next => {
                    fill.color = col;
                },
                Clickable::Previous => {
                    fill.color = col2;
                },
                _ => {continue}
            }
        }
        
    }
}

fn handle_clicks(
    mut er: EventReader<Click>, 
    q: Query<(&BoundingBox, &Clickable, &GlobalTransform, &Parent)>, 
    mut par: Query<&mut ListData>,
    mut glist_data: ResMut<GlobalListData>
) {
    for Click(pos) in er.read() {
        println!("{:?}", pos);
        for (BoundingBox(bbox), clickable, gtrans, parent) in q.iter() {
            let final_pos = (gtrans.translation()).xy();
            let p = *pos - final_pos;
            let bbox = *bbox * 0.5;
            if p.x.abs() < bbox.x && p.y.abs() < bbox.y {
                match clickable {
                    Clickable::IncreaseValue | Clickable::DecreaseValue => {
                        let Ok(data) = par.get_mut(**parent) else {continue};
                        glist_data.data[data.list_index][data.index] += match clickable {
                            Clickable::IncreaseValue => 1,
                            Clickable::DecreaseValue => -1,
                            _ => unreachable!()
                        };
                    },
                    Clickable::Next => {
                        let Ok(mut data) = par.get_mut(**parent) else {continue};
                        if data.index == glist_data.data[data.list_index].len() - 1 {
                            glist_data.data[data.list_index].push_back(rand::thread_rng().gen_range(0..=10));
                        }
                        data.index += 1;
                    },
                    Clickable::Previous => {
                        let Ok(mut data) = par.get_mut(**parent) else {continue};
                        if data.index == 0 {
                            glist_data.data[data.list_index].push_front(rand::thread_rng().gen_range(0..=10));
                        }
                        else {data.index -= 1};
                    }
                }
            }
        }
    }
}

fn handle_drag(
    pos: Res<MousePosition>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut camera = camera.single_mut();
    *camera = Transform::default()
            .with_translation(pos.drag.extend(0.0) + pos.translation.extend(0.0))
            .with_scale(vec3(pos.zoom, pos.zoom, 1.0));
}


fn update_cursor_position( // update position constantly
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut pos: ResMut<MousePosition>
) {
    // pos.previous = pos.current;
    let w = q_windows.single();
    
    let p = q_windows.single().cursor_position();
    pos.current = p.map(|p| vec2(p.x - w.width()/2.0, -p.y + w.height()/2.0));
}


fn mouse_input(
    buttons: Res<ButtonInput<MouseButton>>,
    mut wheel: EventReader<MouseWheel>,
    mut pos: ResMut<MousePosition>,
    mut ew: EventWriter<Click>
) {
    if buttons.just_pressed(MouseButton::Left) {
        pos.last_left_press = pos.current;
    }
    if buttons.pressed(MouseButton::Left) {
        pos.drag = pos.current.zip(pos.last_left_press).map(|(a,b)| b-a).unwrap_or_default() * pos.zoom;
    }
    if buttons.just_released(MouseButton::Left) {
        let d = pos.drag;
        if d.length_squared() < 10.0 {
            ew.send(Click(pos.current.unwrap_or_default() * pos.zoom + pos.translation));
        }
        pos.translation += d;
        pos.drag = Vec2::ZERO;
    }
    for ev in wheel.read() {
        pos.zoom *= 0.95f32.powf(ev.y);
        pos.zoom = pos.zoom.clamp(0.001, 25.0);
    }
}