 
 pub struct Packet {
	 data : String,
	 flag : String,
	 destination_address: String,
	 source_address: String,
	 length: String,
	 fcs: String
 }
 impl Packet {
	 pub fn new(udata:String,ulen:String)->Self{
		 Self {
			 data : udata,
			 flag : "01110111".to_owned(),
			 destination_address: "0000".to_owned(),
			 source_address: "0000".to_owned(),
			 length: ulen.to_owned(),
			 fcs: "0".to_owned()
		 }
	 }
pub	 fn as_string(&self)->String{
		 let mut complete_packet="".to_string();
		 complete_packet+=&self.flag;
		 complete_packet+=&self.destination_address;
		 complete_packet+=&self.source_address;
		 complete_packet+=&self.length;
		 complete_packet+=&self.data;
		 complete_packet+=&self.fcs;
		 complete_packet
	 }
 } 
 
 
