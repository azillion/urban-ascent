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
    All,
    Trucks,
    Cars,
    Taxis,
    Buses,
    Trains,
    Trams,
    Subways,
    Ships,
    Planes,
    Bikes,
    ElectricScooters,
    Pedestrians,
    None,
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
pub enum PathWidth {
    OneLane,
    TwoLane,
    ThreeLane,
    FourLane,
    FiveLane,
    SixLane,
    SevenLane,
    EightLane,
    NineLane,
    TenLane,
}

// PathConfig is a resource that contains the default values for a path when it is created.
// It is also used to validate the path when it is created.
#[derive(Resource)]
pub struct PathConfig {
    pub path_type: PathType,
    pub traffic_white_list: Vec<PathTraffic>,
    pub width: PathWidth,
    pub max_width: PathWidth,
    pub min_width: PathWidth,
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
        Self {
            path_type: PathType::Pedestrian,
            traffic_white_list: vec![PathTraffic::Pedestrians],
            width: PathWidth::OneLane,
            max_width: PathWidth::OneLane,
            min_width: PathWidth::OneLane,
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
    pub fn default_pedestrian_path() -> PathConfig {
        PathConfig {
            path_type: PathType::Pedestrian,
            traffic_white_list: vec![PathTraffic::Pedestrians],
            width: PathWidth::OneLane,
            max_width: PathWidth::OneLane,
            ..Default::default()
        }
    }

    pub fn default_bicycle_path() -> PathConfig {
        PathConfig {
            path_type: PathType::Bicycle,
            traffic_white_list: vec![PathTraffic::Bikes],
            width: PathWidth::OneLane,
            max_width: PathWidth::OneLane,
            ..Default::default()
        }
    }

    pub fn default_1u_road() -> PathConfig {
        PathConfig {
            path_type: PathType::Road,
            traffic_white_list: vec![
                PathTraffic::Cars,
                PathTraffic::Taxis,
                PathTraffic::Buses,
                PathTraffic::Trucks,
            ],
            width: PathWidth::OneLane,
            max_width: PathWidth::OneLane,
            ..Default::default()
        }
    }
}

pub struct PathPlugin;

impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PathConfig::default())
            .add_system(setup_path.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_path.in_set(OnUpdate(AppState::InGame)));
    }
}

fn setup_path(mut commands: Commands) {}

fn update_path(time: Res<Time>) {}
