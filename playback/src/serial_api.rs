extern crate serialport;

pub struct SerialApi {
    port: Option<Box<serialport::SerialPort>>
}

impl SerialApi {
    fn connect(&mut self) {
        let port_name = "/dev/tty.wchusbserial14510";
        match serialport::open(port_name) {
            Ok(port) => {
                self.port = Some(port);
                info!("Port {} opened successfuly!", port_name);
            },
            Err(e) => {
                info!("Failed to open port {}. Error: {}", port_name, e);
            }
        }
    }
}

unsafe impl Send for SerialApi { }