PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS papers (
    id TEXT PRIMARY KEY,

    title TEXT NOT NULL,
    subtitle TEXT,
    abstract TEXT,

    publication_year INTEGER,
    publication_date TEXT,

    doi TEXT UNIQUE,
    arxiv_id TEXT UNIQUE,

    publication_type TEXT NOT NULL DEFAULT 'unknown',
    venue_name TEXT,
    publisher TEXT,

    url TEXT,
    pdf_url TEXT,

    citation_key TEXT UNIQUE,
    language TEXT,

    reading_status TEXT NOT NULL DEFAULT 'unread',
    rating INTEGER NOT NULL DEFAULT 0,
    favorite INTEGER NOT NULL DEFAULT 0,

    source TEXT,

    imported_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_papers_title ON papers(title);
CREATE INDEX IF NOT EXISTS idx_papers_year ON papers(publication_year);
CREATE INDEX IF NOT EXISTS idx_papers_doi ON papers(doi);
CREATE INDEX IF NOT EXISTS idx_papers_arxiv_id ON papers(arxiv_id);
CREATE INDEX IF NOT EXISTS idx_papers_reading_status ON papers(reading_status);
CREATE INDEX IF NOT EXISTS idx_papers_deleted_at ON papers(deleted_at);