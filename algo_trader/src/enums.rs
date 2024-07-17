#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub enum BidOrAsk {
    #[default]
    BID,
    ASK,
}
