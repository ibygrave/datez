use bevy_ecs::system::Query;

use crate::components::{DisplayLabel, SpanParts, TotalDays};

pub(crate) fn print_total_days(query: Query<(&DisplayLabel, &TotalDays, &SpanParts)>) {
    for (label, days, parts) in &query {
        println!("{label}: {days} ago ({parts})");
    }
}
