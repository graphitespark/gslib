use graphitesparklib;
fn main(){
    let new = graphitesparklib::Item::new(format!("uwu"),format!("minecraft:stick"),1);
    let mut api = graphitesparklib::CCAPI::connect();
    let _ = api.set_inv(1, new);
}