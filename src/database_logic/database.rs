use rusqlite::{Connection, Result};

pub struct DataBase {
    pub connection: Connection,
}

impl DataBase {
    fn new_connection(db_file_name: String) -> DataBase {
        DataBase {
            connection: Connection::open(db_file_name).unwrap(),
        }
    }
    pub fn create_db(&self) -> Result<()> {
        /*
                self.connection.execute(
                    "
            CREATE TABLE LineItems (
            id	TEXT NOT NULL UNIQUE,
            name	TEXT NOT NULL,
            description	TEXT NOT NULL,
            support_category	TEXT,
            PRIMARY KEY(id))",
                    (),
                )?;

                self.connection.execute(
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

                self.connection.execute(
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

                self.connection.execute(
                    "
            CREATE TABLE Participant__Medical_Contact (
            participant	INTEGER NOT NULL,
            medical_contact	INTEGER NOT NULL,
            FOREIGN KEY(medical_contact) REFERENCES Medical_Contacts(id),
            FOREIGN KEY(participant) REFERENCES Participants(id)
        )",
                    (),
                )?;

        self.connection.execute(
            "
    CREATE TABLE Participants__LineItems (
	participants	INTEGER NOT NULL,
	lineitem	INTEGER NOT NULL,
	FOREIGN KEY(participants) REFERENCES Participants(id),
	FOREIGN KEY(lineitem) REFERENCES LineItems(id)
)",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Participants__Parents (
	participant	INTEGER NOT NULL,
	parent	INTEGER NOT NULL,
	FOREIGN KEY(participant) REFERENCES Participants(id),
	FOREIGN KEY(parent) REFERENCES Parents(id)
)",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Participants__Plan_Managers (
	participant	INTEGER NOT NULL,
	plan_manager	INTEGER NOT NULL,
	FOREIGN KEY(participant) REFERENCES Participants(id),
	FOREIGN KEY(plan_manager) REFERENCES Plan_Managers(id)
)",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Participants__Support_Coordinators (
	participants	INTEGER NOT NULL,
	support_coordinators	INTEGER NOT NULL,
	FOREIGN KEY(participants) REFERENCES Participants(id),
	FOREIGN KEY(support_coordinators) REFERENCES Support_Coordinators
)",
            (),
        )?;

        self.connection.execute(
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

        self.connection.execute(
            "
    CREATE TABLE Participants_Likes (
	subject	INTEGER NOT NULL,
	likes	INTEGER NOT NULL,
	FOREIGN KEY(subject) REFERENCES Participants(id),
	FOREIGN KEY(likes) REFERENCES Participants(id)
)",
            (),
        )?;

        self.connection.execute(
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

        self.connection.execute(
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
*/
        self.connection.execute(
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
	private_health_insurer	TEXT,
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
        
        self.connection.execute(
            "
    CREATE TABLE Support_Workers (
	id	INTEGER NOT NULL UNIQUE,
	first_name	TEXT NOT NULL,
	last_name	TEXT NOT NULL,
    phone	TEXT NOT NULL,
    email	TEXT NOT NULL,
	dob	TEXT,
	address	TEXT,
	suburb	TEXT,
	postcode	TEXT,
	first_aid	INTEGER,
	confidentiality_agreement	INTEGER,
	police_clearance	INTEGER,
	car_insurance	INTEGER,
	other_qualifications	TEXT,
	notes	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Venues (
	id	INTEGER NOT NULL UNIQUE,
	name	TEXT NOT NULL,
	address	TEXT,
	suburb	TEXT,
	postcode	TEXT,
	state	TEXT,
	description	TEXT,
	contact_person_name	TEXT,
	contact_person_phone	TEXT,
	venue_phone_number	TEXT,
	price	TEXT,
	notes	TEXT,
	PRIMARY KEY(id AUTOINCREMENT)
)",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Workshop__Support_Worker (
	workshop	INTEGER NOT NULL,
	support_worker	INTEGER NOT NULL,
	FOREIGN KEY(support_worker) REFERENCES Support_Workers(id),
	FOREIGN KEY(workshop) REFERENCES Workshops(id)
)",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Workshop__Participants (
	workshop	INTEGER NOT NULL,
	participant	INTEGER NOT NULL,
	FOREIGN KEY(participant) REFERENCES Participants(id),
	FOREIGN KEY(workshop) REFERENCES Workshops(id)
)",
            (),
        )?;
        self.connection.execute(
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

    pub fn drop_db(&self) -> Result<()> {
        let tables = [
            /*
            "LineItems",
            "Medical_Contacts",
            "Parents",
            "Participant__Medical_Contact",
            
            "Participants__LineItems",
            "Participants__Parents",
            "Participants__Plan_Managers",
            "Participants__Support_Coordinators",
            "Participants_Dislike",
            "Participants_Likes",
            "Plan_Managers",
            "Support_Coordinators", */
            "Participants",
            "Support_Workers",
            "Venues",
            "Workshop__Support_Worker",
            "Workshop__Participants",
            "Workshops",
        ];
        for table in tables.iter() {
            self.connection
                .execute(&format!("DROP TABLE IF EXISTS {}", table), [])?;
        }
        Ok(())
    }
}
impl Default for DataBase {
    fn default() -> Self {
        DataBase::new_connection(String::from("LightBulb.db"))
    }
}
