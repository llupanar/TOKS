mod mainwindow;

fn main() {
	let portnames = mainwindow::serport::search_available_pair();
	let port_write_dev=serialport::new(portnames.1, 9600)
	.parity(serialport::Parity::None)
	.open()
	.expect("Failed to open serial port");
	
	let port_read_dev = serialport::new(portnames.0, 9600)
	.parity(serialport::Parity::None)
	.open()
	.expect("Failed to open serial port");
	
	
	let mut options = eframe::NativeOptions::default();
	options.resizable=false;
	eframe::run_native(
		"Serial Port Communication Program",
		options,
		Box::new(|cc| Box::new(mainwindow::ProgramMainApp::new(cc,port_write_dev, port_read_dev,portnames.2))),
	);
}

