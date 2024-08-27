use rusqlite::{Connection, Result};

/// A struct representing the database connection.
pub struct DataBase {
    pub connection: Connection,
}

impl DataBase {
    /// Creates a new `DataBase` instance with a connection to the specified SQLite database file.
    ///
    /// # Arguments
    ///
    /// * `db_file_name` - A `String` representing the path to the SQLite database file.
    ///
    /// # Returns
    ///
    /// A `DataBase` instance with an open connection to the specified SQLite file.
    fn new_connection(db_file_name: String) -> DataBase {
        DataBase {
            connection: Connection::open(db_file_name).unwrap(),
        }
    }

    /// Creates all necessary tables for the database.
    ///
    /// This method creates tables for participants, support workers, venues, workshops, and their relationships.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or failure (`Err(e)`) of the operation.
    pub fn create_db(&self) -> Result<()> {
        // Create the `Participants` table
        self.connection.execute(
            "
    CREATE TABLE Participants (
        id INTEGER NOT NULL UNIQUE,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        medicare_number TEXT NOT NULL,
        dob INTEGER,
        address TEXT,
        suburb TEXT,
        postcode TEXT,
        phone TEXT,
        email TEXT,
        medical_notes TEXT,
        dietary_notes TEXT,
        physical_notes TEXT,
        other_notes TEXT,
        support_ratio TEXT,
        photo_permission INTEGER,
        private_hospital_preference INTEGER,
        private_health_insurer TEXT,
        private_health_number TEXT,
        communication_preference TEXT,
        ndis_plan_number TEXT,
        ndis_plan_start_date TEXT,
        core_funding INTEGER,
        capacity_building_funding INTEGER,
        self_managed INTEGER,
        plan_managed INTEGER,
        ndis_plan_end_date TEXT,
        PRIMARY KEY(id AUTOINCREMENT)
    )",
            (),
        )?;

        // Create the `Support_Workers` table
        self.connection.execute(
            "
    CREATE TABLE Support_Workers (
        id INTEGER NOT NULL UNIQUE,
        first_name TEXT NOT NULL,
        last_name TEXT NOT NULL,
        phone TEXT NOT NULL,
        email TEXT NOT NULL,
        dob TEXT,
        address TEXT,
        suburb TEXT,
        postcode TEXT,
        first_aid INTEGER,
        confidentiality_agreement INTEGER,
        police_clearance INTEGER,
        car_insurance INTEGER,
        other_qualifications TEXT,
        notes TEXT,
        PRIMARY KEY(id AUTOINCREMENT)
    )",
            (),
        )?;

        // Create the `Venues` table
        self.connection.execute(
            "
    CREATE TABLE Venues (
        id INTEGER NOT NULL UNIQUE,
        name TEXT NOT NULL,
        address TEXT,
        suburb TEXT,
        postcode TEXT,
        state TEXT,
        description TEXT,
        contact_person_name TEXT,
        contact_person_phone TEXT,
        venue_phone_number TEXT,
        price TEXT,
        notes TEXT,
        PRIMARY KEY(id AUTOINCREMENT)
    )",
            (),
        )?;

        // Create the `Workshops` table
        self.connection.execute(
            "
    CREATE TABLE Workshops (
        id INTEGER NOT NULL UNIQUE,
        name TEXT NOT NULL,
        facilitator INTEGER NOT NULL,
        venue INTEGER NOT NULL,
        start_date TEXT NOT NULL,
        end_date TEXT NOT NULL,
        FOREIGN KEY(facilitator) REFERENCES Support_Workers(id),
        FOREIGN KEY(venue) REFERENCES Venues(id),
        PRIMARY KEY(id AUTOINCREMENT)
    )",
            (),
        )?;

        // Create the many-to-many relationship tables
        self.connection.execute(
            "
    CREATE TABLE Workshop__Support_Worker (
        workshop INTEGER NOT NULL,
        support_worker INTEGER NOT NULL,
        FOREIGN KEY(support_worker) REFERENCES Support_Workers(id),
        FOREIGN KEY(workshop) REFERENCES Workshops(id)
    )",
            (),
        )?;

        self.connection.execute(
            "
    CREATE TABLE Workshop__Participants (
        workshop INTEGER NOT NULL,
        participant INTEGER NOT NULL,
        FOREIGN KEY(participant) REFERENCES Participants(id),
        FOREIGN KEY(workshop) REFERENCES Workshops(id)
    )",
            (),
        )?;

        Ok(())
    }

    /// Drops all tables from the database.
    ///
    /// This method removes the specified tables from the database if they exist.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or failure (`Err(e)`) of the operation.
    pub fn drop_db(&self) -> Result<()> {
        let tables = [
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

    /// Populates the database with initial data.
    ///
    /// This method inserts sample data into the `Participants`, `Support_Workers`, and `Venues` tables.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success (`Ok(())`) or failure (`Err(e)`) of the operation.
    pub fn populate_database(&self) -> Result<()> {
        // Insert sample data into `Participants` table
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
        ('Emma', 'Wilson', '5678901234', '1995-12-09', '404 Fir St', 'Springfield', '12345', '0412-987-654', 'emma.wilson@example.com', 'Autism', 'Vegetarian', 'Sensory processing issues', 'None', 'Medium', 0, 1, 'HealthInsureCo', 'HI890123', 'Email', '890123', '2023-02-01', 9500, 4750, 1, 0, '2024-08-31'),
        ('Frank', 'Taylor', '8901234567', '1989-05-10', '505 Spruce St', 'Springfield', '12345', '0412-543-210', 'frank.taylor@example.com', 'Multiple sclerosis', 'Low-sodium diet', 'Requires wheelchair access', 'None', 'High', 1, 1, 'HealthInsureInc', 'HI123098', 'Phone', '123098', '2023-05-01', 11500, 5750, 1, 0, '2024-11-30'),
        ('Grace', 'Anderson', '0123456789', '2000-04-11', '606 Willow St', 'Springfield', '12345', '0412-654-321', 'grace.anderson@example.com', 'Cancer', 'Gluten-free', 'Needs chemotherapy', 'None', 'Low', 0, 0, 'HealthInsureLtd', 'HI678901', 'In-person', '678901', '2023-08-01', 10500, 5250, 0, 1, '2025-01-31'),
        ('Henry', 'Thomas', '3456789012', '1988-07-12', '707 Poplar St', 'Springfield', '12345', '0412-321-654', 'henry.thomas@example.com', 'Diabetes', 'Low-sugar diet', 'Requires insulin', 'None', 'Medium', 1, 1, 'HealthInsureCo', 'HI345678', 'Email', '345678', '2023-04-01', 8000, 4000, 1, 0, '2025-06-30'),
        ('Isabella', 'Jackson', '6789012345', '1997-08-13', '808 Chestnut St', 'Springfield', '12345', '0412-876-543', 'isabella.jackson@example.com', 'Arthritis', 'None', 'Needs physical therapy', 'None', 'Low', 0, 1, 'HealthInsureInc', 'HI789012', 'Phone', '789012', '2023-10-01', 10000, 5000, 0, 1, '2025-07-31');
        ",
            (),
        )?;

        // Insert sample data into `Support_Workers` table
        self.connection.execute(
            "
        INSERT INTO Support_Workers (
            first_name, last_name, phone, email, dob, address, suburb, postcode, first_aid, confidentiality_agreement, police_clearance, car_insurance, other_qualifications, notes
        ) VALUES
        ('Sarah', 'Brown', '0412-345-678', 'sarah.brown@example.com', '1985-01-01', '12 Queen St', 'Springfield', '12345', 1, 1, 1, 1, 'Certificate IV in Disability', 'Experienced in disability support'),
        ('Michael', 'Green', '0412-567-890', 'michael.green@example.com', '1990-05-15', '34 King St', 'Springfield', '12345', 1, 1, 1, 1, 'Diploma in Community Services', 'Skilled in community engagement'),
        ('Linda', 'Clark', '0412-876-543', 'linda.clark@example.com', '1982-09-10', '56 Prince St', 'Springfield', '12345', 1, 1, 1, 0, 'First Aid Certificate', 'Knowledgeable in medical emergencies'),
        ('Robert', 'Lewis', '0412-135-790', 'robert.lewis@example.com', '1993-11-20', '78 Duke St', 'Springfield', '12345', 1, 0, 1, 1, 'Cert III in Aged Care', 'Passionate about aged care'),
        ('Emily', 'Walker', '0412-246-810', 'emily.walker@example.com', '1988-03-25', '90 Duchess St', 'Springfield', '12345', 1, 1, 0, 1, 'Advanced Diploma in Mental Health', 'Experienced with mental health support'),
        ('James', 'Young', '0412-543-210', 'james.young@example.com', '1986-07-30', '12 Earl St', 'Springfield', '12345', 1, 1, 1, 0, 'Certificate IV in Mental Health', 'Well-versed in mental health'),
        ('Olivia', 'Harris', '0412-987-654', 'olivia.harris@example.com', '1994-12-05', '34 Lady St', 'Springfield', '12345', 0, 1, 1, 1, 'Cert IV in Disability Support', 'Specialist in disability support'),
        ('Daniel', 'Martin', '0412-654-321', 'daniel.martin@example.com', '1987-06-14', '56 Lord St', 'Springfield', '12345', 1, 1, 0, 1, 'Cert III in Individual Support', 'Experienced in individual support'),
        ('Sophia', 'King', '0412-876-543', 'sophia.king@example.com', '1991-02-22', '78 Knight St', 'Springfield', '12345', 1, 1, 1, 1, 'Diploma in Aged Care', 'Expert in aged care'),
        ('William', 'Scott', '0412-321-654', 'william.scott@example.com', '1989-08-10', '90 Baron St', 'Springfield', '12345', 1, 0, 1, 0, 'Certificate IV in Community Services', 'Skilled in community services');
        ",
            (),
        )?;

        // Insert sample data into `Venues` table
        self.connection.execute(
            "
        INSERT INTO Venues (
            name, address, suburb, postcode, state, description, contact_person_name, contact_person_phone, venue_phone_number, price, notes
        ) VALUES
        ('Community Hall', '123 Main St', 'Springfield', '12345', 'NSW', 'Large hall with kitchen', 'Alice Johnson', '0412-345-678', '02-1234-5678', '500', 'Available for all types of events'),
        ('Local Park', '456 Oak St', 'Springfield', '12345', 'NSW', 'Outdoor space with BBQ facilities', 'Bob Smith', '0412-567-890', '02-2345-6789', '150', 'Great for picnics and small gatherings'),
        ('City Library', '789 Pine St', 'Springfield', '12345', 'NSW', 'Quiet meeting rooms', 'Carol Lee', '0412-876-543', '02-3456-7890', '200', 'Perfect for workshops and seminars'),
        ('Sports Complex', '101 Maple St', 'Springfield', '12345', 'NSW', 'Facilities for sports and fitness', 'David Brown', '0412-135-790', '02-4567-8901', '300', 'Includes gym and swimming pool'),
        ('Art Gallery', '202 Birch St', 'Springfield', '12345', 'NSW', 'Exhibition space and café', 'Emma Wilson', '0412-246-810', '02-5678-9012', '400', 'Ideal for exhibitions and social events'),
        ('Convention Center', '303 Cedar St', 'Springfield', '12345', 'NSW', 'Large conference rooms', 'Frank Davis', '0412-543-210', '02-6789-0123', '800', 'Suitable for large conferences and conventions'),
        ('Dance Studio', '404 Fir St', 'Springfield', '12345', 'NSW', 'Studio space for dancing', 'Grace Taylor', '0412-987-654', '02-7890-1234', '250', 'Great for dance classes and workshops'),
        ('Music Hall', '505 Spruce St', 'Springfield', '12345', 'NSW', 'Venue for musical performances', 'Henry Martin', '0412-654-321', '02-8901-2345', '350', 'Perfect for concerts and musical events'),
        ('Cooking School', '606 Willow St', 'Springfield', '12345', 'NSW', 'Kitchen facilities and dining area', 'Isabella Wilson', '0412-876-543', '02-9012-3456', '450', 'Ideal for cooking classes and events'),
        ('Tech Hub', '707 Poplar St', 'Springfield', '12345', 'NSW', 'Technology and innovation space', 'James Harris', '0412-321-654', '02-0123-4567', '500', 'Great for tech workshops and meetings');
        ",
            (),
        )?;
        // Insert sample data into `Workshops` table
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

        // Insert sample data into `Workshop__Participants` table
        self.connection.execute(
            "
            INSERT INTO Workshop__Participants (workshop, participant) VALUES
            (1, 1),
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 5),
            (3, 6),
            (4, 7),
            (4, 8),
            (5, 9),
            (5, 10),
            (6, 1),
            (6, 3),
            (7, 2),
            (7, 4),
            (8, 5),
            (8, 6),
            (9, 7),
            (9, 9),
            (10, 8),
            (10, 10),
            (1, 3),
            (2, 4),
            (3, 7),
            (4, 6),
            (5, 5),
            (6, 9),
            (7, 8),
            (8, 2),
            (9, 10),
            (10, 1);
            ",
            (),
        )?;

        // Insert sample data into `Workshop__Support_Worker` table
        self.connection.execute(
            "
            INSERT INTO Workshop__Support_Worker (workshop, support_worker) VALUES
            (1, 1),
            (1, 2),
            (2, 3),
            (2, 4),
            (3, 5),
            (3, 6),
            (4, 7),
            (4, 8),
            (5, 9),
            (5, 10),
            (6, 1),
            (6, 3),
            (7, 2),
            (7, 4),
            (8, 5),
            (8, 6),
            (9, 7),
            (9, 9),
            (10, 8),
            (10, 10),
            (1, 3),
            (2, 4),
            (3, 7),
            (4, 6),
            (5, 5),
            (6, 9),
            (7, 8),
            (8, 2),
            (9, 10),
            (10, 1);
            ",
            (),
        )?;
        Ok(())
    }
}

// Implement Default trait for DataBase
impl Default for DataBase {
    fn default() -> Self {
        DataBase::new_connection(String::from("LightBulb.db"))
    }
}