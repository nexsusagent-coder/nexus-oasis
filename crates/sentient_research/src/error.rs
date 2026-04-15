//! ─── RESEARCH ERROR TYPES ───

use sentient_common::error::SENTIENTError;
use std::fmt;

#[derive(Debug)]
pub enum ResearchError {
    ExtractionFailed(String),
    RateLimited(String),
    NoSourcesFound,
    AnalysisFailed(String),
    ParsingFailed(String),
    NetworkError(String),
    Timeout,
    InvalidQuery(String),
    CredibilityCheckFailed(String),
    ReportGenerationFailed(String),
    Internal(String),
}

impl fmt::Display for ResearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtractionFailed(s) => write!(f, "Source extraction failed: {}", s),
            Self::RateLimited(s) => write!(f, "Rate limited by source: {}", s),
            Self::NoSourcesFound => write!(f, "No sources found for query"),
            Self::AnalysisFailed(s) => write!(f, "Analysis failed: {}", s),
            Self::ParsingFailed(s) => write!(f, "Content parsing failed: {}", s),
            Self::NetworkError(s) => write!(f, "Network error: {}", s),
            Self::Timeout => write!(f, "Timeout while researching"),
            Self::InvalidQuery(s) => write!(f, "Invalid query: {}", s),
            Self::CredibilityCheckFailed(s) => write!(f, "Credibility check failed: {}", s),
            Self::ReportGenerationFailed(s) => write!(f, "Report generation failed: {}", s),
            Self::Internal(s) => write!(f, "Internal error: {}", s),
        }
    }
}

impl std::error::Error for ResearchError {}

impl From<ResearchError> for SENTIENTError {
    fn from(err: ResearchError) -> Self {
        SENTIENTError::General(err.to_string())
    }
}

impl From<reqwest::Error> for ResearchError {
    fn from(err: reqwest::Error) -> Self {
        ResearchError::NetworkError(err.to_string())
    }
}

impl ResearchError {
    pub fn to_sentient_message(&self) -> String {
        match self {
            Self::ExtractionFailed(s) => format!("RESEARCH_MODULE: İçerik çıkarılamadı - {}", s),
            Self::RateLimited(s) => format!("RESEARCH_MODULE: Hız sınırı aşıldı - {}", s),
            Self::NoSourcesFound => "RESEARCH_MODULE: Kaynak bulunamadı. Farklı anahtar kelimeler deneyin.".to_string(),
            Self::AnalysisFailed(s) => format!("RESEARCH_MODULE: Analiz hatası - {}", s),
            Self::ParsingFailed(s) => format!("RESEARCH_MODULE: İçerik ayrıştırılamadı - {}", s),
            Self::NetworkError(s) => format!("RESEARCH_MODULE: Ağ hatası - {}", s),
            Self::Timeout => "RESEARCH_MODULE: Araştırma zaman aşımına uğradı.".to_string(),
            Self::InvalidQuery(s) => format!("RESEARCH_MODULE: Geçersiz sorgu - {}", s),
            Self::CredibilityCheckFailed(s) => format!("RESEARCH_MODULE: Güvenilirlik kontrolü başarısız - {}", s),
            Self::ReportGenerationFailed(s) => format!("RESEARCH_MODULE: Rapor oluşturulamadı - {}", s),
            Self::Internal(s) => format!("RESEARCH_MODULE: İç hata - {}", s),
        }
    }
}
