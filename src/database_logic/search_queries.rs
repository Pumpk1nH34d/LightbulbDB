use crate::database_logic::data_structs::{Participant, SupportWorker, Venue, Workshop};
use crate::database_logic::database::DataBase;
use chrono::NaiveDate;

// Implementing methods for the DataBase struct
impl DataBase {
    // Method to get all participants, with sorting option
    pub fn get_all_participants(&self, sort: String) -> Vec<Participant> {
        // Prepare the SQL query to select all participants with sorting
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM Participants {}", sort))
            .unwrap();

        // Execute the query and map the results to Participant structs
        stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),  // Get the id
                first_name: row.get_unwrap(1),  // Get the first name
                last_name: row.get_unwrap(2),  // Get the last name
                medicare_number: row.get_unwrap(3),  // Get the medicare number
                dob: match row.get::<_, String>(4) {  // Get the date of birth
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
                address: row.get(5).unwrap_or(Some(String::new())),  // Get the address
                suburb: row.get(6).unwrap_or(Some(String::new())),  // Get the suburb
                postcode: row.get(7).unwrap_or(Some(String::new())),  // Get the postcode
                phone: row.get(8).unwrap_or(Some(String::new())),  // Get the phone number
                email: row.get(9).unwrap_or(Some(String::new())),  // Get the email
                medical_notes: row.get(10).unwrap_or(Some(String::new())),  // Get medical notes
                dietary_notes: row.get(11).unwrap_or(Some(String::new())),  // Get dietary notes
                physical_notes: row.get(12).unwrap_or(Some(String::new())),  // Get physical notes
                other_notes: row.get(13).unwrap_or(Some(String::new())),  // Get other notes
                support_ratio: row.get(14).unwrap_or(Some(String::new())),  // Get support ratio
                photo_permission: match row.get(15).unwrap() {  // Get photo permission
                    1 => Some(true),
                    _ => Some(false),
                },
                private_hospital_preference: row.get(16).unwrap_or(Some(false)),  // Get private hospital preference
                private_health_insurer: row.get(17).unwrap_or(Some(String::new())),  // Get private health insurer
                private_health_number: row.get(18).unwrap_or(Some(String::new())),  // Get private health number
                communication_preference: row.get(19).unwrap_or(Some(String::new())),  // Get communication preference
                ndis_plan_number: row.get(20).unwrap_or(Some(String::new())),  // Get NDIS plan number
                ndis_plan_start_date: match row.get::<_, String>(21) {  // Get NDIS plan start date
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
                core_funding: row.get(22).unwrap_or(Some(false)),  // Get core funding
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),  // Get capacity building funding
                self_managed: row.get(24).unwrap_or(Some(false)),  // Get self-managed status
                plan_managed: row.get(25).unwrap_or(Some(false)),  // Get plan-managed status
                ndis_plan_end_date: match row.get::<_, String>(26) {  // Get NDIS plan end date
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of Participant structs
    }

    // Method to get participants based on a filter and sorting
    pub fn get_filtered_participants(&self, filter: String, sort: String) -> Vec<Participant> {
        // Prepare the SQL query with a filter and sorting
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Participants WHERE {} {}", filter, sort)).unwrap();

        // Execute the query and map the results to Participant structs
        stmt.query_map([], |row| {
            Ok(Participant {
                id: row.get_unwrap(0),  // Get the id
                first_name: row.get_unwrap(1),  // Get the first name
                last_name: row.get_unwrap(2),  // Get the last name
                medicare_number: row.get_unwrap(3),  // Get the medicare number
                dob: match row.get::<_, String>(4) {  // Get the date of birth
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
                address: row.get(5).unwrap_or(Some(String::new())),  // Get the address
                suburb: row.get(6).unwrap_or(Some(String::new())),  // Get the suburb
                postcode: row.get(7).unwrap_or(Some(String::new())),  // Get the postcode
                phone: row.get(8).unwrap_or(Some(String::new())),  // Get the phone number
                email: row.get(9).unwrap_or(Some(String::new())),  // Get the email
                medical_notes: row.get(10).unwrap_or(Some(String::new())),  // Get medical notes
                dietary_notes: row.get(11).unwrap_or(Some(String::new())),  // Get dietary notes
                physical_notes: row.get(12).unwrap_or(Some(String::new())),  // Get physical notes
                other_notes: row.get(13).unwrap_or(Some(String::new())),  // Get other notes
                support_ratio: row.get(14).unwrap_or(Some(String::new())),  // Get support ratio
                photo_permission: match row.get(15).unwrap() {  // Get photo permission
                    1 => Some(true),
                    _ => Some(false),
                },
                private_hospital_preference: row.get(16).unwrap_or(Some(false)),  // Get private hospital preference
                private_health_insurer: row.get(17).unwrap_or(Some(String::new())),  // Get private health insurer
                private_health_number: row.get(18).unwrap_or(Some(String::new())),  // Get private health number
                communication_preference: row.get(19).unwrap_or(Some(String::new())),  // Get communication preference
                ndis_plan_number: row.get(20).unwrap_or(Some(String::new())),  // Get NDIS plan number
                ndis_plan_start_date: match row.get::<_, String>(21) {  // Get NDIS plan start date
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
                core_funding: row.get(22).unwrap_or(Some(false)),  // Get core funding
                capacity_building_funding: row.get(23).unwrap_or(Some(false)),  // Get capacity building funding
                self_managed: row.get(24).unwrap_or(Some(false)),  // Get self-managed status
                plan_managed: row.get(25).unwrap_or(Some(false)),  // Get plan-managed status
                ndis_plan_end_date: match row.get::<_, String>(26) {  // Get NDIS plan end date
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of Participant structs
    }

    // Method to get all support workers, with sorting option
    pub fn get_all_support_workers(&self, sort: String) -> Vec<SupportWorker> {
        // Prepare the SQL query to select all support workers with sorting
        let mut stmt = self
            .connection
            .prepare(&format!("SELECT * FROM Support_Workers {}", sort))
            .unwrap();

        // Execute the query and map the results to SupportWorker structs
        stmt.query_map([], |row| {
            Ok(SupportWorker {
                id: row.get_unwrap(0),  // Get the id
                first_name: row.get_unwrap(1),  // Get the first name
                last_name: row.get_unwrap(2),  // Get the last name
                phone: row.get(3).unwrap(),  // Get the phone number
                email: row.get(4).unwrap(),  // Get the email
                dob: match row.get::<_, String>(5) {  // Get the date of birth
                    Ok(value) => {Some(value.parse::<NaiveDate>().unwrap())},
                    Err(_) => {None}
                },
                address: row.get(6).unwrap_or(Some(String::new())),  // Get the address
                suburb: row.get(7).unwrap_or(Some(String::new())),  // Get the suburb
                postcode: row.get(8).unwrap_or(Some(String::new())),  // Get the postcode
                first_aid: row.get(9).unwrap_or(Some(false)),  // Get first aid status
                confidentiality_agreement: row.get(10).unwrap_or(Some(false)),  // Get confidentiality agreement status
                police_clearance: row.get(11).unwrap_or(Some(false)),  // Get police clearance status
                car_insurance: row.get(12).unwrap_or(Some(false)),  // Get car insurance status
                other_qualifications: row.get(13).unwrap_or(Some(String::new())),  // Get other qualifications
                notes: row.get(14).unwrap_or(Some(String::new())),  // Get notes
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of SupportWorker structs
    }

    // Method to get support workers based on a filter and sorting
    pub fn get_filtered_support_workers(&self, filter: String, sort: String) -> Vec<SupportWorker> {
        // Prepare the SQL query with a filter and sorting
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Support_Workers WHERE {} {}", filter, sort)).unwrap();

        // Execute the query and map the results to SupportWorker structs
        stmt.query_map([], |row| {
            Ok(SupportWorker {
                id: row.get_unwrap(0),  // Get the id
                first_name: row.get_unwrap(1),  // Get the first name
                last_name: row.get_unwrap(2),  // Get the last name
                phone: row.get(3).unwrap(),  // Get the phone number
                email: row.get(4).unwrap(),  // Get the email
                dob: Some(
                    row.get_unwrap::<_, String>(4)
                        .parse::<NaiveDate>()
                        .unwrap_or_default(),
                ),
                address: row.get(5).unwrap_or(Some(String::new())),  // Get the address
                suburb: row.get(6).unwrap_or(Some(String::new())),  // Get the suburb
                postcode: row.get(7).unwrap_or(Some(String::new())),  // Get the postcode
                first_aid: row.get(8).unwrap_or(Some(false)),  // Get first aid status
                confidentiality_agreement: row.get(9).unwrap_or(Some(false)),  // Get confidentiality agreement status
                police_clearance: row.get(10).unwrap_or(Some(false)),  // Get police clearance status
                car_insurance: row.get(11).unwrap_or(Some(false)),  // Get car insurance status
                other_qualifications: row.get(12).unwrap_or(Some(String::new())),  // Get other qualifications
                notes: row.get(13).unwrap_or(Some(String::new())),  // Get notes
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of SupportWorker structs
    }

    // Method to get all venues, with sorting option
    pub fn get_all_venues(&self, sort: String) -> Vec<Venue> {
        // Prepare the SQL query to select all venues with sorting
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Venues {}", sort)).unwrap();

        // Execute the query and map the results to Venue structs
        stmt.query_map([], |row| {
            Ok(Venue {
                id: row.get_unwrap(0),  // Get the id
                name: row.get_unwrap(1),  // Get the name
                address: row.get(2).unwrap_or(Some(String::new())),  // Get the address
                suburb: row.get(3).unwrap_or(Some(String::new())),  // Get the suburb
                postcode: row.get(4).unwrap_or(Some(String::new())),  // Get the postcode
                state: row.get(5).unwrap_or(Some(String::new())),  // Get the state
                description: row.get(6).unwrap_or(Some(String::new())),  // Get the description
                contact_person_name: row.get(7).unwrap_or(Some(String::new())),  // Get the contact person name
                contact_person_phone: row.get(8).unwrap_or(Some(String::new())),  // Get the contact person phone number
                venue_phone_number: row.get(9).unwrap_or(Some(String::new())),  // Get the venue phone number
                price: row.get(10).unwrap_or(Some(String::new())),  // Get the price
                notes: row.get(11).unwrap_or(Some(String::new())),  // Get notes
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of Venue structs
    }

    // Method to get venues based on a filter and sorting
    pub fn get_filtered_venues(&self, filter: String, sort: String) -> Vec<Venue> {
        // Prepare the SQL query with a filter and sorting
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Venues WHERE {} {}", filter, sort)).unwrap();

        // Execute the query and map the results to Venue structs
        stmt.query_map([], |row| {
            Ok(Venue {
                id: row.get_unwrap(0),  // Get the id
                name: row.get_unwrap(1),  // Get the name
                address: row.get(2).unwrap_or(Some(String::new())),  // Get the address
                suburb: row.get(3).unwrap_or(Some(String::new())),  // Get the suburb
                postcode: row.get(4).unwrap_or(Some(String::new())),  // Get the postcode
                state: row.get(5).unwrap_or(Some(String::new())),  // Get the state
                description: row.get(6).unwrap_or(Some(String::new())),  // Get the description
                contact_person_name: row.get(7).unwrap_or(Some(String::new())),  // Get the contact person name
                contact_person_phone: row.get(8).unwrap_or(Some(String::new())),  // Get the contact person phone number
                venue_phone_number: row.get(9).unwrap_or(Some(String::new())),  // Get the venue phone number
                price: row.get(10).unwrap_or(Some(String::new())),  // Get the price
                notes: row.get(11).unwrap_or(Some(String::new())),  // Get notes
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of Venue structs
    }

    // Method to get all workshops, with sorting option
    pub fn get_all_workshops(&self, sort: String) -> Vec<Workshop> {
        // Prepare the SQL query to select all workshops with sorting
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshops {}", sort)).unwrap();

        // Execute the query and map the results to Workshop structs
        stmt.query_map([], |row| {
            Ok(Workshop {
                id: row.get_unwrap(0),  // Get the id
                name: row.get_unwrap(1),  // Get the name
                facilitator: row.get(2).unwrap(),  // Get the facilitator
                venue: row.get(3).unwrap(),  // Get the venue
                start_date: row.get_unwrap::<_, String>(4)
                    .parse::<NaiveDate>()
                    .unwrap_or_default(),  // Get the start date
                end_date: row.get_unwrap::<_, String>(5)
                    .parse::<NaiveDate>()
                    .unwrap_or_default(),  // Get the end date
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of Workshop structs
    }

    // Method to get workshops based on a filter and sorting
    pub fn get_filtered_workshops(&self, filter: String, sort: String) -> Vec<Workshop> {
        // Prepare the SQL query with a filter and sorting
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshops WHERE {} {}", filter, sort)).unwrap();

        // Execute the query and map the results to Workshop structs
        stmt.query_map([], |row| {
            Ok(Workshop {
                id: row.get_unwrap(0),  // Get the id
                name: row.get_unwrap(1),  // Get the name
                facilitator: row.get(2).unwrap(),  // Get the facilitator
                venue: row.get(3).unwrap(),  // Get the venue
                start_date: row.get_unwrap::<_, String>(4).parse().unwrap(),  // Get the start date
                end_date: row.get_unwrap::<_, String>(5).parse().unwrap(),  // Get the end date
            })
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()  // Collect the results into a vector of Workshop structs
    }

    // Method to get participant IDs from a workshop
    pub fn get_participants_from_workshop(&self, workshop: i32) -> Vec<i32> {
        // Prepare the SQL query to select participant IDs from a specific workshop
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshop__Participants WHERE workshop = '{}'", workshop)).unwrap();

        // Execute the query and collect participant IDs into a vector
        stmt.query_map([], |row| {
            Ok(row.get_unwrap(1))  // Get the participant ID
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }

    // Method to get support worker IDs from a workshop
    pub fn get_support_workers_from_workshop(&self, workshop: i32) -> Vec<i32> {
        // Prepare the SQL query to select support worker IDs from a specific workshop
        let mut stmt = self.connection.prepare(&format!("SELECT * FROM Workshop__Support_Worker WHERE workshop = '{}'", workshop)).unwrap();

        // Execute the query and collect support worker IDs into a vector
        stmt.query_map([], |row| {
            Ok(row.get_unwrap(1))  // Get the support worker ID
        })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
    }
}
