// Import necessary crates and modules
use crate::database_logic::data_structs::{Participant, SupportWorker, Venue, Workshop}; // Import data structures for participants, support workers, venues, and workshops
use crate::database_logic::database::DataBase; // Import the DataBase struct
use rusqlite::{params, Result}; // Import necessary components from rusqlite

// Implement methods for the DataBase struct
impl DataBase {

    // Method to add a participant to the database
    pub fn add_participant(&self, participant: Participant) -> Result<()> {
        self.connection.execute(
            // SQL query to insert a new participant into the Participants table
            "INSERT INTO Participants (first_name, last_name, medicare_number, dob, address, suburb, postcode, phone, email, medical_notes, dietary_notes, physical_notes, other_notes, support_ratio, photo_permission, private_hospital_preference, private_health_insurer, private_health_number, communication_preference, ndis_plan_number, ndis_plan_start_date, core_funding, capacity_building_funding, self_managed, plan_managed, ndis_plan_end_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)",
            params![
                // Map each field of the participant to the corresponding placeholder in the SQL query
                participant.first_name,
                participant.last_name,
                participant.medicare_number,
                participant.dob.map(|value| value.to_string()), // Convert Option<Date> to Option<String>
                participant.address.map(|value| value.to_string()),
                participant.suburb.map(|value| value.to_string()),
                participant.postcode.map(|value| value.to_string()),
                participant.phone.map(|value| value.to_string()),
                participant.email.map(|value| value.to_string()),
                participant.medical_notes.map(|value| value.to_string()),
                participant.dietary_notes.map(|value| value.to_string()),
                participant.physical_notes.map(|value| value.to_string()),
                participant.other_notes.map(|value| value.to_string()),
                participant.support_ratio.map(|value| value.to_string()),
                participant.photo_permission.map(|value| { match value { true => 1, false => 0 } }), // Convert boolean to integer
                participant.private_hospital_preference.map(|value| { match value { true => 1, false => 0 } }),
                participant.private_health_insurer.map(|value| value.to_string()),
                participant.private_health_number.map(|value| value.to_string()),
                participant.communication_preference.map(|value| value.to_string()),
                participant.ndis_plan_number.map(|value| value.to_string()),
                participant.ndis_plan_start_date.map(|value| value.to_string()),
                participant.core_funding.map(|value| { match value { true => 1, false => 0 } }),
                participant.capacity_building_funding.map(|value| { match value { true => 1, false => 0 } }),
                participant.self_managed.map(|value| { match value { true => 1, false => 0 } }),
                participant.plan_managed.map(|value| { match value { true => 1, false => 0 } }),
                participant.ndis_plan_end_date.map(|value| value.to_string()),
            ],
        )?;
        Ok(())
    }

    // Method to add a support worker to the database
    pub fn add_support_worker(&self, support_worker: SupportWorker) -> Result<()> {
        self.connection.execute(
            // SQL query to insert a new support worker into the Support_Workers table
            "INSERT INTO Support_Workers (first_name, last_name, phone, email, dob, address, suburb, postcode, first_aid, confidentiality_agreement, police_clearance, car_insurance, other_qualifications, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                // Map each field of the support worker to the corresponding placeholder in the SQL query
                support_worker.first_name,
                support_worker.last_name,
                support_worker.email,
                support_worker.phone,
                support_worker.dob.map(|value| value.to_string()), // Convert Option<Date> to Option<String>
                support_worker.address.map(|value| value.to_string()),
                support_worker.suburb.map(|value| value.to_string()),
                support_worker.postcode.map(|value| value.to_string()),
                support_worker.first_aid.map(|value| value.to_string()),
                support_worker.confidentiality_agreement.map(|value| value.to_string()),
                support_worker.police_clearance.map(|value| value.to_string()),
                support_worker.car_insurance.map(|value| value.to_string()),
                support_worker.other_qualifications.map(|value| value.to_string()),
                support_worker.notes.map(|value| value.to_string()),
            ],
        )?;
        Ok(())
    }

    // Method to add a venue to the database
    pub fn add_venue(&self, venue: Venue) -> Result<()> {
        self.connection.execute(
            // SQL query to insert a new venue into the Venues table
            "INSERT INTO Venues (name, address, suburb, postcode, state, description, contact_person_name, contact_person_phone, venue_phone_number, price, notes) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                // Map each field of the venue to the corresponding placeholder in the SQL query
                venue.name,
                venue.address.map(|value| value.to_string()),
                venue.suburb.map(|value| value.to_string()),
                venue.postcode.map(|value| value.to_string()),
                venue.state.map(|value| value.to_string()),
                venue.description.map(|value| value.to_string()),
                venue.contact_person_name.map(|value| value.to_string()),
                venue.contact_person_phone.map(|value| value.to_string()),
                venue.venue_phone_number.map(|value| value.to_string()),
                venue.price.map(|value| value.to_string()),
                venue.notes.map(|value| value.to_string()),
            ],
        )?;
        Ok(())
    }

    // Method to add a workshop to the database
    pub fn add_workshop(&self, workshop: Workshop) -> Result<()> {
        self.connection.execute(
            // SQL query to insert a new workshop into the Workshops table
            "INSERT INTO Workshops (name, facilitator, venue, start_date, end_date) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                // Map each field of the workshop to the corresponding placeholder in the SQL query
                workshop.name,
                workshop.facilitator,
                workshop.venue,
                workshop.start_date.to_string(),
                workshop.end_date.to_string()
            ],
        )?;
        Ok(())
    }

    // Method to add participants to a workshop
    pub fn add_participants_to_workshop(&self, participants: &Vec<i32>, workshop: i32) -> Result<()> {
        for participant in participants {
            self.connection.execute(
                // SQL query to insert each participant into the Workshop__Participants table
                "INSERT INTO Workshop__Participants (workshop, participant) VALUES (?1, ?2)",
                params![
                workshop,
                participant
            ],
            )?;
        }
        Ok(())
    }

    // Method to add support workers to a workshop
    pub fn add_support_workers_to_workshop(&self, support_workers: &Vec<i32>, workshop: i32) -> Result<()> {
        for support_worker in support_workers {
            self.connection.execute(
                // SQL query to insert each support worker into the Workshop__Support_Worker table
                "INSERT INTO Workshop__Support_Worker (workshop, support_worker) VALUES (?1, ?2)",
                params![
                workshop,
                support_worker
            ],
            )?;
        }
        Ok(())
    }
}
