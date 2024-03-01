#![allow(dead_code)]

use std::io;
use crate::constant;

pub struct SmppPdu {
    pub header: SmppHeader,
    pub body: SmppBody,
}

impl SmppPdu {
    pub fn encode(&self) -> Vec<u8> {
        let mut body_buf = match &self.body {
            SmppBody::BindTransmitter(bind_transmitter) => bind_transmitter.encode(),
            _ => unimplemented!(),
        };

        let command_length = (body_buf.len() + 16) as i32;
        let header = SmppHeader {
            command_length,
            command_id: self.header.command_id,
            command_status: self.header.command_status,
            sequence_number: self.header.sequence_number,
        };

        let mut buf = header.encode();
        buf.append(&mut body_buf);
        buf
    }

    pub fn decode(buf: &[u8]) -> io::Result<Self> {
        let header = SmppHeader::decode(&buf[0..16])?;
        let body = match header.command_id {
            constant::BIND_TRANSMITTER_RESP_ID => SmppBody::BindTransmitterResp(BindTransmitterResp::decode(&buf[16..])?),
            _ => unimplemented!(),
        };
        Ok(SmppPdu { header, body })
    }
}

pub struct SmppHeader {
    pub command_length: i32,
    pub command_id: u32,
    pub command_status: i32,
    pub sequence_number: i32,
}

impl SmppHeader {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![];
        buf.extend_from_slice(&self.command_length.to_be_bytes());
        buf.extend_from_slice(&self.command_id.to_be_bytes());
        buf.extend_from_slice(&self.command_status.to_be_bytes());
        buf.extend_from_slice(&self.sequence_number.to_be_bytes());
        buf
    }

    pub(crate) fn decode(buf: &[u8]) -> io::Result<Self> {
        if buf.len() < 16 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Buffer too short for SmppHeader"));
        }
        let command_id = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let command_status = i32::from_be_bytes(buf[4..8].try_into().unwrap());
        let sequence_number = i32::from_be_bytes(buf[8..12].try_into().unwrap());
        Ok(SmppHeader {
            command_length: 0,
            command_id,
            command_status,
            sequence_number,
        })
    }
}

pub enum SmppBody {
    BindReceiver(BindReceiver),
    BindReceiverResp(BindReceiverResp),
    BindTransmitter(BindTransmitter),
    BindTransmitterResp(BindTransmitterResp),
    QuerySm(QuerySm),
    QuerySmResp(QuerySmResp),
    SubmitSm(SubmitSm),
    SubmitSmResp(SubmitSmResp),
    DeliverSm(DeliverSm),
    DeliverSmResp(DeliverSmResp),
    Unbind(Unbind),
    UnbindResp(UnbindResp),
    ReplaceSm(ReplaceSm),
    ReplaceSmResp(ReplaceSmResp),
    CancelSm(CancelSm),
    CancelSmResp(CancelSmResp),
    BindTransceiver(BindTransceiver),
    BindTransceiverResp(BindTransceiverResp),
    Outbind(Outbind),
    EnquireLink(EnquireLink),
    EnquireLinkResp(EnquireLinkResp),
    SubmitMulti(SubmitMulti),
    SubmitMultiResp(SubmitMultiResp),
}

pub struct BindReceiver {
    pub system_id: String,
    pub password: String,
    pub system_type: String,
    pub interface_version: u8,
    pub addr_ton: u8,
    pub addr_npi: u8,
    pub address_range: String,
}

impl BindReceiver {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![];
        write_cstring(&mut buf, &self.system_id);
        write_cstring(&mut buf, &self.password);
        write_cstring(&mut buf, &self.system_type);
        buf.push(self.interface_version);
        buf.push(self.addr_ton);
        buf.push(self.addr_npi);
        write_cstring(&mut buf, &self.address_range);
        buf
    }

    pub(crate) fn decode(buf: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let system_id = read_cstring(buf, &mut offset)?;
        let password = read_cstring(buf, &mut offset)?;
        let system_type = read_cstring(buf, &mut offset)?;
        let interface_version = buf[offset];
        offset += 1;
        let addr_ton = buf[offset];
        offset += 1;
        let addr_npi = buf[offset];
        offset += 1;
        let address_range = read_cstring(buf, &mut offset)?;

        Ok(BindReceiver {
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        })
    }
}

pub struct BindReceiverResp {
    pub system_id: String,
}

impl BindReceiverResp {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![];
        write_cstring(&mut buf, &self.system_id);
        buf
    }

    pub(crate) fn decode(buf: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let system_id = read_cstring(buf, &mut offset)?;
        Ok(BindReceiverResp { system_id })
    }
}

pub struct BindTransmitter {
    pub system_id: String,
    pub password: String,
    pub system_type: String,
    pub interface_version: u8,
    pub addr_ton: u8,
    pub addr_npi: u8,
    pub address_range: String,
}

impl BindTransmitter {
    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![];
        write_cstring(&mut buf, &self.system_id);
        write_cstring(&mut buf, &self.password);
        write_cstring(&mut buf, &self.system_type);
        buf.push(self.interface_version);
        buf.push(self.addr_ton);
        buf.push(self.addr_npi);
        write_cstring(&mut buf, &self.address_range);
        buf
    }

    pub(crate) fn decode(buf: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let system_id = read_cstring(buf, &mut offset)?;
        let password = read_cstring(buf, &mut offset)?;
        let system_type = read_cstring(buf, &mut offset)?;
        let interface_version = buf[offset];
        offset += 1;
        let addr_ton = buf[offset];
        offset += 1;
        let addr_npi = buf[offset];
        offset += 1;
        let address_range = read_cstring(buf, &mut offset)?;

        Ok(BindTransmitter {
            system_id,
            password,
            system_type,
            interface_version,
            addr_ton,
            addr_npi,
            address_range,
        })
    }
}

pub struct BindTransmitterResp {
    pub system_id: String,
}

impl BindTransmitterResp {
    pub fn new(system_id: String) -> BindTransmitterResp {
        BindTransmitterResp { system_id }
    }

    pub(crate) fn encode(&self) -> Vec<u8> {
        let mut buf = vec![];
        write_cstring(&mut buf, &self.system_id);
        buf
    }

    pub(crate) fn decode(buf: &[u8]) -> io::Result<Self> {
        let mut offset = 0;
        let system_id = read_cstring(buf, &mut offset)?;
        Ok(BindTransmitterResp { system_id })
    }
}

pub struct QuerySm {
    pub message_id: String,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: String,
}

pub struct QuerySmResp {
    pub message_id: String,
    pub final_date: String,
    pub message_state: u8,
    pub error_code: u8,
}

pub struct SubmitSm {
    pub service_type: String,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: String,
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: String,
    pub esm_class: u8,
    pub protocol_id: u8,
    pub priority_flag: u8,
    pub schedule_delivery_time: String,
    pub validity_period: String,
    pub registered_delivery: u8,
    pub replace_if_present_flag: u8,
    pub data_coding: u8,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub message_payload: TagLengthValue,
}

pub struct SubmitSmResp {
    pub message_id: String,
}

pub struct DeliverSm {
    pub service_type: String,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: String,
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: String,
    pub esm_class: u8,
    pub protocol_id: u8,
    pub priority_flag: u8,
    pub schedule_delivery_time: String,
    pub validity_period: String,
    pub registered_delivery: u8,
    pub replace_if_present_flag: u8,
    pub data_coding: u8,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
}

pub struct DeliverSmResp {
    pub message_id: String,
}

pub struct Unbind {}

pub struct UnbindResp {}

pub struct ReplaceSm {
    pub message_id: String,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: String,
    pub schedule_delivery_time: String,
    pub validity_period: String,
    pub registered_delivery: u8,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
}

pub struct ReplaceSmResp {
    pub message_id: String,
}

pub struct CancelSm {
    pub service_type: String,
    pub message_id: String,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: String,
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: String,
}

pub struct CancelSmResp {}

pub struct BindTransceiver {
    pub system_id: String,
    pub password: String,
    pub system_type: String,
    pub interface_version: u8,
    pub addr_ton: u8,
    pub addr_npi: u8,
    pub address_range: String,
}

pub struct BindTransceiverResp {
    pub system_id: String,
}

pub struct Outbind {
    pub system_id: String,
    pub password: String,
}

pub struct EnquireLink {}

pub struct EnquireLinkResp {}

pub struct SubmitMulti {
    pub service_type: String,
    pub source_addr_ton: u8,
    pub source_addr_npi: u8,
    pub source_addr: String,
    pub number_of_dests: u8,
    pub dest_address: Vec<DestinationAddress>,
    pub esm_class: u8,
    pub protocol_id: u8,
    pub priority_flag: u8,
    pub schedule_delivery_time: String,
    pub validity_period: String,
    pub registered_delivery: u8,
    pub replace_if_present_flag: u8,
    pub data_coding: u8,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
}

pub struct SubmitMultiResp {
    pub message_id: String,
    pub no_unsuccess: u8,
    pub unsuccess_smes: Vec<UnsuccessfulDelivery>,
}

pub struct DestinationAddress {
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: String,
}

pub struct TagLengthValue {
    pub tag: u16,
    pub length: u16,
    pub value: Vec<u8>,
}

pub struct UnsuccessfulDelivery {
    pub dest_addr_ton: u8,
    pub dest_addr_npi: u8,
    pub destination_addr: String,
    pub error_status_code: i32,
}

fn write_cstring(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(s.as_bytes());
    buf.push(0);
}

fn read_cstring(buf: &[u8], offset: &mut usize) -> io::Result<String> {
    let start = *offset;
    while *offset < buf.len() && buf[*offset] != 0 {
        *offset += 1;
    }
    if *offset >= buf.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid C string: null terminator not found"));
    }
    let s = std::str::from_utf8(&buf[start..*offset])
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?
        .to_string();
    *offset += 1;
    Ok(s)
}
