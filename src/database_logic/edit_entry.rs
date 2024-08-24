use crate::database_logic::data_structs::{Participant, SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use rusqlite::{params, Result};

impl DataBase {
    pub fn edit_participant(&self, participant: Participant) -> Result<()> {
        self.connection.execute(
            "UPDATE Participants SET first_name = ?1, last_name = ?2, medicare_number = ?3, dob = ?4, address = ?5, suburb = ?6, postcode = ?7, phone = ?8, email = ?9, medical_notes = ?10, dietary_notes = ?11, physical_notes = ?12, other_notes = ?13, support_ratio = ?14, photo_permission = ?15, private_hospital_preference = ?16, private_health_insurer = ?17, private_health_number = ?18, communication_preference = ?19, ndis_plan_number = ?20, ndis_plan_start_date = ?21, core_funding = ?22, capacity_building_funding = ?23, self_managed = ?24, plan_managed = ?25, ndis_plan_end_date = ?26 WHERE id = ?27",
            params![
                participant.first_name,
                participant.last_name,
                participant.medicare_number,
                participant.dob.map(|value| value.to_string()),
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
                participant.photo_permission,
                participant.private_hospital_preference.map(|value| value.to_string()),
                participant.private_health_insurer.map(|value| value.to_string()),
                participant.private_health_number.map(|value| value.to_string()),
                participant.communication_preference.map(|value| value.to_string()),
                participant.ndis_plan_number.map(|value| value.to_string()),
                participant.ndis_plan_start_date.map(|value| value.to_string()),
                participant.core_funding.map(|value| value.to_string()),
                participant.capacity_building_funding.map(|value| value.to_string()),
                participant.self_managed.map(|value| value.to_string()),
                participant.plan_managed.map(|value| value.to_string()),
                participant.ndis_plan_end_date.map(|value| value.to_string()),
                participant.id,
            ],
        )?;
        Ok(())
    }

    pub fn edit_support_worker(&self, support_worker: SupportWorker) -> Result<()> {
        self.connection.execute(
            "UPDATE Support_Workers SET first_name = ?1, last_name = ?2, phone = ?3, email = ?4, dob = ?5, address = ?6, suburb = ?7, postcode = ?8, first_aid = ?9, confidentiality_agreement = ?10, police_clearance = ?11, car_insurance = ?12, other_qualifications = ?13, notes = ?14 WHERE id = ?15",
            params![
                support_worker.first_name,
                support_worker.last_name,
                support_worker.phone,
                support_worker.email,
                support_worker.dob.map(|value| value.to_string()),
                support_worker.address.map(|value| value.to_string()),
                support_worker.suburb.map(|value| value.to_string()),
                support_worker.postcode.map(|value| value.to_string()),
                support_worker.first_aid.map(|value| value.to_string()),
                support_worker.confidentiality_agreement.map(|value| value.to_string()),
                support_worker.police_clearance.map(|value| value.to_string()),
                support_worker.car_insurance.map(|value| value.to_string()),
                support_worker.other_qualifications.map(|value| value.to_string()),
                support_worker.notes.map(|value| value.to_string()),
                support_worker.id
            ],
        )?;
        Ok(())
    }

    pub fn edit_venue(&self, venue: Venue) -> Result<()> {
        self.connection.execute(
            "UPDATE Venues SET name = ?1, address = ?2, suburb = ?3, postcode = ?4, state = ?5, description = ?6, contact_person_name = ?7, contact_person_phone = ?8, venue_phone_number = ?9, price = ?10, notes = ?11 WHERE id = ?12",
            params![
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
                venue.id
            ],
        ).unwrap();
        Ok(())
    }

    pub fn edit_workshop(&self, workshop: Workshop) -> Result<()> {
        self.connection.execute(
            "UPDATE Venues SET name = ?1, facilitator = ?2, venue = ?3, start_date = ?4, end_date = ?5 WHERE id = ?6",
            params![
                workshop.name,
                workshop.facilitator,
                workshop.venue,
                workshop.start_date.to_string(),
                workshop.end_date.to_string(),
                workshop.id
            ],
        ).unwrap();
        Ok(())
    }
}
