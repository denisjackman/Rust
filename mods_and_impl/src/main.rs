// this imports the module  Rustc will say hello I see a module named sample_module does the directory
// exist and if so I will import
mod sample_module;
use sample_module::SampleImpl;

fn main(){
    let my_object = SampleImpl::new("Matt".to_string());
    my_object.hello_world();
}