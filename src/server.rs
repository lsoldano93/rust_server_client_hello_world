use std::io::prelude::*;

pub struct MyServer
{
   tcp_listener : std::net::TcpListener
}

impl MyServer
{
   pub fn new(addr: &str) -> MyServer
   {
      MyServer
      { 
         tcp_listener : std::net::TcpListener::bind(addr).unwrap()
      }
   }

   pub fn start(&self)
   {
      match self.tcp_listener.accept() 
      {
         Ok((mut socket, _addr)) =>
         {
            match socket.write("Hello, I'll be your server tonight, how can I help you?".as_bytes())
            {
               Ok(_) =>
               {
                  match socket.flush()
                  {
                     Ok(_) =>
                     {
                        println!("blah");
                     }
                     Err(e) =>
                     {
                        println!("MyServer::start() failed to flush stream with error {}", e);
                     }

                  }

               }
               Err(e) =>
               {
                  println!("MyServer::start() failed to write with error {}", e);
               }
            }
            
         }
         Err(e) =>
         {
            println!("MyServer::start() failed to connect with error {}", e);
         }

      }

      return;
   }

}