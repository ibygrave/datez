use std::fmt::Display;

use bevy_ecs::error::Result;
use bevy_ecs::{bundle::Bundle, component::Component, system::Query};
use jiff::{Span, civil::Date};
use jiff::{Unit, Zoned};
use serde::Deserialize;

#[derive(Component, Default)]
pub(crate) struct FixedDate(Date);

#[derive(Component, Deserialize)]
pub(crate) struct NextOrPrevDate {
    // offset from next occurence of the date.
    count: i16,
    month: i8,
    day: i8,
}

#[derive(Component, Default)]
pub(crate) struct ElapsedSince(Span);

#[derive(Component, Default)]
pub(crate) struct TotalDays(i64);

impl Display for TotalDays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.cmp(&0) {
            std::cmp::Ordering::Less => write!(f, "{} days away", -self.0),
            std::cmp::Ordering::Equal => write!(f, "today"),
            std::cmp::Ordering::Greater => write!(f, "{} days ago", self.0),
        }
    }
}

#[derive(Component, Default)]
pub(crate) struct SpanParts {
    years: i64,
    weeks: i8,
    days: i8,
}

impl Display for SpanParts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} years {} weeks {} days",
            self.years, self.weeks, self.days
        )
    }
}

#[derive(Component)]
pub(crate) struct DisplayLabel(String);

impl Display for DisplayLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
    }
}

pub(crate) fn calculate_next_date(mut query: Query<(&mut FixedDate, &NextOrPrevDate)>) -> Result {
    let now = Zoned::now().date();
    let this_year = now.year();
    for (mut next, date) in &mut query {
        let adjust: i16 = if Date::new(this_year, date.month, date.day)? > now {
            0
        } else {
            1
        };
        next.0 = Date::new(this_year + date.count + adjust, date.month, date.day)?;
    }
    Ok(())
}

pub(crate) fn calculate_fixed_date(mut query: Query<(&mut ElapsedSince, &FixedDate)>) -> Result {
    let now = Zoned::now().date();
    for (mut since, date) in &mut query {
        since.0 = Span::try_from(now.duration_since(date.0))?;
    }
    Ok(())
}

pub(crate) fn update_total_days(mut query: Query<(&mut TotalDays, &ElapsedSince)>) -> Result {
    let now = Zoned::now().date();
    for (mut total_days, span) in &mut query {
        total_days.0 = span.0.total((Unit::Day, now))? as i64;
    }
    Ok(())
}

pub(crate) fn update_span_parts(mut query: Query<(&mut SpanParts, &ElapsedSince)>) -> Result {
    let now = Zoned::now().date();
    for (mut span_parts, span) in &mut query {
        let mut s = span.0.abs();
        let years = s.total((Unit::Year, now))? as i64;
        s = s.checked_sub((Span::new().years(years), now))?;
        let weeks = s.total((Unit::Week, now))? as i8;
        s = s.checked_sub((Span::new().weeks(weeks), now))?;
        let days = s.total((Unit::Day, now))? as i8;
        *span_parts = SpanParts { years, weeks, days };
    }
    Ok(())
}

pub(crate) fn fixed_date_bundle(label: String, date: Date) -> impl Bundle {
    (
        DisplayLabel(label),
        FixedDate(date),
        ElapsedSince::default(),
        TotalDays::default(),
        SpanParts::default(),
    )
}

pub(crate) fn next_or_prev_date_bundle(label: String, next_date: NextOrPrevDate) -> impl Bundle {
    (
        DisplayLabel(label),
        next_date,
        FixedDate::default(),
        ElapsedSince::default(),
        TotalDays::default(),
        SpanParts::default(),
    )
}
