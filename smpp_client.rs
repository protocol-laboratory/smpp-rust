use std::io;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::bound_atomic::BoundAtomic;
use crate::constant;
use crate::protocol::{
    BindTransmitter, BindTransmitterResp, SmppBody, SmppHeader, SmppPdu,
};

pub struct SmppClient {
    host: String,
    port: u16,
    sequence_number: BoundAtomic,
    stream: Option<TcpStream>,
}

impl SmppClient {
    pub fn new(host: &str, port: u16) -> Self {
        SmppClient {
            host: host.to_string(),
            port,
            sequence_number: BoundAtomic::new(1, 0x7FFFFFFF),
            stream: None,
        }
    }

    pub async fn connect(&mut self) -> io::Result<()> {
        let addr = format!("{}:{}", self.host, self.port);
        let stream = TcpStream::connect(addr).await?;
        self.stream = Some(stream);
        Ok(())
    }

    pub async fn bind_transmitter(
        &mut self,
        bind_transmitter: BindTransmitter,
    ) -> io::Result<BindTransmitterResp> {
        if let Some(stream) = &mut self.stream {
            let sequence_number = self.sequence_number.next_val();
            let pdu = SmppPdu {
                header: SmppHeader {
                    command_length: 0,
                    command_id: constant::BIND_TRANSMITTER_ID,
                    command_status: 0,
                    sequence_number,
                },
                body: SmppBody::BindTransmitter(bind_transmitter),
            };
            let encoded_request = pdu.encode();
            stream.write_all(&encoded_request).await?;

            let mut length_buf = [0u8; 4];
            stream.read_exact(&mut length_buf).await?;
            let msg_length = u32::from_be_bytes(length_buf) as usize - 4;

            let mut msg_buf = vec![0u8; msg_length];
            stream.read_exact(&mut msg_buf).await?;

            let response = SmppPdu::decode(&msg_buf)?;
            if response.header.command_status != 0 {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Error response: {:?}", response.header.command_status),
                ))
            } else {
                // Assuming response.body is of type BindTransmitterResp
                match response.body {
                    SmppBody::BindTransmitterResp(resp) => Ok(resp),
                    _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unexpected response body")),
                }
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected"))
        }
    }

    pub async fn close(&mut self) -> io::Result<()> {
        if let Some(mut stream) = self.stream.take() {
            stream.shutdown().await?;
        }
        Ok(())
    }
}
