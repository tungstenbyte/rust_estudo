pub(super) const SQL_LIST: &str = r#"
    SELECT id, status_code, name, description, allows_transactions,
           max_transaction_amount, created_at, updated_at
    FROM meuexemplo
    ORDER BY id LIMIT $1 OFFSET $2
"#;

pub(super) const SQL_GET_BY_ID: &str = r#"
    SELECT id, status_code, name, description, allows_transactions,
           max_transaction_amount, created_at, updated_at
    FROM meuexemplo WHERE id = $1
"#;

pub(super) const SQL_GET_BY_STATUS_CODE: &str = r#"
    SELECT id, status_code, name, description, allows_transactions,
           max_transaction_amount, created_at, updated_at
    FROM meuexemplo WHERE status_code = $1
"#;

pub(super) const SQL_INSERT: &str = r#"
    INSERT INTO meuexemplo (status_code, name, description, allows_transactions,
                           max_transaction_amount, created_at, updated_at)
    VALUES ($1, $2, $3, $4, $5, $6, $7)
    RETURNING id
"#;

pub(super) const SQL_UPDATE: &str = r#"
    UPDATE meuexemplo SET
        status_code = $1, name = $2, description = $3,
        allows_transactions = $4, max_transaction_amount = $5,
        updated_at = $6
    WHERE id = $7
"#;

pub(super) const SQL_DELETE: &str = "DELETE FROM meuexemplo WHERE id = $1";
