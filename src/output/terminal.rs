use colored::*;
use crate::diff::engine::{SchemaDiff, TableDiff, ColumnDiff};

/// Prints a schema diff to the terminal with colors
pub fn print_diff(diff: &SchemaDiff) {
    if !diff.has_changes() {
        println!("{}", "No changes detected".green());
        return;
    }

    // Print added tables
    for table in &diff.tables_added {
        println!("{} {}", "+".green().bold(), table.name.green());
        for col in &table.columns {
            println!("  {} {} ({})", "+".green(), col.name, col.data_type);
        }
    }

    // Print removed tables
    for table in &diff.tables_removed {
        println!("{} {}", "-".red().bold(), table.name.red());
        for col in &table.columns {
            println!("  {} {} ({})", "-".red(), col.name, col.data_type);
        }
    }

    // Print modified tables
    for table_diff in &diff.tables_modified {
        print_table_diff(table_diff);
    }
}

fn print_table_diff(diff: &TableDiff) {
    println!("{} {}", "~".yellow().bold(), diff.table_name.yellow());

    for col in &diff.columns_added {
        println!("  {} {} ({})", "+".green(), col.name.green(), col.data_type);
    }

    for col in &diff.columns_removed {
        println!("  {} {} ({})", "-".red(), col.name.red(), col.data_type);
    }

    for col_diff in &diff.columns_modified {
        print_column_diff(col_diff);
    }
}

fn print_column_diff(diff: &ColumnDiff) {
    println!("  {} {}", "~".yellow(), diff.column_name.yellow());

    if diff.old.data_type != diff.new.data_type {
        println!(
            "    type: {} -> {}",
            diff.old.data_type.red(),
            diff.new.data_type.green()
        );
    }

    if diff.old.is_nullable != diff.new.is_nullable {
        let old_null = if diff.old.is_nullable { "NULL" } else { "NOT NULL" };
        let new_null = if diff.new.is_nullable { "NULL" } else { "NOT NULL" };
        println!("    nullable: {} -> {}", old_null.red(), new_null.green());
    }

    if diff.old.default_value != diff.new.default_value {
        let old_default = diff.old.default_value.as_deref().unwrap_or("(none)");
        let new_default = diff.new.default_value.as_deref().unwrap_or("(none)");
        println!("    default: {} -> {}", old_default.red(), new_default.green());
    }
}
