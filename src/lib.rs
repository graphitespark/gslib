use std::collections::HashMap;
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};
use std::net::TcpStream;
pub fn safeify(mut text:String) -> String{
    text = text.replace("\\","\\\\");
    text.replace("\"","\\\"")
}
pub struct Item{
    name:String,
    material:String,
    count:i32,
    int_tags:HashMap<String,i32>,
    str_tags:HashMap<String,String>
}
pub struct CCAPI{
    socket:WebSocket<MaybeTlsStream<TcpStream>>,
    scopes:Vec<String>
}
impl CCAPI{
    pub fn connect() -> CCAPI{
        let (mut socket,_) = tungstenite::connect("ws://localhost:31375").expect("Error Connecting to CCAPI");
        let mut scopes: Vec<String> = Vec::new();
        scopes.push(String::from("default"));
        CCAPI {socket,scopes}
    }
    pub fn terminate(){

    }
}
impl Item{
    pub fn new(name:String,material:String,count:i32) -> Item{
        let str_tags: HashMap<String, String> = HashMap::new();
        let int_tags: HashMap<String, i32> = HashMap::new();
        Item {name,material,count,int_tags,str_tags}
    }
    pub fn set_inv(&self,slot:i32,ccapi:&mut CCAPI) -> Result<(), &'static str>{
        if ccapi.scopes.contains(&String::from("inventory")){
            let mut tag_build = String::new();
            for (str_key,str_value) in self.str_tags.clone(){
                tag_build = format!("{}{}",tag_build,format!("{{\"hypercube:{str_key}\":\"{str_value}\"}}")) // {"hypercube:terminal":"abc"}
            }
            let build = format!("[{{count: {}, Slot: {}b, components: {{{{\"minecraft:custom_data\": {{PublicBukkitValues: {}}}}}, id: \"{}\"}}]",self.count,slot,tag_build,self.material);
            let _ = ccapi.socket.send(Message::text(format!("setinv {}",build)));
            return Ok(());
        }else{
            return Err("Insufficient Scopes");
        }
    }
    pub fn set_material(&mut self,material:String){
        self.material = material;
    }

    pub fn set_name(&mut self,name:String){
        self.name = name;
    }
    pub fn set_str_tag(&mut self,key:String,value:String){
        self.str_tags.insert(key,value);
    }

    pub fn set_int_tag(&mut self,key:String,value:i32){
        self.int_tags.insert(key,value);
    }
    pub fn set_count(&mut self,count:i32){
        self.count = count;
    }
} 