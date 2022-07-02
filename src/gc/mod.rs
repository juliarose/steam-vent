#[cfg(feature = "tf2")]
pub mod tf2;

use protobuf::Message;
use std::{fmt::Debug, io::{Cursor, Write}};
use crate::{
    net::{PROTO_MASK, NetworkError, RawNetMessage},
    message::{NetMessage, MalformedBody},
    proto::{
        enums_clientserver::EMsg,
        steammessages_clientserver_2::{
            CMsgGCClient,
        },
        steammessages_base::CMsgProtoBufHeader,
    },
};
use byteorder::{LittleEndian, ReadBytesExt};
use bytes::{Buf, BytesMut};

pub trait App {
    const APPID: u32;
}

pub trait GCMessage: Debug + Message {}

#[derive(Debug)]
pub struct ClientFromGCMessage {
    pub appid: u32,
    pub msgtype: i32,
    pub target_job_id: u64,
    pub payload: BytesMut,
}

impl ClientFromGCMessage {
    const KIND: EMsg = EMsg::k_EMsgClientFromGC;
    
    pub fn from_message(msg: RawNetMessage) -> Result<Self, NetworkError> {
        let msg = into_message(msg)?;
        let appid = msg.get_appid();
        let msgtype = msg.get_msgtype() as i32 & (!PROTO_MASK) as i32;
        let payload = msg.get_payload();
        let is_proto = (msg.get_msgtype() as i32 & PROTO_MASK as i32) != 0;
        let mut buff = BytesMut::from(payload);
        
        let (target_job_id, payload) = if is_proto {
            let proto_bytes = {
                // take first 8 bytes
                let header = buff.split_to(8);
                let mut reader = Cursor::new(&header);
                // skip the first 4 bytes
                let _ = reader.read_i32::<LittleEndian>()?;
                let header_length = reader.read_i32::<LittleEndian>()?;
                let proto_bytes = buff.split_to(header_length as usize);
                
                proto_bytes
            };
            let header = CMsgProtoBufHeader::parse_from_reader(&mut proto_bytes.reader())
                .map_err(|e| MalformedBody(Self::KIND, e.into()))?;
            let target_job_id = header.get_jobid_target();
            let payload = BytesMut::from(buff);
            
            (target_job_id, payload)
        } else {
            let header = buff.split_to(18);
            let mut reader = Cursor::new(header);
            let _ = reader.read_u16::<LittleEndian>()?;
            let target_job_id = reader.read_u64::<LittleEndian>()?;
            let payload = BytesMut::from(buff);
            
            (target_job_id, payload)
        };
        
        Ok(Self {
            appid,
            msgtype,
            target_job_id,
            payload,
        })
    }
    
    pub fn payload_into_message<Request: Message>(
        self,
    ) -> Result<Request, NetworkError> {
        Ok(
            Request::parse_from_reader(&mut self.payload.reader())
                .map_err(|e| MalformedBody(Self::KIND, e.into()))?,
        )
    }
}

fn into_message(msg: RawNetMessage) -> Result<CMsgGCClient, NetworkError> {
    if msg.kind == EMsg::k_EMsgClientFromGC {
        Ok(
            CMsgGCClient::parse_from_reader(&mut msg.data.reader())
                .map_err(|e| MalformedBody(EMsg::k_EMsgClientFromGC, e.into()))?
        )
    } else {
        Err(NetworkError::DifferentMessage(msg.kind, EMsg::k_EMsgClientFromGC))
    }
}

#[derive(Debug, Clone)]
pub struct ClientToGCMessage(pub CMsgGCClient);

impl ClientToGCMessage {
    /// Sets the payload.
    pub fn set_payload(
        &mut self,
        payload: Vec<u8>,
    ) {
        self.0.set_payload(payload);
    }
    
    pub fn new(
        appid: u32,
        msgtype: i32,
        is_proto: bool,
    ) -> Self {
        let mut body = CMsgGCClient::new();
        
        body.set_appid(appid);
        
        if is_proto {
            body.set_msgtype(msgtype as u32 | PROTO_MASK);
        } else {
            body.set_msgtype(msgtype as u32);
        }
        
        Self(body)
    }
}

impl NetMessage for ClientToGCMessage {
    const KIND: EMsg = EMsg::k_EMsgClientToGC;
    const IS_PROTOBUF: bool = true;

    // fn read_body(_data: BytesMut, _header: &NetMessageHeader) -> Result<Self, MalformedBody> {
    //     panic!("Reading not implemented for {}", type_name::<Self>())
    // }

    fn write_body<W: Write>(&self, mut writer: W) -> Result<(), std::io::Error> {
        self.0.write_to_writer(&mut writer)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
    }

    fn encode_size(&self) -> usize {
        self.0.compute_size() as usize
    }

    // fn process_header(&self, _header: &mut NetMessageHeader) {}
}
