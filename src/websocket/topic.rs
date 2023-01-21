use crate::BitFlyerError;
use fehler::throw;
use serde::de::{Error, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Topic {
    BoardSnapshot(Option<String>),
    Board(Option<String>),
    Ticker(Option<String>),

    // requires auth
    Order,
    Margin,
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Topic::*;
        let repr = match self {
            BoardSnapshot(None) => "lightning_board_snapshot_BTC_JPY".to_string(),
            BoardSnapshot(Some(filter)) => format!("lightning_board_snapshot_:{}", filter),
            Board(None) => "lightning_board_BTC_JPY".to_string(),
            Board(Some(filter)) => format!("lightning_board_:{}", filter),
            Ticker(None) => "lightning_ticker_BTC_JPY".to_string(),
            Ticker(Some(filter)) => format!("lightning_ticker_:{}", filter),
            
            // requires auth
            Order => "child_order_events".to_string(),
            Margin => "parent_order_events".to_string(),
        };

        write!(f, "{}", repr)
    }
}

impl std::str::FromStr for Topic {
    type Err = BitFlyerError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::Topic::*;
        let reprs: Vec<_> = s.split(':').collect();

        let topic = match reprs.as_slice() {
            ["lightning_board_snapshot_", filter] => BoardSnapshot(Some((*filter).to_string())),
            ["lightning_board_", filter] => Board(Some((*filter).to_string())),
            ["lightning_ticker_", filter] => Ticker(Some((*filter).to_string())),

            // requires auth
            ["order"] => Order,
            ["margin"] => Margin,
            _ => throw!(BitFlyerError::ParseTopicError(s.into())),
        };

        Ok(topic)
    }
}
impl Serialize for Topic {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let repr = String::deserialize(deserializer)?;
        let topic = repr
            .parse()
            .map_err(|_| D::Error::invalid_value(Unexpected::Str(&repr), &"A valid topic"))?;
        Ok(topic)
    }
}
