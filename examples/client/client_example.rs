use smpp_rust::protocol::BindTransmitter;
use smpp_rust::smpp_client::SmppClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SmppClient::new("127.0.0.1", 2775);
    client.connect().await?;
    let bind_transmitter = BindTransmitter{
        system_id: "system_id".to_string(),
        password: "password".to_string(),
        system_type: "system_type".to_string(),
        interface_version: 0x34,
        addr_ton: 0,
        addr_npi: 0,
        address_range: "".to_string(),
    };
    client.bind_transmitter(bind_transmitter).await?;
    client.close().await?;
    Ok(())
}
