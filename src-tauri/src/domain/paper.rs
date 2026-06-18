// src-tauri/src/domain/paper.rs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Paper {
    pub id: String,

    pub title: String,
    pub subtitle: Option<String>,
    pub abstract_text: Option<String>,

    pub publication_year: Option<i64>,
    pub publication_date: Option<String>,

    pub doi: Option<String>,
    pub arxiv_id: Option<String>,

    pub publication_type: String,
    pub venue_name: Option<String>,
    pub publisher: Option<String>,

    pub url: Option<String>,
    pub pdf_url: Option<String>,

    pub citation_key: Option<String>,
    pub language: Option<String>,

    pub reading_status: String,
    pub rating: i64,
    pub favorite: i64,

    pub source: Option<String>,

    pub imported_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePaperRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub abstract_text: Option<String>,
    pub publication_year: Option<i64>,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub publication_type: Option<String>,
    pub venue_name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaperRequest {
    pub id: String,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub abstract_text: Option<String>,
    pub publication_year: Option<i64>,
    pub doi: Option<String>,
    pub arxiv_id: Option<String>,
    pub publication_type: Option<String>,
    pub venue_name: Option<String>,
    pub url: Option<String>,
    pub reading_status: Option<String>,
    pub rating: Option<i64>,
    pub favorite: Option<i64>,
}