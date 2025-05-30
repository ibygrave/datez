use bevy_app::{App, Plugin, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    components::{update_elapsed_since, update_span_parts, update_total_days},
    display::{print_span_parts, print_total_days},
};

pub(crate) struct DatezPlugin;

impl Plugin for DatezPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_elapsed_since,
                update_total_days,
                update_span_parts,
                print_total_days,
                print_span_parts,
            )
                .chain(),
        );
    }
}
