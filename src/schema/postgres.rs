use anyhow::Result;
use sqlx::postgres::PgPool;
use sqlx::Row;

use super::models::{Column, Constraint, ConstraintType, Index, Schema, Table};

/// Extracts schema from a PostgreSQL database
pub async fn extract_schema(connection_string: &str) -> Result<Schema> {
    let pool = PgPool::connect(connection_string).await?;

    let tables = extract_tables(&pool).await?;

    Ok(Schema::with_tables(tables))
}

/// Get all user tables (excluding system schemas)
async fn extract_tables(pool: &PgPool) -> Result<Vec<Table>> {
    let rows = sqlx::query(
        r#"
        SELECT table_schema, table_name
        FROM information_schema.tables
        WHERE table_type = 'BASE TABLE'
          AND table_schema NOT IN ('pg_catalog', 'information_schema')
        ORDER BY table_schema, table_name
        "#,
    )
    .fetch_all(pool)
    .await?;

    let mut tables = Vec::new();

    for row in rows {
        let schema: String = row.get("table_schema");
        let name: String = row.get("table_name");

        let columns = extract_columns(pool, &schema, &name).await?;
        let indexes = extract_indexes(pool, &schema, &name).await?;
        let constraints = extract_constraints(pool, &schema, &name).await?;

        tables.push(Table {
            name,
            schema,
            columns,
            indexes,
            constraints,
        });
    }

    Ok(tables)
}

/// Get all columns for a table
async fn extract_columns(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<Column>> {
    let rows = sqlx::query(
        r#"
        SELECT
            column_name,
            data_type,
            is_nullable,
            column_default,
            ordinal_position
        FROM information_schema.columns
        WHERE table_schema = $1 AND table_name = $2
        ORDER BY ordinal_position
        "#,
    )
    .bind(schema)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let columns = rows
        .iter()
        .map(|row| {
            let is_nullable: String = row.get("is_nullable");
            Column {
                name: row.get("column_name"),
                data_type: row.get("data_type"),
                is_nullable: is_nullable == "YES",
                default_value: row.get("column_default"),
                ordinal_position: row.get("ordinal_position"),
            }
        })
        .collect();

    Ok(columns)
}

/// Get all indexes for a table
async fn extract_indexes(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<Index>> {
    let rows = sqlx::query(
        r#"
        SELECT
            i.relname AS index_name,
            array_agg(a.attname ORDER BY array_position(ix.indkey, a.attnum)) AS columns,
            ix.indisunique AS is_unique,
            ix.indisprimary AS is_primary
        FROM pg_class t
        JOIN pg_index ix ON t.oid = ix.indrelid
        JOIN pg_class i ON i.oid = ix.indexrelid
        JOIN pg_namespace n ON n.oid = t.relnamespace
        JOIN pg_attribute a ON a.attrelid = t.oid AND a.attnum = ANY(ix.indkey)
        WHERE n.nspname = $1
          AND t.relname = $2
        GROUP BY i.relname, ix.indisunique, ix.indisprimary
        ORDER BY i.relname
        "#,
    )
    .bind(schema)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let indexes = rows
        .iter()
        .map(|row| Index {
            name: row.get("index_name"),
            columns: row.get("columns"),
            is_unique: row.get("is_unique"),
            is_primary: row.get("is_primary"),
        })
        .collect();

    Ok(indexes)
}

/// Get all constraints for a table
async fn extract_constraints(pool: &PgPool, schema: &str, table: &str) -> Result<Vec<Constraint>> {
    let rows = sqlx::query(
        r#"
        SELECT
            tc.constraint_name,
            tc.constraint_type,
            array_agg(DISTINCT kcu.column_name) AS columns,
            ccu.table_name AS foreign_table,
            array_agg(DISTINCT ccu.column_name) FILTER (WHERE tc.constraint_type = 'FOREIGN KEY') AS foreign_columns
        FROM information_schema.table_constraints tc
        JOIN information_schema.key_column_usage kcu
            ON tc.constraint_name = kcu.constraint_name
            AND tc.table_schema = kcu.table_schema
        LEFT JOIN information_schema.constraint_column_usage ccu
            ON tc.constraint_name = ccu.constraint_name
            AND tc.table_schema = ccu.table_schema
        WHERE tc.table_schema = $1
          AND tc.table_name = $2
          AND tc.constraint_type IN ('PRIMARY KEY', 'FOREIGN KEY', 'UNIQUE', 'CHECK')
        GROUP BY tc.constraint_name, tc.constraint_type, ccu.table_name
        ORDER BY tc.constraint_name
        "#,
    )
    .bind(schema)
    .bind(table)
    .fetch_all(pool)
    .await?;

    let constraints = rows
        .iter()
        .map(|row| {
            let constraint_type: String = row.get("constraint_type");
            let foreign_table: Option<String> = row.get("foreign_table");
            let foreign_columns: Option<Vec<String>> = row.get("foreign_columns");

            Constraint {
                name: row.get("constraint_name"),
                constraint_type: match constraint_type.as_str() {
                    "PRIMARY KEY" => ConstraintType::PrimaryKey,
                    "FOREIGN KEY" => ConstraintType::ForeignKey,
                    "UNIQUE" => ConstraintType::Unique,
                    "CHECK" => ConstraintType::Check,
                    _ => ConstraintType::Check,
                },
                columns: row.get("columns"),
                foreign_table: if constraint_type == "FOREIGN KEY" {
                    foreign_table
                } else {
                    None
                },
                foreign_columns: if constraint_type == "FOREIGN KEY" {
                    foreign_columns
                } else {
                    None
                },
            }
        })
        .collect();

    Ok(constraints)
}
