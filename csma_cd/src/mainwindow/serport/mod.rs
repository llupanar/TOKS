use eframe::egui;
use std::time::{ Duration, Instant };

pub struct SerialConnection{
	pub self_read:u32,
	pub self_write:u32,
	pub other_read:u32,
	pub other_write:u32,
}
impl SerialConnection {
	fn default() -> Self {
		Self {
			self_read:21,
			self_write:20,
			other_read:11,
			other_write:10,
		}
	}
	fn change_pair ()->Self{
		Self {
			self_read:11,
			self_write:10,
			other_read:21,
			other_write:20,
		}
	}
}

pub fn search_available_pair()->(String,String,SerialConnection){
	let mut portname_write_dev= "";
	let mut portname_read_dev= "";
	let mut serial_connection_inf=SerialConnection::default();
	
	let mut flag_search_ports = false;
	let start = Instant::now();
	let error_time = Duration::new(5, 0);
	while flag_search_ports!=true{
		if error_time<start.elapsed(){
			let mut options = eframe::NativeOptions::default();
			options.resizable=false;
			options.initial_window_size=Some(egui::Vec2::new(150.0,120.0));
			eframe::run_native(
				"Serial Port ERROR",
				options,
				Box::new(|cc| Box::new(ErrorConnectionApp::new(cc))),
			);
			std::process::exit(1);
		}
		match check_open_port("/tmp/ttyS20".to_string(),serialport::Parity::None){
			Ok(_port) => { 
				match check_open_port("/tmp/ttyS21".to_string(), serialport::Parity::None){
					Ok (_port)=> {
						portname_write_dev ="/tmp/ttyS20";
						portname_read_dev = "/tmp/ttyS21";
						flag_search_ports=true;
					}
					Err(_err)=> flag_search_ports=false ,
				}
			}
			Err(_err) => {
				match check_open_port ("/tmp/ttyS10".to_string(),serialport::Parity::None){
					Ok(_port) => {
						match check_open_port("/tmp/ttyS11".to_string(),serialport::Parity::None){
							Ok(_port)=>{
								portname_write_dev ="/tmp/ttyS10";
								portname_read_dev = "/tmp/ttyS11";
								serial_connection_inf=SerialConnection::change_pair();
								flag_search_ports=true;
							}
							Err(_err)=>flag_search_ports=false,
						}
					}
					Err(_err)=> flag_search_ports=false,
				}
			}
		}
	}
	(portname_read_dev.to_string(),portname_write_dev.to_string(),serial_connection_inf)
}
fn check_open_port(port_name:String,parity:serialport::Parity)-> Result<Box<dyn serialport::SerialPort>, serialport::Error>{
	let port = serialport::new(port_name, 9600)
	.parity(parity)
	.open()?;
	Ok(port)
}
struct ErrorConnectionApp {}
impl ErrorConnectionApp{
	fn new (_cc: &eframe::CreationContext<'_>)->Self{
		Self{}
	}
}

impl eframe::App for ErrorConnectionApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.label(egui::widget_text::RichText::new("\n\n\n⚠ No available ports").color(egui::Color32::from_rgb(255,56,83)));
			
		});
	}
}
fn write_to_port_data(data:String,tx:std::sync::mpsc::Sender<String>,port: &mut Box <dyn serialport::SerialPort>){
	let mut is_byte_send = false; 
	let mut collision_count:usize = 0;
	let mut sdata = "".to_string();
	for (i,byte) in data.chars().enumerate(){
		loop{
			let p = rand::thread_rng().gen_range(1..=4);
			if p==1||p==2 {break;}
		}
		is_byte_send=false;
		collision_count = 0;
		while is_byte_send == false {
			let mut send_buffer = [0; 4];
			let send_symbol = byte.encode_utf8(&mut send_buffer);
			port.write(send_symbol.as_bytes());
			let p = rand::thread_rng().gen_range(1..=4);
			sleep(Duration::from_millis(1000)); 
			match p {
				4 => {
					collision_count+=1;
					sdata+="c";
					tx.send(sdata.to_string()).unwrap();
					sleep(Duration::from_millis(pow(2,collision_count)+900));
				}
				_=> {
					is_byte_send=true;
					sdata+="\n";
					tx.send(sdata.to_string()).unwrap();
				}
			}
			if collision_count > 10 {
				sdata+=" send error\n";
				break;
			}
		}
	}
	tx.send("!".to_string()).unwrap();	
	port.write("\n".as_bytes());
	
}
