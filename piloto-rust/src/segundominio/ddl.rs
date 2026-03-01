pub const SQL_LIST: &str = r#"
    SELECT id, name, created_at, updated_at
    FROM segundominio
    ORDER BY id LIMIT $1 OFFSET $2
"#;

pub const SQL_GET_BY_ID: &str = r#"
    SELECT id, name, created_at, updated_at
    FROM segundominio WHERE id = $1
"#;

pub const SQL_INSERT: &str = r#"
    INSERT INTO segundominio (name, created_at, updated_at)
    VALUES ($1, $2, $3)
    RETURNING id
"#;

pub const SQL_UPDATE: &str = r#"
    UPDATE segundominio SET name = $1, updated_at = $2
    WHERE id = $3
"#;

pub const SQL_DELETE: &str = "DELETE FROM segundominio WHERE id = $1";
