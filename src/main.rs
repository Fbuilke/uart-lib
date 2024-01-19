mod serial_com;

fn main() {
    match serial_com::get_serial_port_available_list() {
        Ok(ports) => {
            for port in ports {
                println!("{:?}", port);
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
