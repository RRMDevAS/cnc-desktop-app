
use std::{str};
use std::sync::mpsc;
use serde::{Deserialize, Serialize};

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

enum ECncCtrlState {
    eOffline,
    eConnected,
}

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

pub struct CncCtrl
{
    e_cnc_ctrl_state: ECncCtrlState,
    pub target_coords   : CncCoordinates,
    pub current_coords  : CncCoordinates,
    o_tx            : Option<mpsc::Sender<ECncCtrlMessage>>,
    o_rx            : Option<mpsc::Receiver<ECncStatusMessage>>,
}

impl CncCtrl {
    pub fn new() -> CncCtrl {
        CncCtrl{
            e_cnc_ctrl_state: ECncCtrlState::eOffline,
            target_coords   : CncCoordinates::new(),
            current_coords  : CncCoordinates::new(),
            o_tx            : None,
            o_rx            : None,
        }
    }

    pub fn update_status(&mut self) {
        if let Some(ref rx) = self.o_rx {
            match rx.try_recv() {
                Ok( res ) => {
                    match res {
                        ECncStatusMessage::eCurrentPosition( current ) => {
                            println!("Received coordinates: {:?}", current.clone());
                            // self.current_coords = current;
                            self.set_current_coords(current.x, current.y, current.z);
                        },
                        ECncStatusMessage::eDisconnected => {

                        },
                        ECncStatusMessage::eStatus => {

                        },
                    }
                },
                Err(e) => {
                    if e==mpsc::TryRecvError::Disconnected {
                        println!("Failed to receive: {:?}", e);
                    }
                },
            }
        }
    }

    pub fn set_current_coords(&mut self, x: f32, y: f32, z: f32) {
        self.current_coords.x = x;
        self.current_coords.y = y;
        self.current_coords.z = z;
    }

    pub fn set_target_coords(&mut self, target_pos: CncCoordinates) {
        self.target_coords = target_pos;

        if let Some(ref tx) = self.o_tx {
            match tx.send( ECncCtrlMessage::eTargetPosition(self.target_coords.clone()) ) {
                Ok( () ) => {
                    println!("Message sent");
                }, 
                Err(e) => {
                    println!("Failed to send a message: {:?}", e);
                }
            }
        } else {
            println!("Cant send target position: Connection not set up!");
        }
    }

    pub fn get_target_coords(&self) -> CncCoordinates {
        self.target_coords.clone()
    }

    pub fn set_channels(&mut self, tx: mpsc::Sender<ECncCtrlMessage>, rx: mpsc::Receiver<ECncStatusMessage>) {
        self.o_tx = Some(tx);
        self.o_rx = Some(rx);
        self.e_cnc_ctrl_state = ECncCtrlState::eConnected;
    }
}