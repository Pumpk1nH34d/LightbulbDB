use crate::database_logic::database::DataBase;
use rusqlite::{params, Result};

// Methods for deleting records from the database

impl DataBase {
    // Deletes a participant by their ID.
    // This method removes the participant from the Participants table
    // and also removes any associated records from the Workshop__Participants table.
    pub fn delete_participant(&self, id: i32) -> Result<()> {
        // Delete participant from Participants table
        self.connection
            .execute("DELETE FROM Participants WHERE id = ?1;", params![id])?;
        // Remove any records from Workshop__Participants where this participant was involved
        self.connection
            .execute("DELETE FROM Workshop__Participants WHERE participant = ?1;", params![id])?;
        Ok(())
    }

    // Deletes a support worker by their ID.
    // This method removes the support worker from the Support_Workers table,
    // and also deletes any associated workshops and related records from Workshop__Support_Worker
    // and Workshop__Participants tables.
    pub fn delete_support_worker(&self, id: i32) -> Result<()> {
        // Retrieve all workshops where the support worker was a facilitator
        let workshops_to_delete = self.get_filtered_workshops(format!("facilitator = '{}'", id), String::new());
        for workshop in workshops_to_delete {
            // Remove support worker from Workshop__Support_Worker table for the current workshop
            self.connection
                .execute("DELETE FROM Workshop__Support_Worker WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            // Remove participants from Workshop__Participants table for the current workshop
            self.connection
                .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            // Delete the workshop from Workshops table
            self.connection
                .execute("DELETE FROM Workshops WHERE id = ?1;", params![workshop.id.unwrap()])?;
        }
        // Finally, delete the support worker from Support_Workers table
        self.connection
            .execute("DELETE FROM Support_Workers WHERE id = ?1;", params![id])?;
        Ok(())
    }

    // Deletes a venue by its ID.
    // This method removes the venue from the Venues table,
    // and also deletes any workshops associated with this venue.
    pub fn delete_venues(&self, id: i32) -> Result<()> {
        // Retrieve all workshops that were held at this venue
        let workshops_to_delete = self.get_filtered_workshops(format!("venue = '{}'", id), String::new());
        for workshop in workshops_to_delete {
            // Remove support workers from Workshop__Support_Worker table for the current workshop
            self.connection
                .execute("DELETE FROM Workshop__Support_Worker WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            // Remove participants from Workshop__Participants table for the current workshop
            self.connection
                .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![workshop.id.unwrap()])?;
            // Delete the workshop from Workshops table
            self.connection
                .execute("DELETE FROM Workshops WHERE id = ?1;", params![workshop.id.unwrap()])?;
        }
        // Finally, delete the venue from Venues table
        self.connection
            .execute("DELETE FROM Venues WHERE id = ?1;", params![id])?;
        Ok(())
    }

    // Deletes a workshop by its ID.
    // This method removes the workshop from the Workshops table
    // and also deletes any associated records from the Workshop__Participants table.
    pub fn delete_workshop(&self, id: i32) -> Result<()> {
        // Remove participants from Workshop__Participants table for this workshop
        self.connection
            .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![id])?;
        // Delete the workshop from Workshops table
        self.connection
            .execute("DELETE FROM Workshops WHERE id = ?1;", params![id])?;
        Ok(())
    }

    // Deletes all participants associated with a specific workshop.
    // This method removes records from the Workshop__Participants table where the workshop ID matches.
    pub fn delete_workshop_participants(&self, id: i32) -> Result<()> {
        // Remove participants from Workshop__Participants table for this workshop
        self.connection
            .execute("DELETE FROM Workshop__Participants WHERE workshop = ?1;", params![id]).unwrap();
        Ok(())
    }

    // Deletes all support workers associated with a specific workshop.
    // This method removes records from the Workshop__Support_Worker table where the workshop ID matches.
    pub fn delete_workshop_support_workers(&self, id: i32) -> Result<()> {
        // Remove support workers from Workshop__Support_Worker table for this workshop
        self.connection
            .execute("DELETE FROM Workshop__Support_Worker WHERE workshop = ?1;", params![id]).unwrap();
        Ok(())
    }
}
