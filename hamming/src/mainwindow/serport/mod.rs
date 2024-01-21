mod packet;
mod bitstaffing;
mod hamming;
use eframe::egui;
use std::time::{ Duration, Instant };
use bitstaffing::{ bit_staffing,debit_staffing };
use hamming::{ encode_hamming, decode_hamming };

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
	let mut bit_staffing_check = "".to_string();
	let mut fcs = "".to_string();
	bit_staffing_check+=&send_packet[9..send_packet.len()];
	if bit_staffing_check.contains("0111011"){
		send_packet=bit_staffing(send_packet);
		fcs+="0";
	}
	else {
		let data = packet.data;
		fcs = encode_hamming(data);
	}
	send_packet+=&fcs;
	send_packet
}
pub fn write_to_port(port: &mut Box <dyn serialport::SerialPort>, mut data:String){  //port,data
	data = data[0..data.len() - 1 as usize].to_string();
	let packet_number:f32=data.len() as f32/16 as f32;
	let mut start :i32 = 0;
	let mut end:i32 = 16;
	for _i in 0..packet_number.floor() as i32{
		let packet=packet::Packet::new(data[start as usize..end as usize].to_string(),"1111".to_string());
		let send_packet=encode_packet(packet);
		println!("send packet {}", send_packet);
		port.write(send_packet.as_bytes()); //send
		start=end;
		end=start+16;
	}
	if packet_number!=1.0{
		end=data.len() as i32;
		let x = end-start;
		let blen = format!("{x:b}").to_string();
		let mut ulen = "".to_string();
		let null_count =  4 - blen.len() as i32;
		for _i in 0..null_count {
			ulen+="0";
		}
		ulen+=&blen;
		
		let packet=packet::Packet::new(data[start as usize..end as usize].to_string(),ulen.to_string());
		let send_packet=encode_packet(packet);	
		println!("send packet {}", send_packet);
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
		if symbol=='|' {break;}
		udata+=&symbol.to_string();
	}
	udata+="\n";
	udata.to_string()
}
fn decode_frame (frame: String)->(String,String,i32){
	if frame[9..].contains("0111011"){
		let received_frame = debit_staffing(frame);
		return (received_frame.clone(), received_frame,0)
	}
	let decode_result = decode_hamming(frame);
	(decode_result.0, decode_result.1, decode_result.2)
}
pub fn read_from_port(port: &mut Box <dyn serialport::SerialPort>)->(String, String,i32,bool){
	let mut data_in_port = "".to_string();
	port.read_to_string(&mut data_in_port);
	let mut is_port_empty = true;
	let mut received_frame = "".to_string();
	let mut received_data = "".to_string();
	let mut distortion_count:i32 = 0;
	if data_in_port=="".to_string(){
		return (received_frame,received_data,distortion_count,is_port_empty)
	}
	is_port_empty=false;
	println!("DATA_IN_PORT\n{}\n",data_in_port);
	
	let mut start:i32=0;
	let mut start_frame:i32=0;
	let mut end:i32=8;
	let mut is_new_frame = false;
	loop{
		if end>data_in_port.len() as i32 {
			let decode_result = decode_frame(data_in_port[start_frame as usize..data_in_port.len()].to_string());
			println!("LAST FRAME{}\n",data_in_port[start_frame as usize..data_in_port.len()].to_string());
			
			received_frame = decode_result.0;
			distortion_count = decode_result.2;
			let tmp = get_data_from_frame(decode_result.1);
			received_data+=&tmp;
			break;
		}
		if data_in_port[start as usize..end as usize].to_string() == "01110111".to_string()&& !is_new_frame {
			start_frame=start;
			is_new_frame=true;
			start=start+8;
			end=end+8;
		}
		else if data_in_port[start as usize..end as usize].to_string() == "01110111".to_string()&& is_new_frame {
			let decode_result = decode_frame(data_in_port[start_frame as usize..start as usize].to_string());
			println!("FIRST FRAME {}\n",data_in_port[start_frame as usize..start as usize].to_string());
			
			received_frame = decode_result.0;
			distortion_count = decode_result.2;
			let tmp = get_data_from_frame(decode_result.1);
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
	(received_frame,received_data, distortion_count, is_port_empty)
}


