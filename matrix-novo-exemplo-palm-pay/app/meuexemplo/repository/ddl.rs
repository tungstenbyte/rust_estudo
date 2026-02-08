// SQL queries - equivalente às constantes Go
const SQL_MEUEXEMPLO_LIST: &str = r#"
    SELECT
        id,
        status_code,
        name,
        description,
        allows_transactions,
        max_transaction_amount,
        created_at,
        updated_at
    FROM meuexemplo
    ORDER BY id LIMIT $1 OFFSET $2
"#;

const SQL_MEUEXEMPLO_INSERT: &str = r#"
    INSERT INTO meuexemplo(
        status_code,
        name,
        description,
        allows_transactions,
        max_transaction_amount,
        created_at,
        updated_at)
    VALUES(
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7) RETURNING id
"#;

const SQL_MEUEXEMPLO_UPDATE: &str = r#"
    UPDATE meuexemplo SET
        status_code = $1,
        name = $2,
        description = $3,
        allows_transactions = $4,
        max_transaction_amount = $5,
        created_at = $6,
        updated_at = $7
    WHERE id = $8
"#;

const SQL_MEUEXEMPLO_DELETE_BY_ID: &str = r#"
    DELETE FROM meuexemplo
    WHERE id = $1
"#;

const SQL_GET_MEUEXEMPLO_BY_ID: &str = r#"
    SELECT
        id,
        status_code,
        name,
        description,
        allows_transactions,
        max_transaction_amount,
        created_at,
        updated_at
    FROM meuexemplo
    WHERE id = $1
"#;

const SQL_GET_MEUEXEMPLO_BY_STATUS_CODE: &str = r#"
    SELECT
        id,
        status_code,
        name,
        description,
        allows_transactions,
        max_transaction_amount,
        created_at,
        updated_at
    FROM meuexemplo
    WHERE status_code LIKE '%' || $1 || '%'
"#;