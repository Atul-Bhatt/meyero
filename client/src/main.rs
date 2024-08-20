mod node;
mod seed;

fn main() {
   let first_seed_server = seed::get_first_seed_server();
   println!("{}", first_seed_server);
}
