use std::borrow::Cow;

use anyhow::bail;
use serde::{Deserialize, Serialize};

///
/// Format for incoming/outgoing messages from the node
///
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Message<'a> {
    pub src: Cow<'a, str>,
    #[serde(rename = "dest")]
    pub dst: Cow<'a, str>,
    pub body: Body<'a>,
}

impl<'a> Message<'a> {
    pub fn respond(
        self,
        callback: impl FnOnce(usize, RequestBody<'a>) -> anyhow::Result<ResponseBody<'a>>,
    ) -> anyhow::Result<Message<'a>> {
        Ok(Message {
            src: self.dst,
            dst: self.src,
            body: match self.body {
                Body::Request { msg_id, body } => Body::Response {
                    in_reply_to: msg_id,
                    body: callback(msg_id, body)?,
                },
                msg => bail!("Invalid request received: {:?}", msg),
            },
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Body<'a> {
    Request {
        msg_id: usize,
        #[serde(flatten)]
        body: RequestBody<'a>,
    },
    Response {
        in_reply_to: usize,
        #[serde(flatten)]
        body: ResponseBody<'a>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum RequestBody<'a> {
    Init {
        node_id: Cow<'a, str>,
        node_ids: Vec<Cow<'a, str>>,
    },
    // Messages For Echo Challenge
    Echo {
        echo: Cow<'a, str>,
    },
    // Messages For Generate Challenge
    Generate {},
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ResponseBody<'a> {
    InitOk {},
    Error { code: ErrorCode, text: Cow<'a, str> },
    EchoOk { echo: Cow<'a, str> },
    GenerateOk { id: Cow<'a, str> },
}

///
/// Allows error codes to be indicated as enum variants
///
#[derive(Clone, Debug, Copy)]
#[repr(u8)]
pub enum ErrorCode {
    Timeout = 0,
    NodeNotFound = 1,
    NotSupported = 10,
    TemporarilyUnavailable = 11,
    MalformedRequest = 12,
    Crash = 13,
    Abort = 14,
    KeyDoesNotExist = 20,
    KeyAlreadyExist = 21,
    PreconditionFailed = 22,
    TxnConflict = 30,
}

impl ErrorCode {
    fn into(self) -> u8 {
        self as u8
    }
    fn from(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Timeout),
            1 => Some(Self::NodeNotFound),
            10 => Some(Self::NotSupported),
            11 => Some(Self::TemporarilyUnavailable),
            12 => Some(Self::MalformedRequest),
            13 => Some(Self::Crash),
            14 => Some(Self::Abort),
            20 => Some(Self::KeyDoesNotExist),
            21 => Some(Self::KeyAlreadyExist),
            22 => Some(Self::PreconditionFailed),
            30 => Some(Self::TxnConflict),
            _ => None,
        }
    }
}

impl Serialize for ErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8((*self).into())
    }
}

impl<'de> Deserialize<'de> for ErrorCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match ErrorCode::from(u8::deserialize(deserializer)?) {
            Some(value) => Ok(value),
            None => Err(serde::de::Error::custom("Failed to get the variant")),
        }
    }
}

#[cfg(test)]
mod tests;
