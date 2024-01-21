pub mod serport;
use eframe::egui;

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
	
}
impl ProgramMainApp {
	pub fn new(cc: &eframe::CreationContext<'_>, uport_write: Box <dyn serialport::SerialPort>,
		   uport_read: Box <dyn serialport::SerialPort>,connection_inf:serport::SerialConnection) -> Self {
			   setup_custom_fonts(&cc.egui_ctx);
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
				   parity:serialport::Parity::None,
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
					serport::write_to_port(&mut self.port_write,self.user_input.clone());
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
		
		let mut size = self.received_frame.len() as f32;	
		if size<10.0 { size = 160.0;}
		else if size>30.0{ 
			size=(size+1.0)*6.5;
		}
		else {size = size*7.5;}
		egui::SidePanel::left("").max_width(size).show(ctx, |ui| {
			let _scroll_delta = egui::Vec2::ZERO;
			ui.label(" Serial port connection:\n");
			ui.label(format!("ttyS{} ➡ ttyS{}",self.serialport_connection.self_write,self.serialport_connection.other_read));
			ui.label(format!("ttyS{} ⬅ ttyS{}\n",self.serialport_connection.self_read,self.serialport_connection.other_write));
			ui.label(" Recieved frame:");
			ui.horizontal_wrapped(|ui| {
				ui.style_mut().wrap = Some(true);
				ui.spacing_mut().item_spacing.x = 0.0;
				let is_fcs = self.received_frame.contains("b");
				match is_fcs {
					true => print_frame_bstaffing (ui, self.received_frame.clone()),
					false => print_frame_hamming (ui, self.received_frame.clone(),self.distortion_count),
				}
				
			});	
			ui.separator();
			
			//============================================================================
			//*********************************PARITY********************************		
			//===========================================================================
			
			ui.label("             Parity\n");
			
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
			if received_infomation.3 == false {
				self.received_frame = received_infomation.0;
				self.received_data+=&received_infomation.1;
				self.distortion_count = received_infomation.2;
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
			"text.ttf")),
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


fn print_frame_bstaffing(ui: &mut egui::Ui,frame:String){
	for (_i, symbol) in frame.chars().enumerate() {
		if symbol=='b' {
			ui.add(egui::Label::new(egui::RichText::new("0").color(egui::Color32::KHAKI).size(11.0)));
		}
			else {
				ui.add(egui::Label::new(egui::RichText::new(format!("{}",symbol)).size(11.0)));
			}
	}
}
fn print_frame_hamming(ui: &mut egui::Ui,frame:String,distortion_count:i32){
	let mut is_fcs = false;
	if frame == "".to_string(){
		return
	}
	for (_i, symbol) in frame.chars().enumerate() {
		if symbol=='|' {
			is_fcs=true;
			continue;
		}
		if is_fcs == true {
			ui.add(egui::Label::new(egui::RichText::new(format!("{}",symbol)).color(egui::Color32::from_rgb(0,127,255)).size(11.0)));
		}
		else {
			ui.add(egui::Label::new(egui::RichText::new(format!("{}",symbol)).size(11.0)));
		}
	}
	ui.add(egui::Label::new(egui::RichText::new(format!("\n{} distortion(s)",distortion_count)).size(11.0)));
	
}
