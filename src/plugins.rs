use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    components::{update_elapsed_since, update_span_parts, update_total_days},
    display::print_total_days,
};

pub(crate) struct DatezPlugin;

impl Plugin for DatezPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_elapsed_since, update_total_days, update_span_parts).chain(),
        );
        app.add_systems(PostUpdate, print_total_days);
    }
}
