use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Message<'a> {
    pub src: Cow<'a, str>,
    #[serde(rename = "dest")]
    pub dst: Cow<'a, str>,
    pub body: Body<'a>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Body<'a> {
    Init {
        msg_id: usize,
        node_id: Cow<'a, str>,
        node_ids: Vec<Cow<'a, str>>,
    },
    InitOk {
        in_reply_to: usize,
    },
    Echo {
        msg_id: usize,
        echo: Cow<'a, str>,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: Cow<'a, str>,
    },
    Error {
        in_reply_to: usize,
        code: ErrorCode,
        text: Cow<'a, str>,
    },
}

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

impl<'a> Body<'a> {
    #[allow(dead_code)]
    fn get_msg_id(&self) -> Option<usize> {
        match self {
            Body::Init { msg_id, .. } | Body::Echo { msg_id, .. } | Body::EchoOk { msg_id, .. } => {
                Some(*msg_id)
            }
            Body::InitOk { .. } | Body::Error { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests;
