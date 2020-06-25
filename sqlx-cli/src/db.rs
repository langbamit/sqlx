use dialoguer::Confirmation;

use anyhow::bail;

pub async fn run_create() -> anyhow::Result<()> {
    let migrator = crate::migrator::get()?;

    if !migrator.can_create_database() {
        bail!(
            "Database creation is not implemented for {}",
            migrator.database_type()
        );
    }

    let db_name = migrator.get_database_name()?;
    let db_exists = migrator.check_if_database_exists(&db_name).await?;

    if !db_exists {
        println!("Creating database: {}", db_name);
        Ok(migrator.create_database(&db_name).await?)
    } else {
        println!("Database already exists, aborting");
        Ok(())
    }
}

pub async fn run_drop() -> anyhow::Result<()> {
    let migrator = crate::migrator::get()?;

    if !migrator.can_drop_database() {
        bail!(
            "Database drop is not implemented for {}",
            migrator.database_type()
        );
    }

    let db_name = migrator.get_database_name()?;
    let db_exists = migrator.check_if_database_exists(&db_name).await?;

    if db_exists {
        if !Confirmation::new()
            .with_text(&format!(
                "\nAre you sure you want to drop the database: {}?",
                db_name
            ))
            .default(false)
            .interact()?
        {
            println!("Aborting");
            return Ok(());
        }

        println!("Dropping database: {}", db_name);
        Ok(migrator.drop_database(&db_name).await?)
    } else {
        println!("Database does not exists, aborting");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[async_std::test]
    async fn test_create_postgres() {
        env::set_var("DATABASE_URL", "postgres://up:up@localhost:5432/up");
        match run_create().await {
            Ok(_) => {}
            Err(e) => {
                print!("{:?}", e);  
                assert!(false)
            }
        }
    }

    #[async_std::test]
    async fn test_drop_postgres() {
        env::set_var("DATABASE_URL", "postgres://up:up@localhost:5432/up");
        match run_drop().await {
            Ok(_) => {}
            Err(e) => {
                print!("{:?}", e);
                assert!(false)
            }
        }
    }

}