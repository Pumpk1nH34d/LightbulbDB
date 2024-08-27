use chrono::NaiveDate; // Import NaiveDate from the chrono crate for date handling


//todo: comment code

/*
#[derive(Debug)]
pub struct LineItem {
    pub name: String,
    pub description: String,
    pub support_category: Option<String>,
}

#[derive(Debug)]
pub struct MedicalContact {
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub relationship: Option<String>,
}

#[derive(Debug)]
pub struct Parent {
    pub first_name: String,
    pub last_name: String,
    pub relationship: String,
    pub phone_number: String,
    pub communication_preference: Option<String>,
}

#[derive(Debug)]
pub struct ParticipantMedicalContact {
    pub participant: i32,
    pub medical_contact: i32,
}

#[derive(Debug)]
pub struct ParticipantLineItem {
    pub participants: i32,
    pub lineitem: i32,
}

#[derive(Debug)]
pub struct ParticipantParent {
    pub participant: i32,
    pub parent: i32,
}

#[derive(Debug)]
pub struct ParticipantPlanManager {
    pub participant: i32,
    pub plan_manager: i32,
}

#[derive(Debug)]
pub struct ParticipantSupportCoordinator {
    pub participants: i32,
    pub support_coordinators: i32,
}

#[derive(Debug)]
pub struct ParticipantDislike {
    pub subject: i32,
    pub dislikes: i32,
}

#[derive(Debug)]
pub struct ParticipantLike {
    pub subject: i32,
    pub likes: i32,
}

#[derive(Debug)]
pub struct PlanManager {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub phone: Option<String>,
    pub company_address: Option<String>,
    pub email: Option<String>,
    pub email_invoice: Option<String>,
    pub company_phone: Option<String>,
}

#[derive(Debug)]
pub struct SupportCoordinator {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub company_phone: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub company_email: Option<String>,
}
*/

// Define a struct for participants with detailed information
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Participant {
    pub id: Option<i32>, // Optional ID of the participant
    pub first_name: String, // First name of the participant
    pub last_name: String, // Last name of the participant
    pub medicare_number: String, // Medicare number of the participant
    pub dob: Option<NaiveDate>, // Optional date of birth of the participant
    pub address: Option<String>, // Optional address of the participant
    pub suburb: Option<String>, // Optional suburb of the participant
    pub postcode: Option<String>, // Optional postcode of the participant
    pub phone: Option<String>, // Optional phone number of the participant
    pub email: Option<String>, // Optional email address of the participant
    pub medical_notes: Option<String>, // Optional medical notes for the participant
    pub dietary_notes: Option<String>, // Optional dietary notes for the participant
    pub physical_notes: Option<String>, // Optional physical notes for the participant
    pub other_notes: Option<String>, // Optional other notes for the participant
    pub support_ratio: Option<String>, // Optional support ratio for the participant
    pub photo_permission: Option<bool>, // Optional permission for photo usage
    pub private_hospital_preference: Option<bool>, // Optional preference for private hospitals
    pub private_health_insurer: Option<String>, // Optional private health insurer
    pub private_health_number: Option<String>, // Optional private health number
    pub communication_preference: Option<String>, // Optional communication preference
    pub ndis_plan_number: Option<String>, // Optional NDIS plan number
    pub ndis_plan_start_date: Option<NaiveDate>, // Optional NDIS plan start date
    pub core_funding: Option<bool>, // Optional core funding status
    pub capacity_building_funding: Option<bool>, // Optional capacity building funding status
    pub self_managed: Option<bool>, // Optional self-managed status
    pub plan_managed: Option<bool>, // Optional plan managed status
    pub ndis_plan_end_date: Option<NaiveDate>, // Optional NDIS plan end date
}

// Define a struct for support workers with detailed information
#[derive(Debug, Default, Clone)]
pub struct SupportWorker {
    pub id: Option<i32>, // Optional ID of the support worker
    pub first_name: String, // First name of the support worker
    pub last_name: String, // Last name of the support worker
    pub phone: String, // Phone number of the support worker
    pub email: String, // Email address of the support worker
    pub dob: Option<NaiveDate>, // Optional date of birth of the support worker
    pub address: Option<String>, // Optional address of the support worker
    pub suburb: Option<String>, // Optional suburb of the support worker
    pub postcode: Option<String>, // Optional postcode of the support worker
    pub first_aid: Option<bool>, // Optional first aid certification status
    pub confidentiality_agreement: Option<bool>, // Optional confidentiality agreement status
    pub police_clearance: Option<bool>, // Optional police clearance status
    pub car_insurance: Option<bool>, // Optional car insurance status
    pub other_qualifications: Option<String>, // Optional other qualifications
    pub notes: Option<String>, // Optional additional notes
}

// Define a struct for venues with detailed information
#[derive(Debug, Default, Clone)]
pub struct Venue {
    pub id: Option<i32>, // Optional ID of the venue
    pub name: String, // Name of the venue
    pub address: Option<String>, // Optional address of the venue
    pub suburb: Option<String>, // Optional suburb of the venue
    pub postcode: Option<String>, // Optional postcode of the venue
    pub state: Option<String>, // Optional state of the venue
    pub description: Option<String>, // Optional description of the venue
    pub contact_person_name: Option<String>, // Optional contact person name at the venue
    pub contact_person_phone: Option<String>, // Optional contact person phone number
    pub venue_phone_number: Option<String>, // Optional venue phone number
    pub price: Option<String>, // Optional price for using the venue
    pub notes: Option<String>, // Optional additional notes about the venue
}

// Define a struct to link workshops with support workers
#[derive(Debug)]
pub struct WorkshopSupportWorker {
    pub workshop: Workshop, // The workshop instance
    pub support_worker: SupportWorker, // The support worker instance
}

// Define a struct to link workshops with participants
#[derive(Debug)]
pub struct WorkshopParticipant {
    pub workshop: i64, // ID of the workshop
    pub participant: i64, // ID of the participant
}

// Define a struct for workshops with detailed information
#[derive(Debug, Clone, Default)]
pub struct Workshop {
    pub id: Option<i32>, // Optional ID of the workshop
    pub name: String, // Name of the workshop
    pub facilitator: i32, // ID of the facilitator
    pub venue: i32, // ID of the venue
    pub start_date: NaiveDate, // Start date of the workshop
    pub end_date: NaiveDate, // End date of the workshop
}

// Define an enum for sorting options
#[derive(Default, Debug, PartialEq)]
pub enum Sort {
    #[default]
    AlphabeticalAscending, // Sort alphabetically in ascending order
    AlphabeticalDescending, // Sort alphabetically in descending order
}

