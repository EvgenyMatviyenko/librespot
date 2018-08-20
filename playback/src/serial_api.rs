extern crate serialport;

pub struct SerialApi {
    port: Box<serialport::SerialPort>
}

impl SerialApi {
    pub fn new() -> Option<SerialApi> {
        let port_name = "/dev/tty.wchusbserial14310";
        match serialport::open(port_name) {
            Ok(port) => {
                info!("Port {} opened successfuly!", port_name);
                Some(SerialApi {
                    port: port
                })
            },
            Err(e) => {
                info!("Failed to open port {}. Error: {}", port_name, e);
                None
            }
        }
    }

    pub fn write(&mut self, command: &str) {
        &self.port.write(command.as_bytes());
    }
}

unsafe impl Send for SerialApi { }

unsafe impl Sync for SerialApi { }