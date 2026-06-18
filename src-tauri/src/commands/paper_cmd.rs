// src-tauri/src/commands/paper_cmd.rs

use crate::db::DbPool;
use crate::domain::paper::{CreatePaperRequest, Paper, UpdatePaperRequest};
use crate::error::AppResult;
use crate::services::paper_service;
use tauri::State;

#[tauri::command]
pub async fn create_paper(
    pool: State<'_, DbPool>,
    req: CreatePaperRequest,
) -> AppResult<Paper> {
    paper_service::create_paper(&pool, req).await
}

#[tauri::command]
pub async fn list_papers(pool: State<'_, DbPool>) -> AppResult<Vec<Paper>> {
    paper_service::list_papers(&pool).await
}

#[tauri::command]
pub async fn get_paper(
    pool: State<'_, DbPool>,
    id: String,
) -> AppResult<Paper> {
    paper_service::get_paper(&pool, &id).await
}

#[tauri::command]
pub async fn update_paper(
    pool: State<'_, DbPool>,
    req: UpdatePaperRequest,
) -> AppResult<Paper> {
    paper_service::update_paper(&pool, req).await
}

#[tauri::command]
pub async fn delete_paper(
    pool: State<'_, DbPool>,
    id: String,
) -> AppResult<()> {
    paper_service::delete_paper(&pool, &id).await
}