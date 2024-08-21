use chrono::NaiveDate;

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
    pub phone: i64,
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
pub struct Participant {
    pub first_name: String,
    pub last_name: String,
    pub medicare_number: String,
    pub dob: Option<NaiveDate>,
    pub address: Option<String>,
    pub suburb: Option<String>,
    pub postcode: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub medical_notes: Option<String>,
    pub dietary_notes: Option<String>,
    pub physical_notes: Option<String>,
    pub other_notes: Option<String>,
    pub support_ratio: Option<String>,
    pub photo_permission: Option<bool>,
    pub private_hospital_preference: Option<bool>,
    pub private_health_insurancer: Option<String>,
    pub private_health_number: Option<String>,
    pub communication_preference: Option<String>,
    pub ndis_plan_number: Option<String>,
    pub ndis_plan_start_date: Option<NaiveDate>,
    pub core_funding: Option<bool>,
    pub capacity_building_funding: Option<bool>,
    pub self_managed: Option<bool>,
    pub plan_managed: Option<bool>,
    pub ndis_plan_end_date: Option<NaiveDate>,
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

#[derive(Debug)]
pub struct SupportWorker {
    pub first_name: String,
    pub last_name: String,
    pub dob: Option<String>,
    pub address: Option<String>,
    pub suburb: Option<String>,
    pub postcode: Option<String>,
    pub first_aid: Option<i32>,
    pub first_aid_file: Option<String>,
    pub confidentiality_agreement: Option<String>,
    pub police_clearance: Option<String>,
    pub car_insurance: Option<i32>,
    pub other_qualifications: Option<String>,
    pub notes: Option<String>,
    pub phone: String,
}

#[derive(Debug)]
pub struct Venue {
    pub name: String,
    pub address: Option<String>,
    pub suburb: Option<String>,
    pub postcode: Option<String>,
    pub state: String,
    pub description: Option<String>,
    pub contact_person_name: Option<String>,
    pub contact_person_phone: Option<String>,
    pub venue_phone_number: Option<String>,
    pub price: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct WorkshopSupportWorker {
    pub workshop: i32,
    pub support_worker: i32,
}

#[derive(Debug)]
pub struct Workshop {
    pub name: String,
    pub facilitator: i32,
    pub venue: i32,
    pub start_date: String,
    pub end_date: String,
}