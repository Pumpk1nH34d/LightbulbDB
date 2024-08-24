use crate::database_logic::data_structs::{Participant};
use rusqlite::{Result, params};
use crate::database_logic::database::DataBase;

impl DataBase {
    pub fn edit_participant(&self, p: Participant) -> Result<()> {
        self.connection.execute(
            "UPDATE Participants SET first_name = ?1, last_name = ?2, medicare_number = ?3, dob = ?4, address = ?5, suburb = ?6, postcode = ?7, phone = ?8, email = ?9, medical_notes = ?10, dietary_notes = ?11, physical_notes = ?12, other_notes = ?13, support_ratio = ?14, photo_permission = ?15, private_hospital_preference = ?16, private_health_insurancer = ?17, private_health_number = ?18, communication_preference = ?19, ndis_plan_number = ?20, ndis_plan_start_date = ?21, core_funding = ?22, capacity_building_funding = ?23, self_managed = ?24, plan_managed = ?25, ndis_plan_end_date = ?26 WHERE id = ?27",
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
                p.id,
            ],
        )?;
        Ok(())
    }
}


