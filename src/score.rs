use crate::ball;
use bevy::prelude::{info, App, Event, EventReader, EventWriter, PostUpdate};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CreateScore>()
            .add_systems(PostUpdate, create_score_handler);
    }
}

#[derive(Debug, Clone)]
pub enum Scorer {
    Player,
    Enemy,
}

#[derive(Debug, Clone, Event)]
pub struct CreateScore {
    pub scorer: Scorer,
}

fn create_score_handler(
    mut create_score: EventReader<CreateScore>,
    mut respawn: EventWriter<ball::Respawn>,
) {
    if create_score.is_empty() {
        return;
    }

    for CreateScore { scorer } in create_score.read() {
        info!("scorer={scorer:?}");
        respawn.send(ball::Respawn);
    }
}
