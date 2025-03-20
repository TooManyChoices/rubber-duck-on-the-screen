use bevy::{
    app::FixedMain,
    asset::io::{
        memory::{Dir, MemoryAssetReader},
        AssetSource, AssetSourceId,
    },
    ecs::component::Component,
    prelude::*,
};
use std::path::Path;

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

const DUCK_MODEL: &[u8] = include_bytes!("../assets/duck.glb");

fn main() {
    let memory_dir = MemoryDir {
        dir: Dir::default(),
    };
    let reader = MemoryAssetReader {
        root: memory_dir.dir.clone(),
    };
    App::new()
        .register_asset_source(
            AssetSourceId::from_static("mem"),
            AssetSource::build().with_reader(move || Box::new(reader.clone())),
        )
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RUBBER DUCK ON THE SCREEN".into(),
                name: Some("float rubber-duck".into()),
                resolution: (300., 300.).into(),
                transparent: true,
                decorations: false,
                resizable: false,
                resize_constraints: WindowResizeConstraints {
                    min_width: 300.,
                    min_height: 300.,
                    max_width: 300.,
                    max_height: 300.,
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(memory_dir)
        .insert_resource(ClearColor(Color::NONE))
        .add_systems(Startup, setup)
        .add_systems(FixedMain, move_duck)
        .run();
}

fn move_duck(time: Res<Time>, mut query: Query<(&mut Transform, &mut Duck)>) {
    for (mut t, mut duck) in query.iter_mut() {
        t.rotate_axis(
            Dir3::from_xyz(
                time.elapsed_secs().sin(),
                time.elapsed_secs().cos() * 0.5,
                0.,
            )
            .unwrap(),
            0.04,
        );
        const BOUNDS: f32 = 0.8;
        if t.translation.x.abs() > BOUNDS || t.translation.y.abs() > BOUNDS {
            let normal: Vec3;
            if t.translation.x > BOUNDS {
                normal = Vec3::new(-1., 0., 0.);
            } else if t.translation.x < -BOUNDS {
                normal = Vec3::new(1., 0., 0.);
            } else if t.translation.y > BOUNDS {
                normal = Vec3::new(0., -1., 0.);
            } else {
                normal = Vec3::new(0., 1., 0.);
            }
            let dot = duck.move_dir.dot(normal);
            duck.move_dir = (duck.move_dir.as_vec3() - (2. * (dot) * normal))
                .try_into()
                .unwrap();
        }
        t.translation += duck.move_dir * duck.move_speed;
    }
}

#[derive(Component)]
struct Duck {
    move_dir: Dir3,
    move_speed: f32,
}

fn setup(mut commands: Commands, ass: Res<AssetServer>, mem_dir: ResMut<MemoryDir>) {
    mem_dir.dir.insert_asset(Path::new("duck.glb"), DUCK_MODEL);

    // duck
    commands.spawn((
        Duck {
            move_dir: Dir3::from_xyz(0.6, 0.3, 0.)
                .unwrap()
                .normalize()
                .try_into()
                .unwrap(),
            move_speed: 0.03,
        },
        SceneRoot(ass.load(GltfAssetLabel::Scene(0).from_asset("mem://duck.glb"))),
    ));

    // camera
    commands.spawn((
        Transform::default().with_translation(Vec3 {
            x: 0.,
            y: 0.,
            z: 1.,
        }),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.01,
            scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                width: 300.,
                height: 300.,
            },
            ..OrthographicProjection::default_3d()
        }),
        Camera3d::default(),
    ));
}
