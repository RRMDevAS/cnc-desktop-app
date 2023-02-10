
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PIDParams{
    pub prop: f32,
    pub inte: f32,
    pub deri: f32,
}

impl PIDParams {
    pub fn new() -> PIDParams {
        PIDParams{
            prop: 0.0f32,
            inte: 0f32,
            deri: 0f32,
        }
    }
}

pub enum ECncCtrlMessage {
    ETargetPosition(CncCoordinates),
    EPIDParams([PIDParams; 3]),
    EQuit,
}

impl ECncCtrlMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncCtrlMessage::ETargetPosition(_) => 1,
            ECncCtrlMessage::EPIDParams(_) => 2,
            ECncCtrlMessage::EQuit => 3,
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
            ECncCtrlMessage::EPIDParams(params) => {
                match bincode::serialize(&params) {
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
    pub speed: f32,
    pub target_position: f32,
    pub target_speed: f32,
    pub pid_prop_control: f32,
    pub pid_int_control: f32,
    pub pid_der_control: f32,
    pub duty: i32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct  CncStatus{
    pub cycle_time: i32,
    pub axis_status: [CncAxisStatus; 3],
}

pub enum ECncStatusMessage {
    ECurrentPosition(CncCoordinates),
    EStatus(CncStatus),
    EPIDParams([PIDParams;3]),
    EDisconnected,
}

impl ECncStatusMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncStatusMessage::ECurrentPosition(_) => 0,
            ECncStatusMessage::EStatus(_) => 1,
            ECncStatusMessage::EPIDParams(_) => 2,
            ECncStatusMessage::EDisconnected => 3,
        }
    }
}