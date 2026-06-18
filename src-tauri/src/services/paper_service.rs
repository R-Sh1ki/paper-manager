// src-tauri/src/services/paper_service.rs

use crate::db::DbPool;
use crate::domain::paper::{CreatePaperRequest, Paper, UpdatePaperRequest};
use crate::error::{AppError, AppResult};
use chrono::Utc;
use uuid::Uuid;

pub async fn create_paper(pool: &DbPool, req: CreatePaperRequest) -> AppResult<Paper> {
    let title = req.title.trim();

    if title.is_empty() {
        return Err(AppError::InvalidInput("title cannot be empty".to_string()));
    }

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let publication_type = req
        .publication_type
        .unwrap_or_else(|| "unknown".to_string());

    sqlx::query(
        r#"
        INSERT INTO papers (
            id,
            title,
            subtitle,
            abstract,
            publication_year,
            doi,
            arxiv_id,
            publication_type,
            venue_name,
            url,
            reading_status,
            rating,
            favorite,
            imported_at,
            created_at,
            updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'unread', 0, 0, ?, ?, ?)
        "#,
    )
    .bind(&id)
    .bind(title)
    .bind(&req.subtitle)
    .bind(&req.abstract_text)
    .bind(req.publication_year)
    .bind(&req.doi)
    .bind(&req.arxiv_id)
    .bind(&publication_type)
    .bind(&req.venue_name)
    .bind(&req.url)
    .bind(&now)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    get_paper(pool, &id).await
}

pub async fn list_papers(pool: &DbPool) -> AppResult<Vec<Paper>> {
    let papers = sqlx::query_as::<_, Paper>(
        r#"
        SELECT
            id,
            title,
            subtitle,
            abstract AS abstract_text,
            publication_year,
            publication_date,
            doi,
            arxiv_id,
            publication_type,
            venue_name,
            publisher,
            url,
            pdf_url,
            citation_key,
            language,
            reading_status,
            rating,
            favorite,
            source,
            imported_at,
            created_at,
            updated_at,
            deleted_at
        FROM papers
        WHERE deleted_at IS NULL
        ORDER BY updated_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(papers)
}

pub async fn get_paper(pool: &DbPool, id: &str) -> AppResult<Paper> {
    let paper = sqlx::query_as::<_, Paper>(
        r#"
        SELECT
            id,
            title,
            subtitle,
            abstract AS abstract_text,
            publication_year,
            publication_date,
            doi,
            arxiv_id,
            publication_type,
            venue_name,
            publisher,
            url,
            pdf_url,
            citation_key,
            language,
            reading_status,
            rating,
            favorite,
            source,
            imported_at,
            created_at,
            updated_at,
            deleted_at
        FROM papers
        WHERE id = ? AND deleted_at IS NULL
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    paper.ok_or_else(|| AppError::NotFound(format!("paper not found: {}", id)))
}

pub async fn update_paper(pool: &DbPool, req: UpdatePaperRequest) -> AppResult<Paper> {
    let existing = get_paper(pool, &req.id).await?;

    let now = Utc::now().to_rfc3339();

    let title = req.title.unwrap_or(existing.title);
    let subtitle = req.subtitle.or(existing.subtitle);
    let abstract_text = req.abstract_text.or(existing.abstract_text);
    let publication_year = req.publication_year.or(existing.publication_year);
    let doi = req.doi.or(existing.doi);
    let arxiv_id = req.arxiv_id.or(existing.arxiv_id);
    let publication_type = req.publication_type.unwrap_or(existing.publication_type);
    let venue_name = req.venue_name.or(existing.venue_name);
    let url = req.url.or(existing.url);
    let reading_status = req.reading_status.unwrap_or(existing.reading_status);
    let rating = req.rating.unwrap_or(existing.rating);
    let favorite = req.favorite.unwrap_or(existing.favorite);

    if title.trim().is_empty() {
        return Err(AppError::InvalidInput("title cannot be empty".to_string()));
    }

    sqlx::query(
        r#"
        UPDATE papers
        SET
            title = ?,
            subtitle = ?,
            abstract = ?,
            publication_year = ?,
            doi = ?,
            arxiv_id = ?,
            publication_type = ?,
            venue_name = ?,
            url = ?,
            reading_status = ?,
            rating = ?,
            favorite = ?,
            updated_at = ?
        WHERE id = ?
        "#,
    )
    .bind(title.trim())
    .bind(subtitle)
    .bind(abstract_text)
    .bind(publication_year)
    .bind(doi)
    .bind(arxiv_id)
    .bind(publication_type)
    .bind(venue_name)
    .bind(url)
    .bind(reading_status)
    .bind(rating)
    .bind(favorite)
    .bind(now)
    .bind(&req.id)
    .execute(pool)
    .await?;

    get_paper(pool, &req.id).await
}

pub async fn delete_paper(pool: &DbPool, id: &str) -> AppResult<()> {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        r#"
        UPDATE papers
        SET deleted_at = ?, updated_at = ?
        WHERE id = ? AND deleted_at IS NULL
        "#,
    )
    .bind(&now)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("paper not found: {}", id)));
    }

    Ok(())
}