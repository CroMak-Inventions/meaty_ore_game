use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct AppGlobals {
    pub score: i32,
    pub high_score: i32,
    pub last_score: i32,
    pub level: i32,
}

impl AppGlobals {
    pub fn final_score_update(&mut self) {
        // Process the final score at the end of a game.
        self.last_score = self.score;

        if self.last_score > self.high_score {
            self.high_score = self.last_score;
        }

        self.score = 0;
        self.level = 1;
    }

}

pub struct AppGlobalsPlugin;

impl Plugin for AppGlobalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppGlobals>()
        .add_systems(Startup, init_globals);
    }
}

fn init_globals(
    mut app_globals: ResMut<AppGlobals>,
) {
    *app_globals = AppGlobals {
        score: 0,
        high_score: 0,
        last_score: 0,
        level: 1,
    }
}
