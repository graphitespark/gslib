use graphitesparklib;

fn main(){
  let mut link = graphitesparklib::CCAPI::connect();  
  link.request_scope(vec!("clear_plot".to_string()));
}
