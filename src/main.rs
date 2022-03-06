mod server;

use std::io::prelude::*;

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

   loop // Loops until a successful connection occurs
   {
      if let Ok(mut client) = std::net::TcpStream::connect(server_addr)
      {
         println!("Client connected to server!");
         let mut read_buffer : [u8; 128] = [0; 128];
         match client.read(&mut read_buffer)
         {
            Ok(bytes_read) =>
            {
               println!("Read {} bytes from server...", bytes_read);

               let mut first_null = 0;
               for x in 0..read_buffer.len() 
               {
                  if read_buffer[x] == "\0".as_bytes()[0]
                  {
                     first_null = x;
                     break;
                  }
                  
               }

               if let Ok(read_msg) = std::str::from_utf8(&read_buffer[0..first_null])
               {
                  println!("Server message: {:?}", read_msg);
               }

            }
            Err(e) =>
            {
               println!("Failed to read bytes from server with error: {}", e);
            }

         }

         break;
      }

   }

   match server_thread.join()
   {
      Ok(_) =>
      {
         // do nothing
      }
      Err(_) =>
      {
         println!("Failed to join server thread!");
      }
   }

   println!("PROGRAM COMPLETE!");
}
