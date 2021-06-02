
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
    eTargetPosition(CncCoordinates),
    eQuit,
}

impl ECncCtrlMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncCtrlMessage::eTargetPosition(coords) => 0,
            ECncCtrlMessage::eQuit => 1,
        }
    }

    pub fn bin_serialize(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let u_type_id = self.get_type_id();
        match self {
            ECncCtrlMessage::eTargetPosition(coords) => {
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
            ECncCtrlMessage::eQuit => {
                Ok(Vec::new())
            },
        }
    }
}

pub enum ECncStatusMessage {
    eCurrentPosition(CncCoordinates),
    eStatus,
    eDisconnected,
}

impl ECncStatusMessage {
    pub fn get_type_id(&self) -> u8 {
        match self {
            ECncStatusMessage::eCurrentPosition(coords) => 0,
            ECncStatusMessage::eStatus => 1,
            ECncStatusMessage::eDisconnected => 2,
        }
    }
}