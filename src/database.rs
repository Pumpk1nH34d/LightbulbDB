use crate::data_structs::*;
use rusqlite::{Connection, Result};

pub struct DataBase {
    db: Connection,
}

impl DataBase {
    fn new_connection(db_file_name: String) -> DataBase {
        DataBase {
            db: Connection::open(db_file_name).unwrap(),
        }
    }
    pub fn create_db(&self) -> Result<()> {
        self.db.execute(
            "
    CREATE TABLE LineItems (
	id	TEXT NOT NULL UNIQUE,
	name	TEXT NOT NULL,
	description	TEXT NOT NULL,
	support_category	TEXT,
	PRIMARY KEY(id))",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Medical_Contacts (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT NOT NULL,
	last_name	TEXT NOT NULL,
	phone	INTEGER NOT NULL,
	relationship	TEXT,
	PRIMARY KEY(id AUTOINCREMENT))",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Parents (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT NOT NULL,
	last_name	TEXT NOT NULL,
	relationship	TEXT NOT NULL,
	phone_number	TEXT NOT NULL,
	communication_preference	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participant__Medical_Contact (
	participant	INTEGER NOT NULL,
	medical_contact	INTEGER NOT NULL,
	FOREIGN KEY(medical_contact) REFERENCES Medical_Contacts(id),
	FOREIGN KEY(participant) REFERENCES Participants(id)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT NOT NULL,
	last_name	TEXT NOT NULL,
	medicare_number	TEXT NOT NULL,
	dob	INTEGER,
	address	TEXT,
	suburb	TEXT,
	postcode	TEXT,
	phone	TEXT,
	email	TEXT,
	medical_notes	TEXT,
	dietary_notes	TEXT,
	physical_notes	TEXT,
	other_notes	TEXT,
	support_ratio	TEXT,
	photo_permission	INTEGER,
	private_hospital_preference	INTEGER,
	private_health_insurancer	TEXT,
	private_health_number	TEXT,
	communication_preference	TEXT,
	ndis_plan_number	TEXT,
	ndis_plan_start_date	TEXT,
	core_funding	INTEGER,
	capacity_building_funding	INTEGER,
	self_managed	INTEGER,
	plan_managed	INTEGER,
	ndis_plan_end_date	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants__LineItems (
	participants	INTEGER NOT NULL,
	lineitem	INTEGER NOT NULL,
	FOREIGN KEY(participants) REFERENCES Participants(id),
	FOREIGN KEY(lineitem) REFERENCES LineItems(id)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants__Parents (
	participant	INTEGER NOT NULL,
	parent	INTEGER NOT NULL,
	FOREIGN KEY(participant) REFERENCES Participants(id),
	FOREIGN KEY(parent) REFERENCES Parents(id)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants__Plan_Managers (
	participant	INTEGER NOT NULL,
	plan_manager	INTEGER NOT NULL,
	FOREIGN KEY(participant) REFERENCES Participants(id),
	FOREIGN KEY(plan_manager) REFERENCES Plan_Managers(id)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants__Support_Coordinators (
	participants	INTEGER NOT NULL,
	support_coordinators	INTEGER NOT NULL,
	FOREIGN KEY(participants) REFERENCES Participants(id),
	FOREIGN KEY(support_coordinators) REFERENCES Support_Coordinators
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants_Dislike (
	subject	INTEGER NOT NULL,
	dislikes	INTEGER NOT NULL,
	FOREIGN KEY(subject) REFERENCES Participants(id),
	FOREIGN KEY(dislikes) REFERENCES Participants(id)
)
",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Participants_Likes (
	subject	INTEGER NOT NULL,
	likes	INTEGER NOT NULL,
	FOREIGN KEY(subject) REFERENCES Participants(id),
	FOREIGN KEY(likes) REFERENCES Participants(id)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Plan_Managers (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT,
	last_name	INTEGER,
	company_name	TEXT,
	phone	TEXT,
	company_address	TEXT,
	email	TEXT,
	email_invoice	TEXT,
	company_phone	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Support_Coordinators (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT,
	last_name	TEXT,
	company_name	TEXT,
	company_phone	TEXT,
	phone	TEXT,
	email	TEXT,
	company_email	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Support_Workers (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT NOT NULL,
	last_name	TEXT NOT NULL,
	dob	TEXT,
	address	TEXT,
	suburb	TEXT,
	postcode	TEXT,
	first_aid	INTEGER,
	first_aid_file	TEXT,
	confidentiality_agreement	TEXT,
	police_clearance	TEXT,
	car_insurance	INTEGER,
	other_qualifications	TEXT,
	notes	TEXT,
	phone	TEXT NOT NULL,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Venues (
	id	INTEGER NOT NULL UNIQUE,
	name	TEXT NOT NULL,
	address	TEXT,
	suburb	TEXT,
	postcode	TEXT,
	state	TEXT NOT NULL,
	description	TEXT,
	contact_person_name	TEXT,
	contact_person_phone	TEXT,
	venue_phone_number	TEXT,
	price	TEXT,
	notes	BLOB,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Workshop__Support_Worker (
	workshop	INTEGER NOT NULL,
	support_worker	INTEGER NOT NULL,
	FOREIGN KEY(support_worker) REFERENCES Support_Workers(id),
	FOREIGN KEY(workshop) REFERENCES Workshops(id)
)",
            (),
        )?;

        self.db.execute(
            "
    CREATE TABLE Workshops (
	id	INTEGER NOT NULL UNIQUE,
	name	TEXT NOT NULL,
	facilitator	INTEGER NOT NULL,
	venue	INTEGER NOT NULL,
	start_date	TEXT NOT NULL,
	end_date	TEXT NOT NULL,
	FOREIGN KEY(facilitator) REFERENCES Support_Workers(id),
	FOREIGN KEY(venue) REFERENCES Venues(id),
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;
        Ok(())
    }
    pub fn new_support_worker(&self, sw: &SupportWorker) -> Result<()> {
        self.db.execute(
            "INSERT INTO Support_Workers (first_name, last_name, dob, address, suburb, postcode, first_aid, first_aid_file, confidentiality_agreement, police_clearance, car_insurance, other_qualifications, notes, phone) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![sw.first_name, sw.last_name, sw.dob, sw.address, sw.suburb, sw.postcode, sw.first_aid, sw.first_aid_file, sw.confidentiality_agreement, sw.police_clearance, sw.car_insurance, sw.other_qualifications, sw.notes, sw.phone],
        )?;
        Ok(())
    }
    pub fn drop_db(&self) -> Result<()> {
        let tables = [
            "LineItems",
            "Medical_Contacts",
            "Parents",
            "Participant__Medical_Contact",
            "Participants",
            "Participants__LineItems",
            "Participants__Parents",
            "Participants__Plan_Managers",
            "Participants__Support_Coordinators",
            "Participants_Dislike",
            "Participants_Likes",
            "Plan_Managers",
            "Support_Coordinators",
            "Support_Workers",
            "Venues",
            "Workshop__Support_Worker",
            "Workshops",
        ];
        for table in tables.iter() {
            self.db
                .execute(&format!("DROP TABLE IF EXISTS {}", table), [])?;
        }
        Ok(())
    }
    pub fn get_name(&self) -> Result<String> {
        let mut stmt = self.db.prepare("SELECT first_name, last_name, dob, address, suburb, postcode, first_aid, first_aid_file, confidentiality_agreement, police_clearance, car_insurance, other_qualifications, notes, phone FROM Support_Workers")?;
        let person_iter = stmt.query_map([], |row| {
            Ok(SupportWorker {
                first_name: row.get(0)?,
                last_name: row.get(1)?,
                dob: row.get(2)?,
                address: row.get(3)?,
                suburb: row.get(4)?,
                postcode: row.get(5)?,
                first_aid: row.get(6)?,
                first_aid_file: row.get(7)?,
                confidentiality_agreement: row.get(8)?,
                police_clearance: row.get(9)?,
                car_insurance: row.get(10)?,
                other_qualifications: row.get(11)?,
                notes: row.get(12)?,
                phone: row.get(13)?,
            })
        })?;
        Ok(format!(
            "{:?}",
            person_iter.into_iter().collect::<Vec<_>>().len()
        ))
    }
}

impl Default for DataBase {
    fn default() -> Self {
        DataBase::new_connection(String::from("LightBulb.db"))
    }
}
