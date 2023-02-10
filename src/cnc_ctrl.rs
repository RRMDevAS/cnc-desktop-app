
use std::sync::mpsc;

use crate::cnc_msg::{CncCoordinates, ECncCtrlMessage, ECncStatusMessage, PIDParams};
use crate::cnc_connection::CncConnection;

enum ECncCtrlState {
    EOffline,
    EConnected,
}


pub struct CncCtrl
{
    e_cnc_ctrl_state    : ECncCtrlState,
    pub target_coords   : CncCoordinates,
    pub current_coords  : CncCoordinates,
    pub pid_params      : [PIDParams; 3],
    connection          : CncConnection<ECncCtrlMessage, ECncStatusMessage>
}

impl CncCtrl {
    pub fn new() -> CncCtrl {
        CncCtrl{
            e_cnc_ctrl_state: ECncCtrlState::EOffline,
            target_coords   : CncCoordinates::new(),
            current_coords  : CncCoordinates::new(),
            pid_params      : [PIDParams::new(), PIDParams::new(), PIDParams::new()],
            connection      : CncConnection::new(),
        }
    }

    pub fn update_status(&mut self) {
        match self.connection.receive() {
            Ok(msg) => {
                if let Some(status) = msg {
                    match status {
                        ECncStatusMessage::ECurrentPosition( current ) => {
                            println!("Received coordinates: {:?}", current.clone());
                            // self.current_coords = current;
                            self.set_current_coords(current.x, current.y, current.z);
                        },
                        ECncStatusMessage::EStatus(status) => {
                            self.set_current_coords(status.axis_status[0].position, status.axis_status[1].position, status.axis_status[2].position);
                        },
                        ECncStatusMessage::EPIDParams(params) => {
                            self.update_pid_params(params);    
                        },
                        ECncStatusMessage::EDisconnected => {
        
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
    }

    pub fn set_current_coords(&mut self, x: f32, y: f32, z: f32) {
        self.current_coords.x = x;
        self.current_coords.y = y;
        self.current_coords.z = z;
    }

    pub fn set_target_coords(&mut self, target_pos: CncCoordinates) {
        self.target_coords = target_pos;

        match self.connection.send(ECncCtrlMessage::ETargetPosition(self.target_coords.clone())) {
            Ok( () ) => {
                println!("Message sent");
            }, 
            Err(e) => {
                println!("Failed to send a message: {:?}", e);
            }
        }
    }
    
    pub fn set_pid_params(&mut self, x_axis: &PIDParams, y_axis: &PIDParams, z_axis: &PIDParams) {
        match self.connection.send(ECncCtrlMessage::EPIDParams([x_axis.clone(), y_axis.clone(), z_axis.clone()])) {
            Ok(()) => {
                println!("Message sent with PID params");
            },
            Err(e) => {
                println!("Failed to send a message with PID params: {:?}", e);
            }
        }
    }
    
    pub fn update_pid_params(&mut self, pid_params: [PIDParams; 3]) {
        self.pid_params = pid_params;
    }

    pub fn get_target_coords(&self) -> CncCoordinates {
        self.target_coords.clone()
    }
    
    pub fn set_connection(&mut self, connection: CncConnection<ECncCtrlMessage, ECncStatusMessage>) {
        self.connection = connection;
        self.e_cnc_ctrl_state = ECncCtrlState::EConnected;
    }
    pub fn quit(&mut self) {
        match self.connection.send(ECncCtrlMessage::EQuit) {
            Ok( () ) => {

            },
            Err(e) => {
                println!("Cant send quit message: {}", e);
            }
        }
        self.e_cnc_ctrl_state = ECncCtrlState::EOffline;
    }
}