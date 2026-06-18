import { invoke } from "@tauri-apps/api/core";

export interface Paper {
  id: string;
  title: string;
  subtitle?: string | null;
  abstract_text?: string | null;
  publication_year?: number | null;
  publication_date?: string | null;
  doi?: string | null;
  arxiv_id?: string | null;
  publication_type: string;
  venue_name?: string | null;
  publisher?: string | null;
  url?: string | null;
  pdf_url?: string | null;
  citation_key?: string | null;
  language?: string | null;
  reading_status: string;
  rating: number;
  favorite: number;
  source?: string | null;
  imported_at: string;
  created_at: string;
  updated_at: string;
  deleted_at?: string | null;
}

export interface CreatePaperRequest {
  title: string;
  subtitle?: string | null;
  abstract_text?: string | null;
  publication_year?: number | null;
  doi?: string | null;
  arxiv_id?: string | null;
  publication_type?: string | null;
  venue_name?: string | null;
  url?: string | null;
}

export async function createPaper(req: CreatePaperRequest): Promise<Paper> {
  return await invoke<Paper>("create_paper", { req });
}

export async function listPapers(): Promise<Paper[]> {
  return await invoke<Paper[]>("list_papers");
}

export async function deletePaper(id: string): Promise<void> {
  await invoke<void>("delete_paper", { id });
}