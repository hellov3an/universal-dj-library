-- Universal DJ Library — Initial Schema
-- Migration 001: Core tables for library management

-- Tracks: the central entity
CREATE TABLE IF NOT EXISTS tracks (
    id              TEXT PRIMARY KEY,
    file_path       TEXT NOT NULL,
    relative_path   TEXT,
    filename        TEXT NOT NULL,
    title           TEXT,
    artist          TEXT,
    album           TEXT,
    album_artist    TEXT,
    genre           TEXT,
    year            INTEGER,
    label           TEXT,
    composer        TEXT,
    comment         TEXT,
    grouping        TEXT,
    track_number    INTEGER,
    disc_number     INTEGER,
    duration_ms     INTEGER NOT NULL DEFAULT 0,
    bitrate         INTEGER,
    sample_rate     INTEGER,
    format          TEXT NOT NULL DEFAULT 'Unknown',
    file_size       INTEGER NOT NULL DEFAULT 0,
    file_hash       TEXT,
    audio_fingerprint TEXT,
    bpm             REAL,
    musical_key     TEXT,
    energy          INTEGER,
    rating          INTEGER NOT NULL DEFAULT 0,
    color           TEXT,
    artwork_hash    TEXT,
    date_added      TEXT NOT NULL,
    date_modified   TEXT NOT NULL,
    date_analyzed   TEXT
);

-- File identity for relinking after moves/renames
CREATE TABLE IF NOT EXISTS file_identities (
    id              TEXT PRIMARY KEY,
    track_id        TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    hash            TEXT,
    fingerprint     TEXT,
    duration_ms     INTEGER,
    file_size       INTEGER,
    metadata_sig    TEXT,
    UNIQUE(track_id)
);

-- Tags
CREATE TABLE IF NOT EXISTS tags (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE,
    color           TEXT
);

CREATE TABLE IF NOT EXISTS track_tags (
    track_id        TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    tag_id          TEXT NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (track_id, tag_id)
);

-- Playlists (supports folders and smart playlists)
CREATE TABLE IF NOT EXISTS playlists (
    id              TEXT PRIMARY KEY,
    name            TEXT NOT NULL,
    parent_id       TEXT REFERENCES playlists(id) ON DELETE CASCADE,
    is_folder       INTEGER NOT NULL DEFAULT 0,
    is_smart        INTEGER NOT NULL DEFAULT 0,
    smart_rules     TEXT,
    sort_order      INTEGER NOT NULL DEFAULT 0,
    date_created    TEXT NOT NULL,
    date_modified   TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS playlist_items (
    id              TEXT PRIMARY KEY,
    playlist_id     TEXT NOT NULL REFERENCES playlists(id) ON DELETE CASCADE,
    track_id        TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    sort_order      INTEGER NOT NULL,
    date_added      TEXT NOT NULL
);

-- Cue points (hot cues, memory cues, and loops)
CREATE TABLE IF NOT EXISTS cue_points (
    id              TEXT PRIMARY KEY,
    track_id        TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    position_ms     REAL NOT NULL,
    cue_type        TEXT NOT NULL,
    name            TEXT,
    color           TEXT,
    hot_cue_index   INTEGER,
    loop_end_ms     REAL,
    loop_length_ms  REAL
);

-- Beatgrids
CREATE TABLE IF NOT EXISTS beatgrids (
    id              TEXT PRIMARY KEY,
    track_id        TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE UNIQUE,
    bpm             REAL NOT NULL,
    offset_ms       REAL NOT NULL,
    markers         TEXT NOT NULL
);

-- Analysis data (waveform, loudness, energy)
CREATE TABLE IF NOT EXISTS analysis (
    id              TEXT PRIMARY KEY,
    track_id        TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE UNIQUE,
    waveform_data   BLOB,
    waveform_overview BLOB,
    loudness_db     REAL,
    energy          INTEGER,
    analysis_version TEXT NOT NULL
);

-- Artwork storage
CREATE TABLE IF NOT EXISTS artwork (
    id              TEXT PRIMARY KEY,
    hash            TEXT NOT NULL UNIQUE,
    data            BLOB NOT NULL,
    mime_type       TEXT NOT NULL,
    width           INTEGER,
    height          INTEGER
);

-- Application settings
CREATE TABLE IF NOT EXISTS settings (
    key             TEXT PRIMARY KEY,
    value           TEXT NOT NULL
);

-- History log
CREATE TABLE IF NOT EXISTS history (
    id              TEXT PRIMARY KEY,
    event_type      TEXT NOT NULL,
    description     TEXT NOT NULL,
    details         TEXT,
    timestamp       TEXT NOT NULL
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_tracks_artist ON tracks(artist);
CREATE INDEX IF NOT EXISTS idx_tracks_title ON tracks(title);
CREATE INDEX IF NOT EXISTS idx_tracks_genre ON tracks(genre);
CREATE INDEX IF NOT EXISTS idx_tracks_bpm ON tracks(bpm);
CREATE INDEX IF NOT EXISTS idx_tracks_key ON tracks(musical_key);
CREATE INDEX IF NOT EXISTS idx_tracks_rating ON tracks(rating);
CREATE INDEX IF NOT EXISTS idx_tracks_hash ON tracks(file_hash);
CREATE INDEX IF NOT EXISTS idx_tracks_date_added ON tracks(date_added);
CREATE INDEX IF NOT EXISTS idx_tracks_format ON tracks(format);
CREATE INDEX IF NOT EXISTS idx_file_identities_hash ON file_identities(hash);
CREATE INDEX IF NOT EXISTS idx_file_identities_fingerprint ON file_identities(fingerprint);
CREATE INDEX IF NOT EXISTS idx_playlist_items_playlist ON playlist_items(playlist_id, sort_order);
CREATE INDEX IF NOT EXISTS idx_cue_points_track ON cue_points(track_id);
CREATE INDEX IF NOT EXISTS idx_history_timestamp ON history(timestamp);

-- Full-text search index
CREATE VIRTUAL TABLE IF NOT EXISTS tracks_fts USING fts5(
    title, artist, album, genre, comment, grouping, label, composer,
    content='tracks',
    content_rowid='rowid'
);

-- Triggers to keep FTS in sync
CREATE TRIGGER IF NOT EXISTS tracks_ai AFTER INSERT ON tracks BEGIN
    INSERT INTO tracks_fts(rowid, title, artist, album, genre, comment, grouping, label, composer)
    VALUES (new.rowid, new.title, new.artist, new.album, new.genre, new.comment, new.grouping, new.label, new.composer);
END;

CREATE TRIGGER IF NOT EXISTS tracks_ad AFTER DELETE ON tracks BEGIN
    INSERT INTO tracks_fts(tracks_fts, rowid, title, artist, album, genre, comment, grouping, label, composer)
    VALUES ('delete', old.rowid, old.title, old.artist, old.album, old.genre, old.comment, old.grouping, old.label, old.composer);
END;

CREATE TRIGGER IF NOT EXISTS tracks_au AFTER UPDATE ON tracks BEGIN
    INSERT INTO tracks_fts(tracks_fts, rowid, title, artist, album, genre, comment, grouping, label, composer)
    VALUES ('delete', old.rowid, old.title, old.artist, old.album, old.genre, old.comment, old.grouping, old.label, old.composer);
    INSERT INTO tracks_fts(rowid, title, artist, album, genre, comment, grouping, label, composer)
    VALUES (new.rowid, new.title, new.artist, new.album, new.genre, new.comment, new.grouping, new.label, new.composer);
END;
