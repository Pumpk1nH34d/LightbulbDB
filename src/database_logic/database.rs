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

    pub fn populate_database(&self) -> Result<()> {
        self.connection.execute(
            "
        INSERT INTO Participants (
            first_name, last_name, medicare_number, dob, address, suburb, postcode, phone, email, medical_notes, dietary_notes, physical_notes, other_notes, support_ratio, photo_permission, private_hospital_preference, private_health_insurer, private_health_number, communication_preference, ndis_plan_number, ndis_plan_start_date, core_funding, capacity_building_funding, self_managed, plan_managed, ndis_plan_end_date
        ) VALUES
        ('John', 'Doe', '1234567890', '1980-01-01', '123 Elm St', 'Springfield', '12345', '0412-345-678', 'john.doe@example.com', 'Diabetic', 'None', 'Allergic to peanuts', 'None', 'High', 1, 0, 'HealthInsureCo', 'HI123456', 'Email', '123456', '2023-01-01', 10000, 5000, 1, 0, '2024-12-31'),
        ('Jane', 'Smith', '9876543210', '1990-02-02', '456 Oak St', 'Springfield', '12345', '0412-567-890', 'jane.smith@example.com', 'Hypertension', 'Vegetarian', 'None', 'Needs mobility aid', 'Medium', 0, 1, 'HealthInsureInc', 'HI654321', 'Phone', '654321', '2023-06-01', 8000, 4000, 0, 1, '2025-05-31'),
        ('Alice', 'Johnson', '1230984567', '1985-03-05', '789 Pine St', 'Springfield', '12345', '0412-876-543', 'alice.johnson@example.com', 'Asthma', 'Gluten-free', 'None', 'Uses CPAP machine', 'Low', 1, 1, 'HealthInsureLtd', 'HI789012', 'In-person', '789012', '2023-03-01', 12000, 6000, 1, 0, '2024-09-30'),
        ('Bob', 'Williams', '4567890123', '1994-10-04', '101 Maple St', 'Springfield', '12345', '0412-432-109', 'bob.williams@example.com', 'Epilepsy', 'None', 'Has prosthetic leg', 'Occasional seizures', 'Medium', 0, 0, 'HealthInsureCo', 'HI345678', 'Email', '345678', '2023-11-01', 9000, 4500, 0, 1, '2025-02-28'),
        ('Carol', 'Davis', '7890123456', '1987-12-07', '202 Birch St', 'Springfield', '12345', '0412-135-790', 'carol.davis@example.com', 'Chronic pain', 'Low-sugar diet', 'Needs physical therapy', 'None', 'High', 1, 1, 'HealthInsureInc', 'HI901234', 'Phone', '901234', '2023-07-01', 11000, 5500, 1, 0, '2024-10-31'),
        ('David', 'Miller', '2345678901', '1992-11-08', '303 Cedar St', 'Springfield', '12345', '0412-246-810', 'david.miller@example.com', 'Depression', 'None', 'Requires counseling', 'Occasional anxiety attacks', 'Low', 1, 0, 'HealthInsureLtd', 'HI234567', 'In-person', '234567', '2023-09-01', 8500, 4250, 0, 1, '2025-03-31'),
        ('Emma', 'Wilson', '5678901234', '1995-12-09', '404 Fir St', 'Springfield', '12345', '0412-987-654', 'emma.wilson@example.com', 'Autism', 'Vegetarian', 'Sensory sensitivities', 'None', 'Medium', 0, 1, 'HealthInsureCo', 'HI890123', 'Email', '890123', '2023-05-01', 10000, 5000, 1, 0, '2024-08-31'),
        ('Frank', 'Moore', '8901234567', '1983-03-10', '505 Spruce St', 'Springfield', '12345', '0412-678-901', 'frank.moore@example.com', 'Multiple sclerosis', 'Gluten-free', 'Needs assistance with mobility', 'Occasional fatigue', 'High', 1, 1, 'HealthInsureInc', 'HI123789', 'Phone', '123789', '2023-04-01', 11500, 5750, 1, 0, '2025-01-31'),
        ('Grace', 'Taylor', '1234567891', '1996-09-11', '606 Fir St', 'Springfield', '12345', '0412-234-567', 'grace.taylor@example.com', 'Cancer', 'Low-sodium diet', 'Undergoing chemotherapy', 'None', 'Low', 0, 1, 'HealthInsureLtd', 'HI456789', 'In-person', '456789', '2023-10-01', 9500, 4750, 0, 1, '2024-07-31'),
        ('Ivy', 'Martin', '3456789012', '2000-07-15', '808 Cedar St', 'Springfield', '12345', '0412-543-210', 'ivy.martin@example.com', 'Scoliosis', 'None', 'Uses back brace', 'Occasional pain', 'Medium', 1, 0, 'HealthInsureLtd', 'HI678901', 'In-person', '678901', '2023-08-01', 10500, 5250, 0, 1, '2025-07-31'),
        ('Jack', 'Moore', '4567890123', '1999-05-21', '909 Elm St', 'Springfield', '12345', '0412-654-321', 'jack.moore@example.com', 'Chronic migraines', 'Low-caffeine diet', 'Needs regular rest', 'None', 'High', 1, 1, 'HealthInsureInc', 'HI789012', 'Email', '789012', '2023-11-01', 9500, 4750, 1, 0, '2024-11-30');
        ",
            (),
        )?;
        self.connection.execute(
            "
        INSERT INTO Support_Workers (
            first_name, last_name, phone, email, dob, address, suburb, postcode, first_aid, confidentiality_agreement, police_clearance, car_insurance, other_qualifications, notes
        ) VALUES
        ('Anna', 'Brown', '0412-345-678', 'anna.brown@example.com', '1985-01-20', '12 Maple St', 'Springfield', '12345', 1, 1, 1, 1, 'Cert IV in Disability', 'Experienced in working with children'),
        ('Brian', 'Clark', '0412-567-890', 'brian.clark@example.com', '1990-02-15', '34 Oak St', 'Springfield', '12345', 1, 0, 1, 0, 'Diploma in Aged Care', 'Specializes in elderly care'),
        ('Clara', 'Davis', '0412-678-901', 'clara.davis@example.com', '1982-03-10', '56 Pine St', 'Springfield', '12345', 0, 1, 0, 1, 'First Aid Level 2', 'Strong background in mental health support'),
        ('David', 'Evans', '0412-789-012', 'david.evans@example.com', '1987-04-05', '78 Cedar St', 'Springfield', '12345', 1, 1, 1, 1, 'Registered Nurse', 'Experienced in medical support'),
        ('Ella', 'Fisher', '0412-890-123', 'ella.fisher@example.com', '1995-05-25', '90 Birch St', 'Springfield', '12345', 1, 0, 1, 0, 'Diploma in Community Services', 'Expert in disability support'),
        ('Frank', 'Garcia', '0412-901-234', 'frank.garcia@example.com', '1980-06-30', '12 Spruce St', 'Springfield', '12345', 0, 1, 1, 1, 'Cert III in Individual Support', 'Skilled in personal care'),
        ('Grace', 'Harris', '0412-012-345', 'grace.harris@example.com', '1992-07-22', '34 Elm St', 'Springfield', '12345', 1, 1, 0, 1, 'Degree in Social Work', 'Background in case management'),
        ('Henry', 'Jackson', '0412-123-456', 'henry.jackson@example.com', '1988-08-18', '56 Fir St', 'Springfield', '12345', 1, 1, 1, 1, 'Cert IV in Allied Health Assistance', 'Specializes in physical therapy'),
        ('Ivy', 'King', '0412-234-567', 'ivy.king@example.com', '1994-09-14', '78 Willow St', 'Springfield', '12345', 0, 0, 1, 0, 'Cert III in Disability Support', 'Focuses on support for adults'),
        ('Jack', 'Lewis', '0412-345-678', 'jack.lewis@example.com', '1983-10-19', '90 Oak St', 'Springfield', '12345', 1, 1, 1, 1, 'Diploma in Mental Health', 'Experienced in crisis intervention');
        ",
            (),
        )?;
        
        self.connection.execute(
            "
        INSERT INTO Venues (
            name, address, suburb, postcode, state, description, contact_person_name, contact_person_phone, venue_phone_number, price, notes
        ) VALUES
        ('Springfield Community Hall', '123 Community Rd', 'Springfield', '12345', 'NSW', 'A spacious hall ideal for community events', 'Sarah Jones', '0412-123-456', '02-1234-5678', 'Free', 'Accessible venue with kitchen facilities'),
        ('Oak Tree Conference Centre', '456 Oak Ave', 'Springfield', '12345', 'VIC', 'Modern conference centre with state-of-the-art facilities', 'John Smith', '0412-234-567', '03-2345-6789', '500 AUD/day', 'Includes projector and audio system'),
        ('Pine View Auditorium', '789 Pine Blvd', 'Springfield', '12345', 'QLD', 'Large auditorium suitable for workshops and seminars', 'Emily White', '0412-345-678', '07-3456-7890', '750 AUD/day', 'Offers seating for up to 300 people'),
        ('Cedar Grove Meeting Room', '101 Cedar St', 'Springfield', '12345', 'SA', 'Intimate meeting room for small groups', 'Michael Brown', '0412-456-789', '08-4567-8901', '250 AUD/day', 'Whiteboard and video conferencing available'),
        ('Birchwood Training Centre', '202 Birch St', 'Springfield', '12345', 'WA', 'Training centre with multiple breakout rooms', 'Jessica Green', '0412-567-890', '09-5678-9012', '600 AUD/day', 'Catering options available upon request'),
        ('Willow Creek Event Space', '303 Willow St', 'Springfield', '12345', 'TAS', 'Versatile event space with outdoor area', 'Daniel Black', '0412-678-901', '03-6789-0123', '400 AUD/day', 'Perfect for workshops and social events'),
        ('Elm Street Recreational Hall', '404 Elm St', 'Springfield', '12345', 'NT', 'Recreational hall with sports facilities', 'Olivia Martin', '0412-789-012', '08-7890-1234', '300 AUD/day', 'Includes gym equipment and multi-purpose space'),
        ('Holly Garden Centre', '505 Holly Rd', 'Springfield', '12345', 'ACT', 'Garden centre with indoor and outdoor options', 'James Wilson', '0412-890-123', '02-8901-2345', '350 AUD/day', 'Beautiful garden setting for events'),
        ('Linden Park Clubhouse', '606 Linden Park', 'Springfield', '12345', 'NSW', 'Clubhouse with modern amenities', 'Sophia Lewis', '0412-901-234', '02-9012-3456', '550 AUD/day', 'Includes catering and event planning services'),
        ('Maple Leaf Venue', '707 Maple St', 'Springfield', '12345', 'VIC', 'Elegant venue for formal events and gatherings', 'Ethan Harris', '0412-012-345', '03-0123-4567', '800 AUD/day', 'High-end facilities with on-site manager');
        ",
            (),
        )?;

        self.connection.execute(
            "
        INSERT INTO Workshops (
            name, facilitator, venue, start_date, end_date
        ) VALUES
        ('Introduction to Cooking', 1, 1, '2024-09-01', '2024-09-03'),
        ('Basic First Aid Training', 2, 2, '2024-10-05', '2024-10-07'),
        ('Digital Literacy Workshop', 3, 3, '2024-11-10', '2024-11-12'),
        ('Managing Chronic Conditions', 4, 4, '2024-12-15', '2024-12-17'),
        ('Social Skills Development', 5, 5, '2025-01-20', '2025-01-22'),
        ('Employment Readiness', 6, 6, '2025-02-25', '2025-02-27'),
        ('Health and Wellness Seminar', 7, 7, '2025-03-30', '2025-04-01'),
        ('Independent Living Skills', 8, 8, '2025-05-10', '2025-05-12'),
        ('Financial Management Basics', 9, 9, '2025-06-15', '2025-06-17'),
        ('Creative Arts and Crafts', 10, 10, '2025-07-20', '2025-07-22');
        ",
            (),
        )?;
        
        Ok(())
        
        
    }

}
impl Default for DataBase {
    fn default() -> Self {
        DataBase::new_connection(String::from("LightBulb.db"))
    }
}
