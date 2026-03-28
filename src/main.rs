mod error;
#[allow(unused)]
mod globals;

fn main() {
    println!("{:?}", globals::get_config());
}
