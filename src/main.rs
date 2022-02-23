mod server;

fn main() {

   let server_addr = "127.0.0.1:8023";

   println!("Welcome to Server-Client Hello World!");

   let mut calc_string = String::new();
   println!("What calculation should the server compute for you?");

   std::io::stdin().read_line(&mut calc_string).unwrap();

   print!("Sending request: '{}' to the server @ {}", calc_string, server_addr);

   let server = server::MyServer::new(server_addr);
   let server_thread = std::thread::spawn(move|| 
   {
      server.start();
   });

   server_thread.join();
}
