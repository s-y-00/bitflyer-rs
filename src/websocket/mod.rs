mod command;
mod message;
mod topic;

pub use self::command::Command;
pub use self::message::Message as BitFlyerWsMessage;
pub use self::message::{
    Action, CancelAllAfterMessage, ErrorMessage, InfoMessage, Limit, SuccessMessage, TableFilter,
    TableMessage,
};
pub use self::topic::Topic;
use crate::consts::WS_URL;
use crate::credential::Credential;
use crate::error::BitFlyerError;
use failure::Fallible;
use fehler::{throw, throws};
use futures::sink::Sink;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use http::Method;
use log::trace;
pub use serde_json::Value;
use serde_json::{from_str, json, to_string};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message as WSMessage;
use url::Url;

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BitFlyerWebsocket {
    credential: Option<Credential>,
    inner: WSStream,
}

impl BitFlyerWebsocket {
    #[throws(failure::Error)]
    pub async fn new() -> Self {
        let (stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;
        Self {
            credential: None,
            inner: stream,
        }
    }

    #[throws(failure::Error)]
    pub async fn with_credential(api_key: &str, api_secret: &str) -> Self {
        let mut c = Self::new().await?;
        c.credential = Some(Credential(api_key.into(), api_secret.into()));
        c
    }

    #[throws(failure::Error)]
    fn get_credential(&self) -> &Credential {
        match self.credential.as_ref() {
            None => throw!(BitFlyerError::NoApiKeySet),
            Some(c) => c,
        }
    }
}

impl Sink<Command> for BitFlyerWebsocket {
    type Error = failure::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: Command) -> Result<(), Self::Error> {
        let command = match &item {
            &Command::Ping => "ping".to_string(),
            &Command::Authenticate(_, expires, _) => {
                let cred = self.get_credential()?;
                let (key, sig) = cred.signature(Method::GET, expires, &Url::parse(&WS_URL)?, "")?;
                to_string(&json!({"op": "authKeyExpires", "args": [key, expires, sig]}))?
            }
            command => to_string(command)?,
        };
        trace!("Sending '{}' through websocket", command);
        let inner = Pin::new(&mut self.inner);
        Ok(inner.start_send(WSMessage::Text(command))?)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_close(cx).map_err(|e| e.into())
    }
}

impl Stream for BitFlyerWebsocket {
    type Item = Fallible<BitFlyerWsMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let inner = Pin::new(&mut self.inner);
        let poll = inner.poll_next(cx);
        match poll {
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(Some(Ok(m))) => match parse_message(m) {
                Ok(m) => Poll::Ready(Some(Ok(m))),
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[throws(failure::Error)]
fn parse_message(msg: WSMessage) -> BitFlyerWsMessage {
    match msg {
        WSMessage::Text(message) => match message.as_str() {
            "pong" => BitFlyerWsMessage::Pong,
            others => match from_str(others) {
                Ok(r) => r,
                Err(_) => unreachable!("Cannot deserialize message from BitFlyer: '{}'", others),
            },
        },
        WSMessage::Close(_) => throw!(BitFlyerError::WebsocketClosed),
        WSMessage::Binary(c) => throw!(BitFlyerError::UnexpectedWebsocketBinaryContent(c)),
        WSMessage::Ping(_) => BitFlyerWsMessage::Ping,
        WSMessage::Pong(_) => BitFlyerWsMessage::Pong,
    }
}
