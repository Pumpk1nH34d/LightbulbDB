use crate::database_logic::data_structs::{Participant, SupportWorker};
use rusqlite::{Result, params};
use crate::database_logic::database::DataBase;

impl DataBase {
    pub fn create_support_worker(&self, sw: SupportWorker) -> Result<()> {
        self.db.execute(
            "INSERT INTO Support_Workers (first_name, last_name, phone, email, dob, address, suburb, postcode, first_aid, first_aid_file, confidentiality_agreement, police_clearance, car_insurance, other_qualifications, notes, phone) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![sw.first_name, sw.last_name, sw.phone, sw.dob.as_ref().unwrap().to_string(), sw.address, sw.suburb, sw.postcode, sw.first_aid, sw.first_aid_file, sw.confidentiality_agreement, sw.police_clearance, sw.car_insurance, sw.other_qualifications, sw.notes],
        )?; // need to add email
        Ok(())
    }

    pub fn create_participant(&self, p: Participant) -> Result<()> {
        self.db.execute(
            "INSERT INTO Participants (first_name, last_name, medicare_number, dob, address, suburb, postcode, phone, email, medical_notes, dietary_notes, physical_notes, other_notes, support_ratio, photo_permission, private_hospital_preference, private_health_insurancer, private_health_number, communication_preference, ndis_plan_number, ndis_plan_start_date, core_funding, capacity_building_funding, self_managed, plan_managed, ndis_plan_end_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26)",
            params![
                p.first_name,
                p.last_name,
                p.medicare_number,
                p.dob.map(|value| value.to_string()),
                p.address.map(|value| value.to_string()),
                p.suburb.map(|value| value.to_string()),
                p.postcode.map(|value| value.to_string()),
                p.phone.map(|value| value.to_string()),
                p.email.map(|value| value.to_string()),
                p.medical_notes.map(|value| value.to_string()),
                p.dietary_notes.map(|value| value.to_string()),
                p.physical_notes.map(|value| value.to_string()),
                p.other_notes.map(|value| value.to_string()),
                p.support_ratio.map(|value| value.to_string()),
                p.photo_permission.map(|value| value),
                p.private_hospital_preference.map(|value| value.to_string()),
                p.private_health_insurancer.map(|value| value.to_string()),
                p.private_health_number.map(|value| value.to_string()),
                p.communication_preference.map(|value| value.to_string()),
                p.ndis_plan_number.map(|value| value.to_string()),
                p.ndis_plan_start_date.map(|value| value.to_string()),
                p.core_funding.map(|value| value.to_string()),
                p.capacity_building_funding.map(|value| value.to_string()),
                p.self_managed.map(|value| value.to_string()),
                p.plan_managed.map(|value| value.to_string()),
                p.ndis_plan_end_date.map(|value| value.to_string()),
            ],
        )?;
        Ok(())
    }
}


