use bytesize::ByteSize;
use rusqlite::{params, Connection};

use crate::{args::ARGS, pasta::PastaFile, Pasta};

pub fn read_all() -> Vec<Pasta> {
    select_all_from_db()
}

pub fn update_all(pastas: &[Pasta]) {
    rewrite_all_to_db(pastas);
}

pub fn rewrite_all_to_db(pasta_data: &[Pasta]) {
    let conn = Connection::open(format!("{}/database.sqlite", ARGS.data_dir))
        .expect("Failed to open SQLite database!");

    conn.execute(
        "
        DROP TABLE IF EXISTS pasta;
        );",
        params![],
    )
    .expect("Failed to drop SQLite table for Pasta!");

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS pasta (
            id INTEGER PRIMARY KEY,
            title TEXT,
            content TEXT NOT NULL,
            file_name TEXT,
            file_size INTEGER,
            extension TEXT NOT NULL,
            read_only INTEGER NOT NULL,
            private INTEGER NOT NULL,
            editable INTEGER NOT NULL,
            encrypt_server INTEGER NOT NULL,
            encrypt_client INTEGER NOT NULL,
            encrypted_key TEXT,
            created INTEGER NOT NULL,
            expiration INTEGER NOT NULL,
            last_read INTEGER NOT NULL,
            read_count INTEGER NOT NULL,
            burn_after_reads INTEGER NOT NULL,
            pasta_type TEXT NOT NULL
        );",
        params![],
    )
    .expect("Failed to create SQLite table for Pasta!");

    for pasta in pasta_data.iter() {
        conn.execute(
            "INSERT INTO pasta (
                id,
                title,
                content,
                file_name,
                file_size,
                extension,
                private,
                read_only,
                editable,
                encrypt_server,
                encrypt_client,
                encrypted_key,
                created,
                expiration,
                last_read,
                read_count,
                burn_after_reads,
                pasta_type
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                pasta.id,
                pasta.title,
                pasta.content,
                pasta.file.as_ref().map_or("", |f| f.name.as_str()),
                pasta.file.as_ref().map_or(0, |f| f.size.as_u64()),
                pasta.extension,
                pasta.private as i32,
                pasta.readonly as i32,
                pasta.editable as i32,
                pasta.encrypt_server as i32,
                pasta.encrypt_client as i32,
                pasta.encrypted_key.as_deref(),
                pasta.created,
                pasta.expiration,
                pasta.last_read,
                pasta.read_count,
                pasta.burn_after_reads,
                pasta.pasta_type,
            ],
        )
        .expect("Failed to insert pasta.");
    }
}

pub fn select_all_from_db() -> Vec<Pasta> {
    let conn = Connection::open(format!("{}/database.sqlite", ARGS.data_dir))
        .expect("Failed to open SQLite database!");

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS pasta (
            id INTEGER PRIMARY KEY,
            title TEXT,
            content TEXT NOT NULL,
            file_name TEXT,
            file_size INTEGER,
            extension TEXT NOT NULL,
            read_only INTEGER NOT NULL,
            private INTEGER NOT NULL,
            editable INTEGER NOT NULL,
            encrypt_server INTEGER NOT NULL,
            encrypt_client INTEGER NOT NULL,
            encrypted_key TEXT,
            created INTEGER NOT NULL,
            expiration INTEGER NOT NULL,
            last_read INTEGER NOT NULL,
            read_count INTEGER NOT NULL,
            burn_after_reads INTEGER NOT NULL,
            pasta_type TEXT NOT NULL
        );",
        params![],
    )
    .expect("Failed to create SQLite table for Pasta!");

    // Migration: Add title column if it doesn't exist
    let column_exists: Result<i32, _> = conn.query_row(
        "SELECT COUNT(*) FROM pragma_table_info('pasta') WHERE name='title'",
        [],
        |row| row.get(0),
    );
    
    if let Ok(count) = column_exists {
        if count == 0 {
            log::info!("Migrating database: Adding 'title' column to pasta table");
            conn.execute("ALTER TABLE pasta ADD COLUMN title TEXT", params![])
                .expect("Failed to add title column to pasta table");
        }
    }

    let mut stmt = conn
        .prepare("SELECT * FROM pasta ORDER BY created ASC")
        .expect("Failed to prepare SQL statement to load pastas");

    let pasta_iter = stmt
        .query_map([], |row| {
            Ok(Pasta {
                id: row.get(0)?,
                title: row.get::<_, Option<String>>(1)?.unwrap_or_else(|| format!("Pasta {}", row.get::<_, u64>(0).unwrap_or(0))),
                content: row.get(2)?,
                file: {
                    let file_name: Option<String> = row.get(3).ok();
                    let file_size: Option<i64> = row.get(4).ok();
                    
                    if let (Some(name), Some(size)) = (file_name, file_size) {
                        if !name.is_empty() && size > 0 {
                            Some(PastaFile {
                                name,
                                size: ByteSize::b(size as u64),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                extension: row.get(5)?,
                readonly: row.get(6)?,
                private: row.get(7)?,
                editable: row.get(8)?,
                encrypt_server: row.get(9)?,
                encrypt_client: row.get(10)?,
                encrypted_key: row.get(11)?,
                created: row.get(12)?,
                expiration: row.get(13)?,
                last_read: row.get(14)?,
                read_count: row.get(15)?,
                burn_after_reads: row.get(16)?,
                pasta_type: row.get(17)?,
            })
        })
        .expect("Failed to select Pastas from SQLite database.");

    pasta_iter
        .map(|r| r.expect("Failed to get pasta"))
        .collect::<Vec<Pasta>>()
}

pub fn insert(pasta: &Pasta) {
    let conn = Connection::open(format!("{}/database.sqlite", ARGS.data_dir))
        .expect("Failed to open SQLite database!");

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS pasta (
            id INTEGER PRIMARY KEY,
            title TEXT,
            content TEXT NOT NULL,
            file_name TEXT,
            file_size INTEGER,
            extension TEXT NOT NULL,
            read_only INTEGER NOT NULL,
            private INTEGER NOT NULL,
            editable INTEGER NOT NULL,
            encrypt_server INTEGER NOT NULL,
            encrypt_client INTEGER NOT NULL,
            encrypted_key TEXT,
            created INTEGER NOT NULL,
            expiration INTEGER NOT NULL,
            last_read INTEGER NOT NULL,
            read_count INTEGER NOT NULL,
            burn_after_reads INTEGER NOT NULL,
            pasta_type TEXT NOT NULL
        );",
        params![],
    )
    .expect("Failed to create SQLite table for Pasta!");

    conn.execute(
        "INSERT INTO pasta (
                id,
                title,
                content,
                file_name,
                file_size,
                extension,
                read_only,
                private,
                editable,
                encrypt_server,
                encrypt_client,
                encrypted_key,
                created,
                expiration,
                last_read,
                read_count,
                burn_after_reads,
                pasta_type
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
        params![
            pasta.id,
            pasta.title,
            pasta.content,
            pasta.file.as_ref().map_or("", |f| f.name.as_str()),
            pasta.file.as_ref().map_or(0, |f| f.size.as_u64()),
            pasta.extension,
            pasta.readonly as i32,
            pasta.private as i32,
            pasta.editable as i32,
            pasta.encrypt_server as i32,
            pasta.encrypt_client as i32,
            pasta.encrypted_key.as_deref(),
            pasta.created,
            pasta.expiration,
            pasta.last_read,
            pasta.read_count,
            pasta.burn_after_reads,
            pasta.pasta_type,
        ],
    )
    .expect("Failed to insert pasta.");
}

pub fn update(pasta: &Pasta) {
    let conn = Connection::open(format!("{}/database.sqlite", ARGS.data_dir))
        .expect("Failed to open SQLite database!");

    conn.execute(
        "UPDATE pasta SET
            title = ?2,
            content = ?3,
            file_name = ?4,
            file_size = ?5,
            extension = ?6,
            read_only = ?7,
            private = ?8,
            editable = ?9,
            encrypt_server = ?10,
            encrypt_client = ?11,
            encrypted_key = ?12,
            created = ?13,
            expiration = ?14,
            last_read = ?15,
            read_count = ?16,
            burn_after_reads = ?17,
            pasta_type = ?18
        WHERE id = ?1;",
        params![
            pasta.id,
            pasta.title,
            pasta.content,
            pasta.file.as_ref().map_or("", |f| f.name.as_str()),
            pasta.file.as_ref().map_or(0, |f| f.size.as_u64()),
            pasta.extension,
            pasta.readonly as i32,
            pasta.private as i32,
            pasta.editable as i32,
            pasta.encrypt_server as i32,
            pasta.encrypt_client as i32,
            pasta.encrypted_key.as_deref(),
            pasta.created,
            pasta.expiration,
            pasta.last_read,
            pasta.read_count,
            pasta.burn_after_reads,
            pasta.pasta_type,
        ],
    )
    .expect("Failed to update pasta.");
}

pub fn delete_by_id(id: u64) {
    let conn = Connection::open(format!("{}/database.sqlite", ARGS.data_dir))
        .expect("Failed to open SQLite database!");

    conn.execute(
        "DELETE FROM pasta 
        WHERE id = ?1;",
        params![id],
    )
    .expect("Failed to delete pasta.");
}
