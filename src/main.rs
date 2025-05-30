use std::{fs::File, path::PathBuf, str::FromStr, time::Duration};

use bevy::MinimalPlugins;
use bevy_app::{App, Plugin, PluginGroup, ScheduleRunnerPlugin, Startup, Update};
use bevy_ecs::{
    component::Component,
    resource::Resource,
    schedule::IntoScheduleConfigs,
    system::{Commands, Query, Res},
};
use jiff::{Span, Unit, Zoned, civil::Date};
use serde::Deserialize;

#[derive(Component)]
struct FixedDate(Date);

#[derive(Component, Default)]
struct ElapsedSince(Span);

#[derive(Component, Default)]
struct TotalDays(u64);

#[derive(Component, Default)]
struct SpanParts {
    years: i64,
    weeks: i8,
    days: i8,
}

#[derive(Deserialize)]
enum Stored {
    FixedDate { label: String, date: String },
}

#[derive(Resource)]
struct DataSource {
    filename: PathBuf,
}

#[derive(Component)]
struct DisplayLabel(String);

fn update_elapsed_since(mut query: Query<(&mut ElapsedSince, &FixedDate)>) {
    let now = Zoned::now().date();
    for (mut since, date) in &mut query {
        since.0 = Span::try_from(now.duration_since(date.0)).unwrap();
    }
}

fn update_total_days(mut query: Query<(&mut TotalDays, &ElapsedSince)>) {
    let now = Zoned::now().date();
    for (mut total_days, span) in &mut query {
        total_days.0 = span.0.total((Unit::Day, now)).unwrap() as u64;
    }
}

fn update_span_parts(mut query: Query<(&mut SpanParts, &ElapsedSince)>) {
    let now = Zoned::now().date();
    for (mut span_parts, span) in &mut query {
        let mut s = span.0;
        let years = s.total((Unit::Year, now)).unwrap() as i64;
        s = s.checked_sub((Span::new().years(years), now)).unwrap();
        let weeks = s.total((Unit::Week, now)).unwrap() as i8;
        s = s.checked_sub((Span::new().weeks(weeks), now)).unwrap();
        let days = s.total((Unit::Day, now)).unwrap() as i8;
        *span_parts = SpanParts { years, weeks, days };
    }
}

fn print_total_days(query: Query<(&DisplayLabel, &TotalDays)>) {
    for (label, days) in &query {
        println!("{}: {} days", label.0, days.0);
    }
}

fn print_span_parts(query: Query<(&DisplayLabel, &SpanParts)>) {
    for (label, parts) in &query {
        println!(
            "{}: {} years {} weeks {} days",
            label.0, parts.years, parts.weeks, parts.days
        );
    }
}

struct DatezPlugin;

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

fn add_events(data_source: Res<DataSource>, mut commands: Commands) {
    let stored: Vec<Stored> =
        serde_json::from_reader(File::open(&data_source.filename).unwrap()).unwrap();
    for entry in stored {
        match entry {
            Stored::FixedDate { label, date } => {
                let date = Date::from_str(&date).unwrap();
                commands.spawn((
                    DisplayLabel(label),
                    FixedDate(date),
                    ElapsedSince::default(),
                    TotalDays::default(),
                    SpanParts::default(),
                ));
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(DataSource {
            filename: "data.json".into(),
        })
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(10))))
        .add_plugins(DatezPlugin)
        .add_systems(Startup, add_events)
        .run();
}
