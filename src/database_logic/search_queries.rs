use crate::database_logic::data_structs::{Participant, SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;

//todo: comment code

impl DataBase {
    pub fn get_all_participants(&self, sort: String) -> Vec<Participant> {
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM Participants {}", sort))
            .unwrap();
        stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                medicare_number: row.get_unwrap(3),
                dob: match row.get::<_, String>(4) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
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
                ndis_plan_start_date: match row.get::<_, String>(21) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
                core_funding: row.get(22).unwrap_or(Some(false)),
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),
                self_managed: row.get(24).unwrap_or(Some(false)),
                plan_managed: row.get(25).unwrap_or(Some(false)),
                ndis_plan_end_date: match row.get::<_, String>(26) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
            })
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }

    pub fn get_filtered_participants(&self, filter: String, sort: String) -> Vec<Participant> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Participants WHERE {} {}", filter, sort)).unwrap();
        stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                medicare_number: row.get_unwrap(3),
                dob: match row.get::<_, String>(4) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
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
                ndis_plan_start_date: match row.get::<_, String>(21) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
                core_funding: row.get(22).unwrap_or(Some(false)),
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),
                self_managed: row.get(24).unwrap_or(Some(false)),
                plan_managed: row.get(25).unwrap_or(Some(false)),
                ndis_plan_end_date: match row.get::<_, String>(26) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
    
    pub fn get_all_support_workers(&self, sort: String) -> Vec<SupportWorker> {
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM Support_Workers {}", sort))
            .unwrap();
        stmt.query_map([], |row| {
            Ok(SupportWorker {
                id: row.get_unwrap(0),
                first_name: row.get_unwrap(1),
                last_name: row.get_unwrap(2),
                phone: row.get(3).unwrap(),
                email: row.get(4).unwrap(),
                dob: match row.get::<_, String>(5) {
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())}
                    Err(_) => {None}
                },
                address: row.get(6).unwrap_or(Some(String::new())),
                suburb: row.get(7).unwrap_or(Some(String::new())),
                postcode: row.get(8).unwrap_or(Some(String::new())),
                first_aid: row.get(9).unwrap_or(Some(false)),
                confidentiality_agreement: row.get(10).unwrap_or(Some(false)),
                police_clearance: row.get(11).unwrap_or(Some(false)),
                car_insurance: row.get(12).unwrap_or(Some(false)),
                other_qualifications: row.get(13).unwrap_or(Some(String::new())),
                notes: row.get(14).unwrap_or(Some(String::new())),
            })
        })
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
    }

    pub fn get_filtered_support_workers(&self, filter: String, sort: String) -> Vec<SupportWorker> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Support_Workers WHERE {} {}", filter, sort)).unwrap();
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

    pub fn get_all_venues(&self, sort: String) -> Vec<Venue> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Venues {}", sort)).unwrap();
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

    pub fn get_filtered_venues(&self, filter: String, sort: String) -> Vec<Venue> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Venues WHERE {} {}", filter, sort)).unwrap();
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

    pub fn get_all_workshops(&self, sort: String) -> Vec<Workshop> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshops {}", sort)).unwrap();
        stmt.query_map([], |row| {
            Ok(Workshop {
                id: row.get_unwrap(0),
                name: row.get_unwrap(1),
                facilitator: row.get(2).unwrap(),
                venue: row.get(3).unwrap(),
                start_date: row.get_unwrap::<_, String>(4)
                    .parse::<NaiveDate>()
                    .unwrap_or_default(),
                end_date: row.get_unwrap::<_, String>(5)
                    .parse::<NaiveDate>()
                    .unwrap_or_default(),
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    pub fn get_filtered_workshops(&self, filter: String, sort: String) -> Vec<Workshop> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshops WHERE {} {}", filter, sort)).expect(&format!("SELECT * FROM Workshops WHERE {} {}", filter, sort));
        stmt.query_map([], |row| {
            Ok(Workshop {
                id: row.get_unwrap(0),
                name: row.get_unwrap(1),
                facilitator: row.get(2).unwrap(),
                venue: row.get(3).unwrap(),
                start_date: row.get_unwrap::<_, String>(4).parse().unwrap(),
                end_date: row.get_unwrap::<_, String>(5).parse().unwrap(),
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    pub fn get_participants_from_workshop(&self, workshop: i32) -> Vec<i32> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshop__Participants WHERE workshop = '{}'", workshop)).unwrap();
        stmt.query_map([], |row| {
            Ok(row.get_unwrap(1))
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    pub fn get_support_workers_from_workshop(&self, workshop: i32) -> Vec<i32> {
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshop__Support_Worker WHERE workshop = '{}'", workshop)).unwrap();
        stmt.query_map([], |row| {
            Ok(row.get_unwrap(1))
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}
