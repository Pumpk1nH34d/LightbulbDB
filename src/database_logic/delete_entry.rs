use crate::database_logic::database::DataBase;
use rusqlite::{params, Result};

//todo: comment code

impl DataBase {
    pub fn delete_participant(&self, id: i32) -> Result<()> {
        self.connection
            .execute("DELETE FROM Participants WHERE id = ?1;", params![id])?;
        self.connection
            .execute("DELETE FROM Workshop__Participants WHERE participant = ?1;", params![id])?;
        Ok(())
    }

    pub fn delete_support_worker(&self, id: i32) -> Result<()> {
        let workshops_to_delete = self.get_filtered_workshops(format!("facilitator = '{}'", id), String::new());
        for workshop in workshops_to_delete {
            self.connection
                .execute("DELETE FROM Workshop__Support_Worker WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            self.connection
                .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            self.connection
                .execute("DELETE FROM Workshops WHERE id = ?1;", params![workshop.id.unwrap()])?;
        }
        self.connection
            .execute("DELETE FROM Support_Workers WHERE id = ?1;", params![id])?;
        Ok(())
    }

    pub fn delete_venues(&self, id: i32) -> Result<()> {
        let workshops_to_delete = self.get_filtered_workshops(format!("venue = '{}'", id), String::new());
        for workshop in workshops_to_delete {
            self.connection
                .execute("DELETE FROM Workshop__Support_Worker WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            self.connection
                .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            self.connection
                .execute("DELETE FROM Workshops WHERE id = ?1;", params![workshop.id.unwrap()])?;
        }
        self.connection
            .execute("DELETE FROM Venues WHERE id = ?1;", params![id])?;
        Ok(())
    }
    pub fn delete_workshop(&self, id: i32) -> Result<()> {
        self.connection
            .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![id])?;
        self.connection
            .execute("DELETE FROM Workshops WHERE id = ?1;", params![id])?;
        Ok(())
    }

    pub fn delete_workshop_participants(&self, id: i32) -> Result<()> {
        self.connection
            .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![id]).unwrap();
        Ok(())
    }

    pub fn delete_workshop_support_workers(&self, id: i32) -> Result<()> {
        self.connection
            .execute("DELETE FROM Workshop__Support_Worker WHERE workshop = ?1;", params![id]).unwrap();
        Ok(())
    }
}
