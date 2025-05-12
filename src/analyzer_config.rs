#[derive(Debug, Default)]
pub struct AnalyzerConfig {
    pub db_connection_uri: String,
    pub distinct_transactions: bool,
    pub commit: bool,
}
