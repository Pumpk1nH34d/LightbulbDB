use rusqlite::{Result, params};
use crate::database_logic::database::DataBase;

impl DataBase {
    pub fn delete_participant(&self, id: i32) -> Result<()> {
        self.connection.execute(
            "DELETE FROM Participants WHERE id = ?1;",
            params![
                id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_support_worker(&self, id: i32) -> Result<()> {
        self.connection.execute(
            "DELETE FROM Support_Workers WHERE id = ?1;",
            params![
                id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_venues(&self, id: i32) -> Result<()> {
        self.connection.execute(
            "DELETE FROM Venues WHERE id = ?1;",
            params![
                id,
            ],
        )?;
        Ok(())
    }
}


