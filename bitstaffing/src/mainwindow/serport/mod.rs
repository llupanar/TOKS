mod packet;
mod bitstaffing;

use eframe::egui;
use std::time::{Duration, Instant};
use bitstaffing::{bit_staffing};
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
fn encode_packet (packet: packet::Packet)->String{
	let mut send_packet=packet.as_string();
	if send_packet[9..send_packet.len()]
		.to_string()
		.contains("0111011"){
		send_packet=bit_staffing(send_packet,7,"0");			
	}
	send_packet
}

pub fn write_to_port(port: &mut Box <dyn serialport::SerialPort>, mut data:String){  //port,data
	data = data[0..data.len() - 1 as usize].to_string();
	let packet_number:f32=data.len() as f32/16 as f32;
	let mut start :usize = 0;
	let mut end:usize = 16;
	for _i in 0..packet_number.floor() as i32{
		let packet=packet::Packet::new(data[start..end].to_string(),"1111".to_string());
		let send_packet=encode_packet(packet);
		port.write(send_packet.as_bytes()); //send
		start=end;
		end=start+16;
	}
	if packet_number!=1.0{
		end=data.len();
		let x = (end-start) as i32;
		let blen = format!("{x:b}").to_string();
		let mut ulen = "".to_string();
		let null_count =  4 - blen.len() as i32;
		for _i in 0..null_count {
			ulen+="0";
		}
		ulen+=&blen;
		
		let packet=packet::Packet::new(data[start..end].to_string(),ulen.to_string());
		let mut send_packet=encode_packet(packet);		
		port.write(send_packet.as_bytes()); //send
	}
}
fn get_data_from_frame(frame:String)->String{
	let mut udata="".to_string();
	
	if frame[16..20].to_string() == "0000"{
		udata+="\n";
		return udata.to_string()
	}
	
	let buffer=frame[20..frame.len()-1 as usize].to_string();
	for (_i,symbol) in buffer.chars().enumerate(){
		if symbol == 'b' {continue;}
		udata+=&symbol.to_string();
	}
	udata+="\n";
	udata.to_string()
}

pub fn read_from_port(port: &mut Box <dyn serialport::SerialPort>)->(String, String,bool){
	let mut data_in_port = "".to_string();
	port.read_to_string(&mut data_in_port);
	let mut is_port_empty = true;
	let mut received_frame = "".to_string();
	let mut received_data = "".to_string();
	if data_in_port=="".to_string(){
		return (received_frame,received_data,is_port_empty)
	}
	is_port_empty=false;	
	let mut start:usize=0;
	let mut start_frame:usize=0;
	let mut end:usize=8;
	let mut is_new_frame = false;
	loop{
		if end>data_in_port.len(){
			received_frame = bit_staffing(data_in_port[start_frame..].to_string(),8,"b");
			let tmp = get_data_from_frame(received_frame.clone());
			received_data+=&tmp;
			break;
		}
		if data_in_port[start..end] == "01110111" && !is_new_frame {
			start_frame=start;
			is_new_frame=true;
			start=start+8;
			end=end+8;
		}
		else if data_in_port[start..end].to_string() == "01110111".to_string()&& is_new_frame {
			received_frame = bit_staffing(data_in_port[start_frame..start].to_string(),8,"b");
			let tmp = get_data_from_frame(received_frame);
			received_data+=&tmp;
			is_new_frame=false;
			start_frame=start;
			start=start+8;
			end=end+8;			
		}
		else {
			start=start+1;
			end=end+1;			
		}
	}	
	(received_frame,received_data,is_port_empty)
}


