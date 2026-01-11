use tonic::{transport::Server, Request, Response, Status};

// TODO: Implémenter l'inclusion des protos générés
// pub mod modules_grpc {
//     tonic::include_proto!("aether.modules");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50054".parse()?;
    println!("Module Chrome-Ghost à l'écoute sur {}", addr);

    // Server::builder()
    //     .add_service(SecurityModuleServer::new(MyModule::default()))
    //     .serve(addr)
    //     .await?;

    Ok(())
}
