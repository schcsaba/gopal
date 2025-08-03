use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ContactInfo {
    pub phone: String,
    pub email: String,
    pub address: String,
    pub hours: Vec<OpeningHours>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OpeningHours {
    pub day: Day,
    pub slots: Vec<TimeSlot>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    pub fn to_french(&self) -> &'static str {
        match self {
            Day::Monday => "Lundi",
            Day::Tuesday => "Mardi",
            Day::Wednesday => "Mercredi",
            Day::Thursday => "Jeudi",
            Day::Friday => "Vendredi",
            Day::Saturday => "Samedi",
            Day::Sunday => "Dimanche",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TimeSlot {
    pub start: String,
    pub end: String,
}

impl TimeSlot {
    pub fn format(&self) -> String {
        format!("{} à {}", self.start.replace(":", "h"), self.end.replace(":", "h"))
    }
}

impl ContactInfo {
    pub fn format_phone_display(&self) -> String {
        if self.phone.starts_with("+33") {
            format!("0{}", &self.phone[3..])
        } else {
            self.phone.clone()
        }
    }

    pub fn format_phone_for_display(&self) -> String {
        let display_phone = self.format_phone_display();
        format!("{} {} {} {} {}", 
            &display_phone[0..2],
            &display_phone[2..4], 
            &display_phone[4..6],
            &display_phone[6..8],
            &display_phone[8..10]
        )
    }
}

pub fn get_contact_info() -> ContactInfo {
    ContactInfo {
        phone: "+33783654565".to_string(),
        email: "contact@legopal.fr".to_string(),
        address: "8 Avenue du Mans, 37100 Tours, France".to_string(),
        hours: vec![
            OpeningHours {
                day: Day::Tuesday,
                slots: vec![TimeSlot {
                    start: "12:00".to_string(),
                    end: "14:00".to_string(),
                }],
            },
            OpeningHours {
                day: Day::Wednesday,
                slots: vec![TimeSlot {
                    start: "12:00".to_string(),
                    end: "14:00".to_string(),
                }],
            },
            OpeningHours {
                day: Day::Thursday,
                slots: vec![TimeSlot {
                    start: "12:00".to_string(),
                    end: "14:00".to_string(),
                }],
            },
            OpeningHours {
                day: Day::Friday,
                slots: vec![
                    TimeSlot {
                        start: "12:00".to_string(),
                        end: "14:00".to_string(),
                    },
                    TimeSlot {
                        start: "19:00".to_string(),
                        end: "21:30".to_string(),
                    },
                ],
            },
                    OpeningHours {
            day: Day::Saturday,
            slots: vec![TimeSlot {
                start: "12:00".to_string(),
                end: "14:00".to_string(),
            }],
        },
        ],
    }
}

pub fn format_opening_hours_text(hours: &OpeningHours) -> String {
    match hours.slots.len() {
        1 => format!("{} : {}", hours.day.to_french(), hours.slots[0].format()),
        2 => format!("{} : {} et {}", 
            hours.day.to_french(), 
            hours.slots[0].format(), 
            hours.slots[1].format()
        ),
        _ => format!("{} : Fermé", hours.day.to_french()),
    }
}