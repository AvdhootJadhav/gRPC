use tonic::{transport::Server, Request, Response, Status};
use txn::transaction_server::{Transaction, TransactionServer};
use txn::{TransactionRequest, TransactionResponse};

pub mod txn {
    tonic::include_proto!("transaction");
}

#[derive(Debug, Default)]
pub struct TransactionService {}

#[tonic::async_trait]
impl Transaction for TransactionService {
    async fn get_transaction(
        &self,
        request: Request<TransactionRequest>,
    ) -> Result<Response<TransactionResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = TransactionResponse { txn_id: 1 };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = TransactionService::default();

    Server::builder()
        .add_service(TransactionServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
