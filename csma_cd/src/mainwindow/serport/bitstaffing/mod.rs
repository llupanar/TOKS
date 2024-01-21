 
 
 pub fn bit_staffing(udata:String)->String{	
	 let mut start:i32=8;
	 let mut end:i32=15;
	 let mut buffer="01110111".to_string();
	 loop{
		 if end>udata.len() as i32 && start!=udata.len() as i32{
			 buffer+=&udata[start as usize..udata.len()].to_string();			
		 }
		 if end > udata.len() as i32  {break;}
		 if udata[start as usize..end as usize].to_string() == "0111011".to_string(){
			 buffer+=&udata[start as usize..end as usize].to_string();
			 buffer+="0";
			 start=start+7;
			 end=end+7;			
		 }
		 else {
			 buffer+=&udata[start as usize..(start + 1)as usize].to_string();			
			 start=start+1;
			 end=end+1;			
		 }
		 
	 }
	 buffer.to_string()
 }
 pub fn debit_staffing(udata:String)->String{
	 let mut start:i32=8;
	 let mut end:i32=15;
	 let mut buffer="01110111".to_string();
	 loop{
		 if end>udata.len() as i32 && start!=udata.len() as i32{
			 buffer+=&udata[start as usize..udata.len()].to_string();			
		 }
		 if end > udata.len() as i32  {break;}
		 if udata[start as usize..end as usize].to_string() == "0111011".to_string(){
			 buffer+=&udata[start as usize..end as usize].to_string();
			 buffer+="b";
			 start=start+8;
			 end=end+8;			
		 }
		 else {
			 buffer+=&udata[start as usize..(start + 1)as usize].to_string();			
			 start=start+1;
			 end=end+1;			
		 }
	 }
	 buffer.to_string()
 }
 
 
 
