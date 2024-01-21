 use num::pow;
 use rand::Rng;
 
 fn hamming_parity_counting(data:String, position:u32, size:u32)->u32{
	 let mut is_odd: bool = false; 
	 let mut i = position - 1;
	 while i < size {
		 for j in i as usize .. (i + position) as usize {
			 if j as u32 >size - 1  {break;}
			 let index = j;
			 if data[index as usize..(index+1) as usize] == "1".to_string(){
				 is_odd=!is_odd;
			 }			
		 }		
		 i = i + 2 * position;
	 }
	 is_odd as u32
 }
 fn data_parity(mut frame:String)->String{
	 frame = frame[20..].to_string();	 
	 let mut is_odd:bool = false;
	 for (_i,symbol) in frame.chars().enumerate() {
		 if symbol=='1' {is_odd=!is_odd;}
	 }	 
	 let parity = is_odd as i32;
	 parity.to_string()
 }
 
 fn make_hamming_string (data:String, mut size:i32)->String{
	 let mut hamming_string = "".to_string();
	 let mut grade = 0;
	 let mut index = 0;
	 size+=data.len() as i32;
	 for i in 0 .. size {
		 if i == pow(2, grade as usize) as i32 - 1 {
			 hamming_string+="x";
			 grade+=1;
		 }
		 else {
			 hamming_string+=&data[index as usize ..(index+1)as usize];
			 index+=1;
		 }
	 }
	 hamming_string
 }
 pub fn encode_hamming(udata: String)->String{
	 let mut fcs = "".to_string();
	 let mut is_odd:bool = false;
	 let mut fcs_size:u32 = 0;
	 while pow(2, fcs_size as usize) as u32 - (fcs_size +1) < udata.len() as u32 {
		 fcs_size+=1;
	}
	 let full_size = fcs_size+udata.len() as u32;
	 let hamming_string = make_hamming_string(udata.clone(),fcs_size as i32);
	 for i in 0.. fcs_size {
		 let position = pow(2, i as usize) as u32;
		 let value = hamming_parity_counting(hamming_string.clone(),position, full_size);
		 fcs+= &value.to_string();
	 }
	 for (_i,symbol) in udata.chars().enumerate() {
		 if symbol=='1' {is_odd=!is_odd;}
	 }
	 let parity = is_odd as i32;
	 fcs+=&parity.to_string();
	 fcs.to_string()
 }
 
 // add '|' to highlight the field fcs with color
fn frame_for_print (rframe: String,fcs_index: usize)-> String{
	 let mut frame = "".to_string();
	 frame+=&rframe[..rframe.len() - fcs_index];
	 frame+="|";
	 frame+=&rframe[rframe.len() - fcs_index..];
	 
	 frame
}

 pub fn decode_hamming(rframe:String)->(String,String,i32){
	 if rframe[16..20].to_string() == "0000"{
		 return (rframe.clone(),rframe,0)
	 }
	 let mut frame = "".to_string();
	 let mut frame_with_error = "".to_string();
	 let mut fcs = "".to_string();
	 let mut fcs_size:usize = 1;
	 let mut data = rframe[20..rframe.len()].to_string();
	 match data.len(){
		 1 => return (frame.clone(), frame, 0),
		 3 => fcs_size = 3,
		 _=> {
			 while pow(2,fcs_size-1) < data.len() as i32 {
			 fcs_size+=1;
		 }
	   }
	}
	fcs = data[data.len() - fcs_size..data.len()].to_string();
	data = data[0..data.len()-fcs_size].to_string();	
	data=random_distortion(data);
	frame_with_error+=&rframe[..20];
	frame_with_error+=&data;
	frame_with_error+=&fcs;
	let mut check_fcs = encode_hamming (data.clone());
	
	if check_fcs==fcs { 
		 frame=frame_for_print(rframe,fcs_size);
		 return (frame.clone(), frame, 0)  
	}
	 else {
		 let mut index = 0;
		 for i in 0..fcs_size - 1  {
			 if fcs[i..i+1]!=check_fcs[i..i+1]{
				 index+=pow(2,i) as i32;
			 }
		 }	
		 let mut hamming_string = make_hamming_string(data.clone(),check_fcs.len() as i32 - 1);		 
		 if index==1||index==2||index==4||index==8||index==16||index as usize>hamming_string.len() {
			 frame = frame_for_print(frame_with_error,fcs_size as usize); //error frame
			 return (frame.clone(),frame,2);
		 }
		 
		 index=index-1;
		 let mut changed_frame = rframe[..20].to_string(); 
		 //---------changing the distorted bit----------
		 if hamming_string[index as usize .. (index+1) as usize] == "0".to_string() {
			 hamming_string.replace_range(index as usize..(index+1) as usize,"1");
		}
		 else {
			 hamming_string.replace_range(index as usize..(index+1) as usize,"0");
		}
		//---------------------------------------------
		for (i,symbol) in hamming_string.chars().enumerate(){
			 if symbol!='x' { 
				 changed_frame+=&hamming_string[i..i + 1 as usize];
			}
		 }
		 if data_parity(changed_frame.clone())!=fcs[fcs.len()-1 as usize..]{
			 frame_with_error = frame_for_print(frame_with_error,fcs_size as usize);
			 frame = frame_for_print(changed_frame,fcs_size as usize);
			 (frame_with_error, frame,2)
		 }
		 else {			 
			 changed_frame+=&fcs;
			 frame_with_error = frame_for_print(frame_with_error,fcs_size as usize);
			 frame = frame_for_print(changed_frame,fcs_size as usize);
 			 (frame_with_error,frame,1)
		 }
	 }
	
 }
 
 fn random_change (mut data:String)->String{
	 let index = rand::thread_rng().gen_range(0..=data.len() - 1 as usize);
	 if data[index as usize .. (index+1) as usize] == "0".to_string() {
		 data.replace_range(index as usize..(index+1) as usize,"1");
	}
	 else {
		 data.replace_range(index as usize..(index+1) as usize,"0");
	}
	 data
 }
 fn random_distortion(mut data:String)->String{
	 let p = rand::thread_rng().gen_range(1..=4);
	 match p {
		 1 => data=random_change(data),
		 3=> data=random_change(data),
		 4 => {
			 println!("2");
			 data=random_change(data);
			 data=random_change(data);
		 }
		 _=> println!("ok")
	 }
	 data
 }
 
