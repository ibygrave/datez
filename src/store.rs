use crate::components::{NextOrPrevDate, fixed_date_bundle, next_or_prev_date_bundle};

use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

use bevy_ecs::error::Result;
use bevy_ecs::resource::Resource;
use bevy_ecs::system::{Commands, Res};
use jiff::civil::Date;
use serde::Deserialize;

#[derive(Resource)]
pub(crate) struct DataSource {
    filename: PathBuf,
}

impl DataSource {
    pub(crate) fn new<T>(filename: T) -> Self
    where
        PathBuf: From<T>,
    {
        Self {
            filename: filename.into(),
        }
    }
}

#[derive(Deserialize)]
pub(crate) enum Stored {
    FixedDate { label: String, date: String },
    NextDate { label: String, date: NextOrPrevDate },
}

impl Stored {
    pub(crate) fn add_event(self, commands: &mut Commands) -> Result {
        match self {
            Stored::FixedDate { label, date } => {
                let date = Date::from_str(&date)?;
                commands.spawn(fixed_date_bundle(label, date));
            }
            Stored::NextDate { label, date } => {
                commands.spawn(next_or_prev_date_bundle(label, date));
            }
        }
        Ok(())
    }
}

pub(crate) fn load(data_source: Res<DataSource>, mut commands: Commands) -> Result {
    let stored: Vec<Stored> = serde_json::from_reader(File::open(&data_source.filename)?)?;
    for entry in stored {
        entry.add_event(&mut commands)?;
    }
    Ok(())
}
