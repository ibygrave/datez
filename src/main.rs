mod components;
mod display;
mod plugins;
mod store;

use std::time::Duration;

use bevy::MinimalPlugins;
use bevy_app::{App, PluginGroup, ScheduleRunnerPlugin, Startup};

fn main() {
    App::new()
        .insert_resource(store::DataSource::new("data.json"))
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs(10))))
        .add_plugins(plugins::DatezPlugin)
        .add_systems(Startup, store::load)
        .run();
}
