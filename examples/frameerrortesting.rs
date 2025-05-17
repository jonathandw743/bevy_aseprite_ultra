use std::time::Duration;

use bevy::{
    image::ImageSamplerDescriptor,
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin},
    winit::WinitSettings,
};
use bevy_aseprite_ultra::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: ImageSamplerDescriptor::nearest(),
        }))
        .insert_resource(WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Reactive {
                wait: Duration::from_millis(200),
                react_to_device_events: false,
                react_to_user_events: false,
                react_to_window_events: false,
            },
            ..default()
        })
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(Material2dPlugin::<MyMaterial>::default())
        .add_plugins(Material2dPlugin::<MyMaterial2>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, render_animation::<MeshMaterial2d<MyMaterial>>)
        .add_systems(PostUpdate, render_animation::<MeshMaterial2d<MyMaterial2>>)
        .run();
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct MyMaterial {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
    #[uniform(3)]
    texture_min: UVec2,
    #[uniform(4)]
    texture_max: UVec2,
    #[uniform(5)]
    time: f32,
}

impl Material2d for MyMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "my_shader.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl RenderAnimation for MyMaterial {
    type Extra<'e> = (Res<'e, Time>, Res<'e, Assets<TextureAtlasLayout>>);
    fn render_animation(
        &mut self,
        aseprite: &Aseprite,
        state: &AnimationState,
        extra: &mut Self::Extra<'_>,
    ) {
        let Some(atlas_layout) = extra.1.get(&aseprite.atlas_layout) else {
            return;
        };
        self.image = aseprite.atlas_image.clone();
        let index = aseprite.get_atlas_index(usize::from(state.current_frame));
        self.texture_min = atlas_layout.textures[index].min;
        self.texture_max = atlas_layout.textures[index].max;
        self.time = extra.0.elapsed_secs();
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Default)]
pub struct MyMaterial2 {
    #[texture(1)]
    #[sampler(2)]
    image: Handle<Image>,
    #[uniform(3)]
    texture_min: UVec2,
    #[uniform(4)]
    texture_max: UVec2,
    #[uniform(5)]
    time: f32,
}

impl Material2d for MyMaterial2 {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "my_shader.wgsl".into()
    }
    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl RenderAnimation for MyMaterial2 {
    type Extra<'e> = (Res<'e, Time>, Res<'e, Assets<TextureAtlasLayout>>);
    fn render_animation(
        &mut self,
        aseprite: &Aseprite,
        state: &AnimationState,
        extra: &mut Self::Extra<'_>,
    ) {
        let Some(atlas_layout) = extra.1.get(&aseprite.atlas_layout) else {
            return;
        };
        self.image = aseprite.atlas_image.clone();
        let index = aseprite.get_atlas_index(usize::from(state.current_frame));
        self.texture_min = atlas_layout.textures[index].min;
        self.texture_max = atlas_layout.textures[index].max;
        self.time = extra.0.elapsed_secs();
    }
}

fn setup(
    mut cmd: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut materials2: ResMut<Assets<MyMaterial2>>,
) {
    info!("Update on the left, PostUpdate on the right");
    cmd.spawn((Camera2d, Transform::default().with_scale(Vec3::splat(0.15))));
    cmd.spawn((
        AseAnimation {
            aseprite: server.load("player_slow.aseprite"),
            animation: Animation::tag("walk-down"),
        },
        Mesh2d(meshes.add(Mesh::from(Rectangle::from_size(Vec2::splat(100.0))))),
        MeshMaterial2d(materials.add(MyMaterial::default())),
        Transform {
            translation: vec3(-50.0, 0.0, 0.0),
            ..default()
        },
    ));
    cmd.spawn((
        AseAnimation {
            aseprite: server.load("player_slow.aseprite"),
            animation: Animation::tag("walk-down"),
        },
        Mesh2d(meshes.add(Mesh::from(Rectangle::from_size(Vec2::splat(100.0))))),
        MeshMaterial2d(materials2.add(MyMaterial2::default())),
        Transform {
            translation: vec3(50.0, 0.0, 0.0),
            ..default()
        },
    ));
}
