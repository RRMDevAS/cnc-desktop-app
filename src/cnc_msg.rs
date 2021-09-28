
use serde::{Deserialize, Serialize};

use std::error::Error;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CncCoordinates{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl CncCoordinates {
    pub fn new() -> CncCoordinates {
        CncCoordinates{
            x: 0.0f32,
            y: 0f32,
            z: 0f32,
        }
    }
}

pub enum ECncCtrlMessage {
    ETargetPosition(CncCoordinates),
    EQuit,
}

impl ECncCtrlMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncCtrlMessage::ETargetPosition(coords) => 0,
            ECncCtrlMessage::EQuit => 1,
        }
    }

    pub fn bin_serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let u_type_id = self.get_type_id();
        match self {
            ECncCtrlMessage::ETargetPosition(coords) => {
                match bincode::serialize(&coords) {
                    Ok(mut vec) => {
                        let mut temp_vec = Vec::from( [u_type_id; 1] );
                        temp_vec.append(&mut vec);
                        Ok(temp_vec)
                    },
                    Err(e) => {
                        Err(e)
                    },
                }
            },
            ECncCtrlMessage::EQuit => {
                Ok(Vec::new())
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CncAxisStatus{
    pub position: f32,
    pub target_position: f32,
    pub speed: f32,
    pub target_speed: f32,
    pub pid_prop_control: f32,
    pub pid_int_control: f32,
    pub pid_der_control: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct  CncStatus{
    pub cycle_time: f32,
    pub axis_status: [CncAxisStatus; 3],
}

pub enum ECncStatusMessage {
    ECurrentPosition(CncCoordinates),
    EStatus(CncStatus),
    EDisconnected,
}

impl ECncStatusMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncStatusMessage::ECurrentPosition(coords) => 0,
            ECncStatusMessage::EStatus(status) => 1,
            ECncStatusMessage::EDisconnected => 2,
        }
    }
}