use tonic::{transport::Server, Request, Response, Status};

use admin_actions::user_manager_server::{UserManager, UserManagerServer};
use admin_actions::{CreateUserReply, CreateUserRequest};

pub mod admin_actions {
    tonic::include_proto!("admin_actions");
}

#[derive(Debug, Default)]
pub struct MyUserManager {}

#[tonic::async_trait]
impl UserManager for MyUserManager {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = admin_actions::CreateUserReply { rc: 0 };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "192.168.1.42:50051".parse()?;
    let user_manager = MyUserManager::default();

    println!("Starting on: {:?}", addr);
    Server::builder()
        .add_service(UserManagerServer::new(user_manager))
        .serve(addr)
        .await?;
    println!("Starting on: {:?}", addr);

    Ok(())
}
