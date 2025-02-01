use graphitesparklib;
fn main(){
    let mut new = graphitesparklib::Item::new(format!("minecraft:stick"),1);
    new.set_str_tag(String::from("\\uwu"), String::from(":3"));
    new.set_str_tag(String::from("\"bar"), String::from("foo"));
    let mut api = graphitesparklib::CCAPI::connect();
    api.request_scope(format!("inventory"));
    let _ = api.set_inv(1, new);
    api.terminate();
}
eprintln