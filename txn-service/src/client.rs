use txn::{transaction_client::TransactionClient, TransactionRequest};

pub mod txn {
    tonic::include_proto!("transaction");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TransactionClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(TransactionRequest { order_id: 10 });

    let response = client.get_transaction(request).await?;

    println!("RESPONSE = {:?}", response.into_inner());

    Ok(())
}
