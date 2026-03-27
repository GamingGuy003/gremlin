mod error;
mod globals;

fn main() {
    println!("{:?}", globals::get_config());
}
