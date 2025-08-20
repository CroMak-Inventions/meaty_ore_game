use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
    pub shooting_sound: Handle<AudioSource>,
    pub meteor_hit_sound: Handle<AudioSource>,
    pub thruster_sound: Handle<AudioSource>,
    pub background_music: Handle<AudioSource>,
    pub font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
        .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>
) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Rock-0.glb")),
        spaceship: asset_server.load(GltfAssetLabel::Scene(0).from_asset("SpaceshipNew.glb")),
        missiles: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Bullet.glb")),
        shooting_sound: asset_server.load("sound/Shoot-2.ogg"),
        meteor_hit_sound: asset_server.load("sound/MeteorHit-3.ogg"),
        thruster_sound: asset_server.load("sound/Rocket.ogg"),
        background_music: asset_server.load("sound/CryForMercyButTheClockTicksDown.ogg"),
        font: asset_server.load("fonts/fira-sans.bold.ttf"),
    }
}
