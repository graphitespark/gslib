use graphitesparklib;
fn main(){
    let new = graphitesparklib::Item::new(format!("uwu"),format!("minecraft:stick"),1);
    let mut api = graphitesparklib::CCAPI::connect();
    let _ = new.set_inv(1, &mut api);
}