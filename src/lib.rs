use std::collections::HashMap;
use tungstenite::{stream::MaybeTlsStream, Message, WebSocket};
use std::net::TcpStream;
fn safeify(mut text:String) -> String{
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
    pub fn set_inv(&mut self,slot:i32,item:Item) -> Result<(), &'static str>{
        if self.scopes.contains(&String::from("inventory")){
            let mut tag_build = String::new();
            for (mut str_key,mut str_value) in item.str_tags.clone(){
                str_key = safeify(str_key);
                str_value = safeify(str_value);
                tag_build = format!("{}{}",tag_build,format!("{{\"hypercube:{str_key}\":\"{str_value}\"}}")) // {"hypercube:terminal":"abc"}
            }
            for (mut int_key,int_value) in item.int_tags.clone(){
                int_key = safeify(int_key);
                tag_build = format!("{}{}",tag_build,format!("{{\"hypercube:{int_key}\":{int_value}}}")) // {"hypercube:terminal":"abc"}
            }
            let build = format!("[{{count: {}, Slot: {}b, components: {{{{\"minecraft:custom_data\": {{PublicBukkitValues: {}}}}}, id: \"{}\"}}]",item.count,slot,tag_build,item.material);
            let _ = self.socket.send(Message::text(format!("setinv {}",build)));
            return Ok(());
        }else{
            return Err("Insufficient Scopes");
        }
    }
    pub fn connect() -> CCAPI{
        let (socket,_) = tungstenite::connect("ws://localhost:31375").expect("Error Connecting to CCAPI");
        let scopes: Vec<String> = Vec::new();
        CCAPI {socket,scopes}
    }
    pub fn terminate(&mut self){
        let _ = &self.socket.close(None);
    }
}
impl Item{
    pub fn new(name:String,material:String,count:i32) -> Item{
        let str_tags: HashMap<String, String> = HashMap::new();
        let int_tags: HashMap<String, i32> = HashMap::new();
        Item {name,material,count,int_tags,str_tags}
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