
use std::sync::mpsc::{self, TryRecvError};
use std::net::TcpStream;
use std::{io::prelude::*, net::Shutdown, time::Duration};
use std::io;

use crate::cnc_msg::{CncCoordinates, ECncCtrlMessage, ECncStatusMessage};

pub trait Message {

}

pub struct CncConnection<T, U> {
    o_tx            : Option<mpsc::Sender<T>>,
    o_rx            : Option<mpsc::Receiver<U>>,
}

impl<T, U> CncConnection<T, U> {
    pub fn new() -> Self {
        CncConnection{
            o_tx : None,
            o_rx : None,
        }
    }
    pub fn new_connected_pair() -> (Self, CncConnection<U, T>) {
        let (tx_1, rx_1) = mpsc::channel();
        let (tx_2, rx_2) = mpsc::channel();
        (
            CncConnection{
                o_tx : Some(tx_1),
                o_rx : Some(rx_2),
            },
            CncConnection{
                o_tx : Some(tx_2),
                o_rx : Some(rx_1),
            }
        )
    }

    pub fn send(&self, msg: T) -> Result<(), String> {
        if let Some(ref tx) = self.o_tx {
            match tx.send( msg ) {
                Ok( () ) => {
                    Ok(())
                }, 
                Err(e) => {
                    Err(format!("Failed to send a message: {:?}", e))
                }
            }
        } else {
            Err(format!("Can't send: Connection not set up!"))
        }
    }

    pub fn receive(&self) -> Result<Option<U>, TryRecvError> {
        if let Some(ref rx) = self.o_rx {
            match rx.try_recv() {
                Ok( res ) => {
                    Ok(Some(res))
                },
                Err(e) => {
                    // println!("Failed to receive: {:?}", e);
                    Err(e)
                },
            }
        } else {
            println!("Can't receive: Connection not set up!");
            Ok(None)
        }
    }
}

pub struct CncConnectionManager {
    stream: TcpStream,
    cnc : CncConnection<ECncStatusMessage, ECncCtrlMessage>,
    running : bool,
}

impl CncConnectionManager {
    pub fn new(stream: TcpStream, connection: CncConnection<ECncStatusMessage, ECncCtrlMessage>) -> Self {
        CncConnectionManager{
            stream,
            cnc : connection,
            running : false,
        }
    }

    fn send_msg_tcp(&mut self, msg: ECncCtrlMessage) {
        let u_type_id = msg.get_type_id();
        match msg {
            ECncCtrlMessage::eTargetPosition(coords) => {
                match bincode::serialize(&coords) {
                    Ok(mut vec) => {
                        let payload = {
                            let mut temp_vec = Vec::from( [u_type_id; 1] );
                            temp_vec.append(&mut vec);
                            temp_vec
                        };
                        match self.stream.write(payload.as_slice()) {
                            Ok(bytes_written) => {
                                if bytes_written!=payload.len() {
                                    println!("Failed to send all bytes: sent {} of {}", bytes_written, vec.len());
                                } else {
                                    println!("Sent {:?}", payload);
                                }
                                self.stream.flush().unwrap();
                            },
                            Err(e) => {
                                println!("Error sending: {:?}", e);
                            }
                        }
                    },
                    Err(e) => {
                        println!("Failed to serialize the coords: {:?}", e);
                    }
                }
            },
            ECncCtrlMessage::eQuit => {
                self.stream.shutdown(Shutdown::Both).unwrap();
                self.running = false;
            },
        }
    }

    pub fn run(&mut self) {
        self.running = true;
        self.stream.set_read_timeout( Some(Duration::from_millis(100)) ).unwrap();
        let mut ab_recv_buffer: [u8; 512] = [0; 512];
        while self.running {
            match self.cnc.receive() {
                Ok(msg) => {
                    if let Some(msg) = msg {
                        self.send_msg_tcp(msg);
                    }
                },
                Err(e) => {
                    if e==mpsc::TryRecvError::Disconnected {
                        println!("Failed to receive: {:?}", e);
                        self.running = false;
                    }
                },
            }
            // if let Some(msg) = self.cnc.receive() {
            // }
    
            match self.stream.read( &mut ab_recv_buffer ) {
                Ok( res ) => {
                    if res>0 {
                        // println!("Received {} bytes", res);
                        let status_type = ab_recv_buffer[0];
                        if status_type==0 {
                            match bincode::deserialize(&ab_recv_buffer[1..]) {
                                Ok(res) => {
                                    match self.cnc.send(ECncStatusMessage::eCurrentPosition(res)) {
                                        Ok( () ) => {
        
                                        },
                                        Err(e) => {
                                            println!("Error sending a received message {:?}", e);
                                        },
                                    }
                                },
                                Err(e) => {
                                    println!("Error deserializeing {:?}", *e);
                                },
                            }
                        } else {
                            println!("Unknown status received: {}", status_type);
                        }
                    }
                },
                Err(e) => {
                    if e.kind()!=io::ErrorKind::WouldBlock {
                        println!("Error receiving {:?}", e);
                    }
                },
            }
        }
    }
}