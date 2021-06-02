
use std::sync::mpsc::{self, TryRecvError};
use std::net::TcpStream;
use std::{io::prelude::*, net::Shutdown, time::Duration};
use std::io;
use crate::thread_pool::ThreadPool;

use crate::cnc_msg::{ECncCtrlMessage, ECncStatusMessage};

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
    pool:   ThreadPool,
}

impl CncConnectionManager {
    pub fn new() -> Self {
        CncConnectionManager{
            pool    : ThreadPool::new(2),
        }
    }

    pub fn run(&mut self, stream: TcpStream) -> CncConnection<ECncCtrlMessage, ECncStatusMessage> {
        let (other_end, own_end) = CncConnection::new_connected_pair();
        self.pool.execute( move || {
            CncConnectionManager::run_tcp(stream, own_end);
        });

        other_end
    }

    fn send_msg_tcp(stream: &mut TcpStream, msg: ECncCtrlMessage) {
        match msg.bin_serialize() {
            Ok(payload) => {
                match stream.write(payload.as_slice()) {
                    Ok(bytes_written) => {
                        if bytes_written!=payload.len() {
                            println!("Failed to send all bytes: sent {} of {}", bytes_written, payload.len());
                        } else {
                            println!("Sent {:?}", payload);
                        }
                        stream.flush().unwrap();
                    },
                    Err(e) => {
                        println!("Error sending: {:?}", e);
                    }
                }
            },
            Err(e) => {
                println!("Failed to serialize the message: {:?}", e);
            }
        }
    }

    fn run_tcp(mut stream: TcpStream, cnc: CncConnection<ECncStatusMessage, ECncCtrlMessage>) {
        let mut running = true;
        stream.set_read_timeout( Some(Duration::from_millis(100)) ).unwrap();
        let mut ab_recv_buffer: [u8; 512] = [0; 512];
        while running {
            match cnc.receive() {
                Ok(Some(ECncCtrlMessage::eQuit)) => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    running = false;
                },
                Ok(msg) => {
                    if let Some(msg) = msg {
                        CncConnectionManager::send_msg_tcp(&mut stream, msg);
                        // CncConnectionManager::send_msg_tcp(&mut stream, msg);
                    }
                },
                Err(e) => {
                    if e==mpsc::TryRecvError::Disconnected {
                        println!("Failed to receive: {:?}", e);
                        running = false;
                    } 
                },
            }
    
            match stream.read( &mut ab_recv_buffer ) {
                Ok( res ) => {
                    if res>0 {
                        // println!("Received {} bytes", res);
                        let status_type = ab_recv_buffer[0];
                        if status_type==0 {
                            match bincode::deserialize(&ab_recv_buffer[1..]) {
                                Ok(res) => {
                                    match cnc.send(ECncStatusMessage::eCurrentPosition(res)) {
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