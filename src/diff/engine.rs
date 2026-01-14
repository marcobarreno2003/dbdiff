use crate::schema::models::{Schema, Table, Column};

/// Result of comparing two schemas
#[derive(Debug)]
pub struct SchemaDiff {
    pub tables_added: Vec<Table>,
    pub tables_removed: Vec<Table>,
    pub tables_modified: Vec<TableDiff>,
}

/// Changes within a single table
#[derive(Debug)]
pub struct TableDiff {
    pub table_name: String,
    pub columns_added: Vec<Column>,
    pub columns_removed: Vec<Column>,
    pub columns_modified: Vec<ColumnDiff>,
}

/// Changes to a single column
#[derive(Debug)]
pub struct ColumnDiff {
    pub column_name: String,
    pub old: Column,
    pub new: Column,
}

impl SchemaDiff {
    pub fn compare(old: &Schema, new: &Schema) -> Self {
        let mut diff = SchemaDiff {
            tables_added: Vec::new(),
            tables_removed: Vec::new(),
            tables_modified: Vec::new(),
        };

        // Find added and modified tables
        for new_table in &new.tables {
            match old.tables.iter().find(|t| t.name == new_table.name) {
                Some(old_table) => {
                    let table_diff = TableDiff::compare(old_table, new_table);
                    if table_diff.has_changes() {
                        diff.tables_modified.push(table_diff);
                    }
                }
                None => {
                    diff.tables_added.push(new_table.clone());
                }
            }
        }

        // Find removed tables
        for old_table in &old.tables {
            if !new.tables.iter().any(|t| t.name == old_table.name) {
                diff.tables_removed.push(old_table.clone());
            }
        }

        diff
    }

    pub fn has_changes(&self) -> bool {
        !self.tables_added.is_empty()
            || !self.tables_removed.is_empty()
            || !self.tables_modified.is_empty()
    }
}

impl TableDiff {
    pub fn compare(old: &Table, new: &Table) -> Self {
        let mut diff = TableDiff {
            table_name: new.name.clone(),
            columns_added: Vec::new(),
            columns_removed: Vec::new(),
            columns_modified: Vec::new(),
        };

        // Find added and modified columns
        for new_col in &new.columns {
            match old.columns.iter().find(|c| c.name == new_col.name) {
                Some(old_col) => {
                    if old_col != new_col {
                        diff.columns_modified.push(ColumnDiff {
                            column_name: new_col.name.clone(),
                            old: old_col.clone(),
                            new: new_col.clone(),
                        });
                    }
                }
                None => {
                    diff.columns_added.push(new_col.clone());
                }
            }
        }

        // Find removed columns
        for old_col in &old.columns {
            if !new.columns.iter().any(|c| c.name == old_col.name) {
                diff.columns_removed.push(old_col.clone());
            }
        }

        diff
    }

    pub fn has_changes(&self) -> bool {
        !self.columns_added.is_empty()
            || !self.columns_removed.is_empty()
            || !self.columns_modified.is_empty()
    }
}
