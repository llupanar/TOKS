pub mod serport;
use eframe::egui;
use std::time::{Duration};
use std::thread::sleep;
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::channel;
use num::pow;
pub struct ProgramMainApp {
	port_write: Box <dyn serialport::SerialPort>,
	port_read: Box <dyn serialport::SerialPort>,
	msg_debug:String,
	user_input:String,
	user_input_prev:String,
	received_frame:String,
	received_data:String,
	serialport_connection:serport::SerialConnection,
	distortion_count: i32,
	is_next_data:bool,
	parity:serialport::Parity,
	tx: std::sync::mpsc::Sender<String>,
	rx: std::sync::mpsc::Receiver<String>,
	is_data_send:bool,
	
}
impl ProgramMainApp {
	pub fn new(cc: &eframe::CreationContext<'_>, uport_write: Box <dyn serialport::SerialPort>,
 		   uport_read: Box <dyn serialport::SerialPort>,connection_inf:serport::SerialConnection) -> Self {
			   setup_custom_fonts(&cc.egui_ctx);
			   let (a,b)=channel::<String>();
			   Self {
				   port_write:uport_write,
				   port_read:uport_read,
				   msg_debug:"".to_owned(),
				   user_input:"".to_owned(),
				   user_input_prev:"".to_owned(),
				   received_frame:"".to_owned(),
				   received_data:"".to_owned(),
				   serialport_connection:connection_inf,
				   distortion_count: 0,
				   is_next_data:false,
				   is_data_send:false,
				   parity:serialport::Parity::None,
				   tx: a,
				   rx: b,
			   }
		   }
}
impl eframe::App for ProgramMainApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		
		//============================================================================
		//*********************************INPUT********************************		
		//============================================================================
		
		egui::TopBottomPanel::top("").show(ctx, |ui| {
			ui.vertical(|ui| {
				ui.label("Send data to another device:\n");
				let scroll_delta = egui::Vec2::ZERO;
				egui::ScrollArea::vertical().max_height(80.00).show(ui, |ui| {
					ui.scroll_with_delta(scroll_delta);
					ui.add_sized([450.0,80.0], egui::TextEdit::multiline(&mut self.user_input));
					
				});
				for (i, symbol) in self.user_input.chars().enumerate() {
					if (symbol>'1'||symbol<'0')&&symbol!='\n' {
						self.user_input=self.user_input[0..i].to_string();
						break;
					} 
				}
				if self.is_next_data==true && self.user_input_prev.len()<self.user_input.len(){
					self.user_input =self.user_input[self.user_input.len() - 1 as usize..self.user_input.len()].to_string();
					self.is_next_data=false;
				}
				if  ui.input().key_pressed(egui::Key::Enter) {
					self.received_frame="".to_string();
					let mut buffer ="".to_string();
					let mut tx = self.tx.clone();
					buffer = self.user_input[0..self.user_input.len()-1 as usize].to_string();
					self.is_data_send = true;
					let mut port = self.port_write.try_clone().unwrap();
					thread::spawn(move || {
						serialport::write_to_port_data(buffer,tx,&mut port);
					});					
					self.user_input_prev=self.user_input.to_string();
					self.is_next_data=true;
				}	
				if ui.button("clear").clicked (){
					self.user_input = "".to_string();
				}
				ui.label("");
			});
		});
		
		
		//============================================================================
		//*********************************INFORMATION********************************				
		//============================================================================
		

		egui::SidePanel::left("").max_width(160.0).show(ctx, |ui| {
			let _scroll_delta = egui::Vec2::ZERO;
			ui.label(" Serial port connection:\n");
			ui.add(egui::Label::new(egui::RichText::new(format!("ttyS{} ➡ ttyS{}",self.serialport_connection.self_write,
																self.serialport_connection.other_read)).size(12.0)));
			ui.add(egui::Label::new(egui::RichText::new(format!("ttyS{} ⬅ ttyS{}",self.serialport_connection.self_read,
																self.serialport_connection.other_write)).size(12.0)));
			ui.label(" \nBYTE SEND STATUS:");
			let scroll_delta = egui::Vec2::ZERO;
			egui::ScrollArea::vertical().max_height(40.00).show(ui, |ui| {
				ui.scroll_with_delta(scroll_delta);
				if self.is_data_send == true {
					let received = self.rx.try_recv();
					let mut buffer = "".to_string();
					match received {
						Ok(msg) =>{
							buffer = msg;
							if buffer != "!"{self.received_frame=buffer;}
							else {self.is_data_send=false;}
						}
						Err(err) => self.msg_debug = "no_info".to_string(),
					}
				}
				ui.add_sized([130.0,63.0], egui::Label::new(format!("{}",self.received_frame)));
			});
			ui.separator();
			
			//============================================================================
			//*********************************PARITY********************************		
			//===========================================================================
			
			ui.label("             Parity");
			
			if ui.add(egui::RadioButton::new(self.parity == serialport::Parity::None, "None")).clicked() {
				self.parity = serialport::Parity::None;
			}
			if ui.add(egui::RadioButton::new(self.parity == serialport::Parity::Odd, "Odd")).clicked() {
				self.parity = serialport::Parity::Odd;				
			}
			if ui.add(egui::RadioButton::new(self.parity == serialport::Parity::Even, "Even")).clicked() {
				self.parity = serialport::Parity::Even;
			}
			egui::Context::request_repaint(&ctx);
			
		});
		
		//============================================================================
		//*********************************OUTPUT********************************		
		//===========================================================================		
		
		egui::CentralPanel::default().show(ctx, |ui| {
			
			let scroll_delta = egui::Vec2::ZERO;
			let received_infomation = serport::read_from_port(&mut self.port_read);
			if received_infomation.1 == false {
				if received_infomation.0=="!"{ self.msg_debug = "collision".to_string();}
				if received_infomation.0 !="!" { println!("{}",received_infomation.0);self.received_data+=&received_infomation.0;}
			}
			egui::ScrollArea::vertical().max_height(130.00).show(ui, |ui| {
				ui.scroll_with_delta(scroll_delta);
				ui.add_sized([310.0,160.0], egui::Label::new(format!("{}",self.received_data.to_string())));
			});
			ui.label("Received data");
			ui.separator();
			ui.add_sized([310.0,40.0], egui::Label::new(format!("{}",self.msg_debug.to_string())));
			ui.label("                                                                       debugging");
			
			egui::Context::request_repaint(&ctx);
			
		});
		egui::Context::request_repaint(&ctx);
		
	}
}

fn setup_custom_fonts(ctx: &egui::Context) {
	let mut fonts = egui::FontDefinitions::default();
	fonts.font_data.insert(
		"program_font".to_owned(), egui::FontData::from_static(include_bytes!(
			"content/font.ttf")),
	);
	fonts
	.families
	.entry(egui::FontFamily::Proportional)
	.or_default()
	.insert(0, "program_font".to_owned());
	fonts
	.families
	.entry(egui::FontFamily::Monospace)
	.or_default()
	.push("program_font".to_owned());
	ctx.set_fonts(fonts);
}


