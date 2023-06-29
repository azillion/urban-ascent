use bevy::prelude::*;

const DAY_LENGTH: f32 = 60. * 24.; // 24 minutes
const WEEK_LENGTH: f32 = DAY_LENGTH / 47.; // 30 seconds

pub enum TimeEvent {
    TimeStateChange(TimeState),
    NewDay,
    NewWeek(u8),
    NewMonth(Month),
    NewSeason(Season),
    NewYear(u32),
}

pub enum TimeState {
    Day,
    Night,
}

pub enum Season {
    Spring,
    Summer,
    Fall,
    Winter,
}

pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

#[derive(Resource)]
pub struct TimeConfig {
    day_timer: Timer,    // 24 minutes
    week_timer: Timer,   // 30 seconds
    seconds_in_day: f32, // 0.0 - 1440.0
    week: u8,            // 0 - 47
    month: u8,           // 0 - 11
    year: u32,
}

impl TimeConfig {
    pub fn time(&self) -> f32 {
        self.seconds_in_day % DAY_LENGTH
    }

    pub fn week(&self) -> u8 {
        self.week
    }

    pub fn month(&self) -> Month {
        match self.month {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            11 => Month::December,
            _ => Month::January,
        }
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn time_state(&self) -> TimeState {
        if self.time() < DAY_LENGTH / 2. {
            TimeState::Day
        } else {
            TimeState::Night
        }
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        TimeConfig {
            day_timer: Timer::from_seconds(DAY_LENGTH, TimerMode::Repeating),
            week_timer: Timer::from_seconds(WEEK_LENGTH, TimerMode::Repeating),
            seconds_in_day: 0.,
            week: 0,
            month: 0,
            year: 2000,
        }
    }
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeConfig::default())
            .add_event::<TimeEvent>();
    }
}

pub fn update_time(
    time: Res<Time>,
    mut time_config: ResMut<TimeConfig>,
    mut events: EventWriter<TimeEvent>,
) {
    time_config.day_timer.tick(time.delta());
    time_config.week_timer.tick(time.delta());

    let mut is_new_day = false;

    let mut events_batch: Vec<TimeEvent> = vec![];

    if time_config.day_timer.finished() {
        time_config.day_timer.reset();
        events_batch.extend(new_day(&mut time_config));
        is_new_day = true;
    }

    if time_config.week_timer.finished() {
        time_config.week_timer.reset();
        let week = time_config.week + 1;
        if week > 47 {
            // new year
            if !is_new_day {
                time_config.day_timer.reset();
                events_batch.extend(new_day(&mut time_config));
                is_new_day = true;
            }
            time_config.week = 0;
            time_config.month = 0;
            events_batch.push(TimeEvent::NewWeek(time_config.week()));
            events_batch.push(TimeEvent::NewMonth(time_config.month()));
            events_batch.extend(new_year(&mut time_config));
        } else {
            events_batch.extend(new_week(&mut time_config));
            if week % 4 == 0 {
                events_batch.extend(new_month(&mut time_config));
            }
        }
    }

    if !is_new_day {
        time_config.seconds_in_day =
            (time_config.seconds_in_day + time.delta_seconds()) % DAY_LENGTH;
    }
    events.send_batch(events_batch);
}

fn new_day(mut time_config: &mut TimeConfig) -> Vec<TimeEvent> {
    time_config.seconds_in_day = 0.;
    vec![
        TimeEvent::NewDay,
        TimeEvent::TimeStateChange(time_config.time_state()),
    ]
}

fn new_week(time_config: &mut TimeConfig) -> Vec<TimeEvent> {
    time_config.week += 1;
    vec![TimeEvent::NewWeek(time_config.week())]
}

fn new_month(time_config: &mut TimeConfig) -> Vec<TimeEvent> {
    let mut events_batch: Vec<TimeEvent> = vec![];
    time_config.month += 1;
    events_batch.push(TimeEvent::NewMonth(time_config.month()));
    let new_season = match time_config.month() {
        Month::March => Some(TimeEvent::NewSeason(Season::Spring)),
        Month::June => Some(TimeEvent::NewSeason(Season::Summer)),
        Month::September => Some(TimeEvent::NewSeason(Season::Fall)),
        Month::December => Some(TimeEvent::NewSeason(Season::Winter)),
        _ => None,
    };
    if let Some(season) = new_season {
        events_batch.push(season);
    }
    events_batch
}

fn new_year(time_config: &mut TimeConfig) -> Vec<TimeEvent> {
    time_config.year += 1;
    vec![TimeEvent::NewYear(time_config.year())]
}
