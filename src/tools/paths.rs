use std::collections::HashSet;

use bevy::prelude::*;

use crate::AppState;

const MAX_SEGMENT_LENGTH: f32 = 10.0;
const MIN_SEGMENT_LENGTH: f32 = 2.0;
const MAX_SEGMENT_ANGLE: f32 = 45.0;
const MIN_SEGMENT_ANGLE: f32 = 0.0;
const MAX_SEGMENT_ANGLE_CHANGE: f32 = 10.0;
const MIN_SEGMENT_ANGLE_CHANGE: f32 = -10.0;
const MAX_SEGMENT_HEIGHT_CHANGE: f32 = 0.5;
const MIN_SEGMENT_HEIGHT_CHANGE: f32 = -0.5;
const MAX_SEGMENT_HEIGHT: f32 = 20.0;
const MIN_SEGMENT_HEIGHT: f32 = 0.0;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PathTraffic {
    None,
    Pedestrians,
    ElectricScooters,
    Bikes,
    Cars,
    Taxis,
    Trucks,
    Buses,
    Trams,
    SemiTrucks,
    Subways,
    Trains,
    Ships,
    Planes,
    All,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PathType {
    Pedestrian,
    Bicycle,
    Road,
    Rail,
    Water,
    Power,
    Gas,
    Sewage,
    Internet,
    Cable,
    Oil,
    Ore,
    Coal,
    Farm,
    Forest,
    Park,
    Leisure,
    Tourism,
    Education,
    Police,
    Fire,
    Prison,
    Government,
    Monument,
    Disaster,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PathLanes {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

// PathConfig is a resource that contains the default values for a path when it is created.
// It is also used to validate the path when it is created.
#[derive(Resource)]
pub struct PathConfig {
    pub path_type: PathType,
    pub traffic_white_list: HashSet<PathTraffic>,
    pub width: PathLanes,
    pub max_width: PathLanes,
    pub min_width: PathLanes,
    pub max_segment_length: f32,
    pub min_segment_length: f32,
    pub max_segment_angle: f32,
    pub min_segment_angle: f32,
    pub max_segment_angle_change: f32,
    pub min_segment_angle_change: f32,
    pub max_segment_height_change: f32,
    pub min_segment_height_change: f32,
    pub max_segment_height: f32,
    pub min_segment_height: f32,
}

impl Default for PathConfig {
    fn default() -> Self {
        let mut set = HashSet::new();
        set.insert(PathTraffic::Pedestrians);
        Self {
            path_type: PathType::Pedestrian,
            traffic_white_list: set,
            width: PathLanes::One,
            max_width: PathLanes::One,
            min_width: PathLanes::One,
            max_segment_length: MAX_SEGMENT_LENGTH,
            min_segment_length: MIN_SEGMENT_LENGTH,
            max_segment_angle: MAX_SEGMENT_ANGLE,
            min_segment_angle: MIN_SEGMENT_ANGLE,
            max_segment_angle_change: MAX_SEGMENT_ANGLE_CHANGE,
            min_segment_angle_change: MIN_SEGMENT_ANGLE_CHANGE,
            max_segment_height_change: MAX_SEGMENT_HEIGHT_CHANGE,
            min_segment_height_change: MIN_SEGMENT_HEIGHT_CHANGE,
            max_segment_height: MAX_SEGMENT_HEIGHT,
            min_segment_height: MIN_SEGMENT_HEIGHT,
        }
    }
}

impl PathConfig {
    pub fn pedestrian_path() -> PathConfig {
        PathConfig {
            path_type: PathType::Pedestrian,
            width: PathLanes::One,
            max_width: PathLanes::One,
            ..Default::default()
        }
    }

    pub fn bicycle_path() -> PathConfig {
        let mut set = HashSet::new();
        set.insert(PathTraffic::Bikes);
        PathConfig {
            path_type: PathType::Bicycle,
            traffic_white_list: set,
            width: PathLanes::One,
            max_width: PathLanes::One,
            ..Default::default()
        }
    }

    pub fn one_unit_road() -> PathConfig {
        let mut set = HashSet::new();
        set.insert(PathTraffic::ElectricScooters);
        set.insert(PathTraffic::Bikes);
        set.insert(PathTraffic::Cars);
        set.insert(PathTraffic::Taxis);
        set.insert(PathTraffic::Buses);
        set.insert(PathTraffic::Trucks);
        set.insert(PathTraffic::SemiTrucks);
        PathConfig {
            path_type: PathType::Road,
            traffic_white_list: set,
            width: PathLanes::One,
            max_width: PathLanes::One,
            ..Default::default()
        }
    }

    pub fn update(&mut self, config: PathConfig) {
        self.path_type = config.path_type;
        self.traffic_white_list = config.traffic_white_list;
        self.width = config.width;
        self.max_width = config.max_width;
        self.min_width = config.min_width;
        self.max_segment_length = config.max_segment_length;
        self.min_segment_length = config.min_segment_length;
        self.max_segment_angle = config.max_segment_angle;
        self.min_segment_angle = config.min_segment_angle;
        self.max_segment_angle_change = config.max_segment_angle_change;
        self.min_segment_angle_change = config.min_segment_angle_change;
        self.max_segment_height_change = config.max_segment_height_change;
        self.min_segment_height_change = config.min_segment_height_change;
        self.max_segment_height = config.max_segment_height;
        self.min_segment_height = config.min_segment_height;
    }
}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathConfig::default())
            .add_system(setup_path_config.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_path_config.in_set(OnUpdate(AppState::InGame)));
    }
}

fn setup_path_config(mut commands: Commands) {}

fn update_path_config(time: Res<Time>) {}
