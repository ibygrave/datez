use bevy_ecs::system::Query;

use crate::components::{DisplayLabel, SpanParts, TotalDays};

pub(crate) fn print_total_days(query: Query<(&DisplayLabel, &TotalDays)>) {
    for (label, days) in &query {
        println!("{label}: {days}");
    }
}

pub(crate) fn print_span_parts(query: Query<(&DisplayLabel, &SpanParts)>) {
    for (label, parts) in &query {
        println!("{label}: {parts}");
    }
}
