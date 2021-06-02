
use std::{str};
use std::sync::mpsc;
use serde::{Deserialize, Serialize};

use crate::cnc_msg::{CncCoordinates, ECncCtrlMessage, ECncStatusMessage};
use crate::cnc_connection::CncConnection;

enum ECncCtrlState {
    eOffline,
    eConnected,
}


pub struct CncCtrl
{
    e_cnc_ctrl_state: ECncCtrlState,
    pub target_coords   : CncCoordinates,
    pub current_coords  : CncCoordinates,
    // o_tx            : Option<mpsc::Sender<ECncCtrlMessage>>,
    // o_rx            : Option<mpsc::Receiver<ECncStatusMessage>>,
    connection      : CncConnection<ECncCtrlMessage, ECncStatusMessage>
}

impl CncCtrl {
    pub fn new() -> CncCtrl {
        CncCtrl{
            e_cnc_ctrl_state: ECncCtrlState::eOffline,
            target_coords   : CncCoordinates::new(),
            current_coords  : CncCoordinates::new(),
            // o_tx            : None,
            // o_rx            : None,
            connection      : CncConnection::new(),
        }
    }

    pub fn update_status(&mut self) {
        match self.connection.receive() {
            Ok(msg) => {
                if let Some(status) = msg {
                    match status {
                        ECncStatusMessage::eCurrentPosition( current ) => {
                            // println!("Received coordinates: {:?}", current.clone());
                            // self.current_coords = current;
                            self.set_current_coords(current.x, current.y, current.z);
                        },
                        ECncStatusMessage::eDisconnected => {
        
                        },
                        ECncStatusMessage::eStatus => {
        
                        },
                    }
                }
            }, 
            Err(e) => {
                if e==mpsc::TryRecvError::Disconnected {
                    println!("Failed to receive: {:?}", e);
                }
            },
        }
        // if let Some(ref rx) = self.o_rx {
        //     match rx.try_recv() {
        //         Ok( res ) => {
        //             match res {
        //                 ECncStatusMessage::eCurrentPosition( current ) => {
        //                     // println!("Received coordinates: {:?}", current.clone());
        //                     // self.current_coords = current;
        //                     self.set_current_coords(current.x, current.y, current.z);
        //                 },
        //                 ECncStatusMessage::eDisconnected => {

        //                 },
        //                 ECncStatusMessage::eStatus => {

        //                 },
        //             }
        //         },
        //         Err(e) => {
                    // if e==mpsc::TryRecvError::Disconnected {
                    //     println!("Failed to receive: {:?}", e);
                    // }
        //         },
        //     }
        // }
    }

    pub fn set_current_coords(&mut self, x: f32, y: f32, z: f32) {
        self.current_coords.x = x;
        self.current_coords.y = y;
        self.current_coords.z = z;
    }

    pub fn set_target_coords(&mut self, target_pos: CncCoordinates) {
        self.target_coords = target_pos;

        match self.connection.send(ECncCtrlMessage::eTargetPosition(self.target_coords.clone())) {
            Ok( () ) => {
                println!("Message sent");
            }, 
            Err(e) => {
                println!("Failed to send a message: {:?}", e);
            }
        }

        // if let Some(ref tx) = self.o_tx {
        //     match tx.send( ECncCtrlMessage::eTargetPosition(self.target_coords.clone()) ) {
        //         Ok( () ) => {
        //             println!("Message sent");
        //         }, 
        //         Err(e) => {
        //             println!("Failed to send a message: {:?}", e);
        //         }
        //     }
        // } else {
        //     println!("Cant send target position: Connection not set up!");
        // }
    }

    pub fn get_target_coords(&self) -> CncCoordinates {
        self.target_coords.clone()
    }

    pub fn set_channels(&mut self, tx: mpsc::Sender<ECncCtrlMessage>, rx: mpsc::Receiver<ECncStatusMessage>) {
        // self.o_tx = Some(tx);
        // self.o_rx = Some(rx);
        self.e_cnc_ctrl_state = ECncCtrlState::eConnected;
    }
    pub fn set_connection(&mut self, connection: CncConnection<ECncCtrlMessage, ECncStatusMessage>) {
        self.connection = connection;
        self.e_cnc_ctrl_state = ECncCtrlState::eConnected;
    }
    pub fn quit(&mut self) {
        match self.connection.send(ECncCtrlMessage::eQuit) {
            Ok( () ) => {

            },
            Err(e) => {
                println!("Cant send quit message: {}", e);
            }
        }
        self.e_cnc_ctrl_state = ECncCtrlState::eOffline;
    }
}