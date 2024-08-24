use crate::database_logic::data_structs::{Participant, SupportWorker, Venue};
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;

impl DataBase {
    pub fn get_all_participants(&self) -> Vec<Participant> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM Participants")
            .unwrap();
        stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                medicare_number: row.get_unwrap(3),
                dob: Some(
                    row.get_unwrap::<_, String>(4)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                address: row.get(5).unwrap_or(Some(String::new())),
                suburb: row.get(6).unwrap_or(Some(String::new())),
                postcode: row.get(7).unwrap_or(Some(String::new())),
                phone: row.get(8).unwrap_or(Some(String::new())),
                email: row.get(9).unwrap_or(Some(String::new())),
                medical_notes: row.get(10).unwrap_or(Some(String::new())),
                dietary_notes: row.get(11).unwrap_or(Some(String::new())),
                physical_notes: row.get(12).unwrap_or(Some(String::new())),
                other_notes: row.get(13).unwrap_or(Some(String::new())),
                support_ratio: row.get(14).unwrap_or(Some(String::new())),
                photo_permission: match row.get(15).unwrap() {
                    1 => Some(true),
                    _ => Some(false),
                },
                private_hospital_preference: row.get(16).unwrap_or(Some(false)),
                private_health_insurer: row.get(17).unwrap_or(Some(String::new())),
                private_health_number: row.get(18).unwrap_or(Some(String::new())),
                communication_preference: row.get(19).unwrap_or(Some(String::new())),
                ndis_plan_number: row.get(20).unwrap_or(Some(String::new())),
                ndis_plan_start_date: Some(
                    row.get_unwrap::<_, String>(21)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                core_funding: row.get(22).unwrap_or(Some(false)),
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),
                self_managed: row.get(24).unwrap_or(Some(false)),
                plan_managed: row.get(25).unwrap_or(Some(false)),
                ndis_plan_end_date: Some(
                    row.get_unwrap::<_, String>(26)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
            })
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }

    pub fn get_filtered_participants(&self, filter: String) -> Vec<Participant> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Participants WHERE {}", filter)).unwrap();
        stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                medicare_number: row.get_unwrap(3),
                dob: Some(
                    row.get_unwrap::<_, String>(4)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                address: row.get(5).unwrap_or(Some(String::new())),
                suburb: row.get(6).unwrap_or(Some(String::new())),
                postcode: row.get(7).unwrap_or(Some(String::new())),
                phone: row.get(8).unwrap_or(Some(String::new())),
                email: row.get(9).unwrap_or(Some(String::new())),
                medical_notes: row.get(10).unwrap_or(Some(String::new())),
                dietary_notes: row.get(11).unwrap_or(Some(String::new())),
                physical_notes: row.get(12).unwrap_or(Some(String::new())),
                other_notes: row.get(13).unwrap_or(Some(String::new())),
                support_ratio: row.get(14).unwrap_or(Some(String::new())),
                photo_permission: match row.get(15).unwrap() {
                    1 => Some(true),
                    _ => Some(false),
                },
                private_hospital_preference: row.get(16).unwrap_or(Some(false)),
                private_health_insurer: row.get(17).unwrap_or(Some(String::new())),
                private_health_number: row.get(18).unwrap_or(Some(String::new())),
                communication_preference: row.get(19).unwrap_or(Some(String::new())),
                ndis_plan_number: row.get(20).unwrap_or(Some(String::new())),
                ndis_plan_start_date: Some(
                    row.get_unwrap::<_, String>(21)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                core_funding: row.get(22).unwrap_or(Some(false)),
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),
                self_managed: row.get(24).unwrap_or(Some(false)),
                plan_managed: row.get(25).unwrap_or(Some(false)),
                ndis_plan_end_date: Some(
                    row.get_unwrap::<_, String>(26)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
    
    pub fn get_all_support_workers(&self) -> Vec<SupportWorker> {
        let mut stmt = self
            .connection
            .prepare("SELECT * FROM Support_Workers")
            .unwrap();
        stmt.query_map([], |row| {
            Ok(SupportWorker {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                phone: row.get(3).unwrap(),
                email: row.get(4).unwrap(),
                dob: Some(
                    row.get_unwrap::<_, String>(4)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                address: row.get(5).unwrap_or(Some(String::new())),
                suburb: row.get(6).unwrap_or(Some(String::new())),
                postcode: row.get(7).unwrap_or(Some(String::new())),
                first_aid: row.get(8).unwrap_or(Some(false)),
                confidentiality_agreement: row.get(9).unwrap_or(Some(false)),
                police_clearance: row.get(10).unwrap_or(Some(false)),
                car_insurance: row.get(11).unwrap_or(Some(false)),
                other_qualifications: row.get(12).unwrap_or(Some(String::new())),
                notes: row.get(13).unwrap_or(Some(String::new())),
            })
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }

    pub fn get_filtered_support_workers(&self, filter: String) -> Vec<SupportWorker> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Support_Workers WHERE {}", filter)).unwrap();
        stmt.query_map([], |row| {
            Ok(SupportWorker {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                phone: row.get(3).unwrap(),
                email: row.get(4).unwrap(),
                dob: Some(
                    row.get_unwrap::<_, String>(4)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                address: row.get(5).unwrap_or(Some(String::new())),
                suburb: row.get(6).unwrap_or(Some(String::new())),
                postcode: row.get(7).unwrap_or(Some(String::new())),
                first_aid: row.get(8).unwrap_or(Some(false)),
                confidentiality_agreement: row.get(9).unwrap_or(Some(false)),
                police_clearance: row.get(10).unwrap_or(Some(false)),
                car_insurance: row.get(11).unwrap_or(Some(false)),
                other_qualifications: row.get(12).unwrap_or(Some(String::new())),
                notes: row.get(13).unwrap_or(Some(String::new())),
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    pub fn get_all_venues(&self) -> Vec<Venue> {
        let mut stmt = self.connection.prepare("SELECT * FROM Venues").unwrap();
        stmt.query_map([], |row| {
            Ok(Venue {
                id: row.get_unwrap(0),
                name: row.get_unwrap(1),
                address: row.get(2).unwrap_or(Some(String::new())),
                suburb: row.get(3).unwrap_or(Some(String::new())),
                postcode: row.get(4).unwrap_or(Some(String::new())),
                state: row.get(5).unwrap_or(Some(String::new())),
                description: row.get(6).unwrap_or(Some(String::new())),
                contact_person_name: row.get(7).unwrap_or(Some(String::new())),
                contact_person_phone: row.get(8).unwrap_or(Some(String::new())),
                venue_phone_number: row.get(9).unwrap_or(Some(String::new())),
                price: row.get(10).unwrap_or(Some(String::new())),
                notes: row.get(11).unwrap_or(Some(String::new())),
            })
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }

    pub fn get_filtered_venues(&self, filter: String) -> Vec<Venue> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Venues WHERE {}", filter)).unwrap();
        stmt.query_map([], |row| {
            Ok(Venue {
                id: row.get_unwrap(0),
                name: row.get_unwrap(1),
                address: row.get(2).unwrap_or(Some(String::new())),
                suburb: row.get(3).unwrap_or(Some(String::new())),
                postcode: row.get(4).unwrap_or(Some(String::new())),
                state: row.get(5).unwrap_or(Some(String::new())),
                description: row.get(6).unwrap_or(Some(String::new())),
                contact_person_name: row.get(7).unwrap_or(Some(String::new())),
                contact_person_phone: row.get(8).unwrap_or(Some(String::new())),
                venue_phone_number: row.get(9).unwrap_or(Some(String::new())),
                price: row.get(10).unwrap_or(Some(String::new())),
                notes: row.get(11).unwrap_or(Some(String::new())),
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}
