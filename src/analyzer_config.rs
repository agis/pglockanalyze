#[derive(Debug, Default)]
pub struct AnalyzerConfig {
    pub db_connection_uri: String,

    /// If true, each statement will be executed in its own transaction.
    /// Otherwise, all statements will be executed in the same transaction.
    pub distinct_transactions: bool,

    /// If true, the transaction(s) will be committed. Otherwise they will be
    /// rolled back.
    pub commit: bool,
}
