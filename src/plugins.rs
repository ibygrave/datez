use bevy_app::{App, Plugin, PostUpdate, Update};
use bevy_ecs::schedule::IntoScheduleConfigs;

use crate::{
    components::{calculate_fixed_date, calculate_next_date, update_span_parts, update_total_days},
    display::print_total_days,
};

pub(crate) struct DatezPlugin;

impl Plugin for DatezPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                calculate_next_date,
                calculate_fixed_date,
                update_total_days,
                update_span_parts,
            )
                .chain(),
        );
        app.add_systems(PostUpdate, print_total_days);
    }
}
