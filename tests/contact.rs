// Tests for the actual contact data module from src/data/contact.rs
use gopal::data::contact::{get_contact_info, format_opening_hours_text, format_all_opening_hours_html, Day, TimeSlot, OpeningHours};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_contact_info_from_src() {
        let contact = get_contact_info();
        
        assert_eq!(contact.phone, "+33783654565");
        assert_eq!(contact.email, "contact@legopal.fr");
        assert_eq!(contact.address, "8 Avenue du Mans, 37100 Tours, France");
        assert_eq!(contact.hours.len(), 5); // Tuesday through Saturday
    }

    #[test]
    fn test_phone_display_formatting_from_src() {
        let contact = get_contact_info();
        assert_eq!(contact.format_phone_display(), "0783654565");
        assert_eq!(contact.format_phone_for_display(), "07 83 65 45 65");
    }

    #[test]
    fn test_day_to_french() {
        assert_eq!(Day::Tuesday.to_french(), "Mardi");
        assert_eq!(Day::Friday.to_french(), "Vendredi");
        assert_eq!(Day::Saturday.to_french(), "Samedi");
    }

    #[test]
    fn test_time_slot_format() {
        let slot = TimeSlot {
            start: "12:00".to_string(),
            end: "14:00".to_string(),
        };
        assert_eq!(slot.format(), "12h00 à 14h00");
    }

    #[test]
    fn test_format_opening_hours_text() {
        let tuesday_hours = OpeningHours {
            day: Day::Tuesday,
            slots: vec![TimeSlot {
                start: "12:00".to_string(),
                end: "14:00".to_string(),
            }],
        };
        assert_eq!(format_opening_hours_text(&tuesday_hours), "Mardi : 12h00 à 14h00");

        let friday_hours = OpeningHours {
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
        };
        assert_eq!(format_opening_hours_text(&friday_hours), "Vendredi : 12h00 à 14h00 et 19h00 à 21h30");
    }

    #[test]
    fn test_actual_contact_data_structure() {
        let contact = get_contact_info();
        
        // Test that Tuesday exists and has the expected format
        let tuesday = contact.hours.iter().find(|h| matches!(h.day, Day::Tuesday)).unwrap();
        assert_eq!(tuesday.slots.len(), 1);
        
        // Test that Friday exists and has two time slots
        let friday = contact.hours.iter().find(|h| matches!(h.day, Day::Friday)).unwrap();
        assert_eq!(friday.slots.len(), 2);
    }

    #[test]
    fn test_format_all_opening_hours_html() {
        // Test with empty list
        let empty_hours: Vec<OpeningHours> = vec![];
        assert_eq!(format_all_opening_hours_html(&empty_hours), "");

        // Test with single day, single time slot
        let tuesday_hours = vec![OpeningHours {
            day: Day::Tuesday,
            slots: vec![TimeSlot {
                start: "12:00".to_string(),
                end: "14:00".to_string(),
            }],
        }];
        let expected_tuesday = "<div class=\"mb-4\"><span class=\"font-bold\">Mardi</span><br/>12h00 à 14h00</div>";
        assert_eq!(format_all_opening_hours_html(&tuesday_hours), expected_tuesday);

        // Test with single day, two time slots
        let friday_hours = vec![OpeningHours {
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
        }];
        let expected_friday = "<div class=\"mb-4\"><span class=\"font-bold\">Vendredi</span><br/>12h00 à 14h00 et 19h00 à 21h30</div>";
        assert_eq!(format_all_opening_hours_html(&friday_hours), expected_friday);

        // Test with closed day (no time slots)
        let closed_day = vec![OpeningHours {
            day: Day::Monday,
            slots: vec![],
        }];
        let expected_closed = "<div class=\"mb-4\"><span class=\"font-bold\">Lundi</span><br/>Fermé</div>";
        assert_eq!(format_all_opening_hours_html(&closed_day), expected_closed);

        // Test with multiple days
        let multi_days = vec![
            OpeningHours {
                day: Day::Tuesday,
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
        ];
        let expected_multi = "<div class=\"mb-4\"><span class=\"font-bold\">Mardi</span><br/>12h00 à 14h00</div><div class=\"mb-4\"><span class=\"font-bold\">Vendredi</span><br/>12h00 à 14h00 et 19h00 à 21h30</div>";
        assert_eq!(format_all_opening_hours_html(&multi_days), expected_multi);
    }

    #[test]
    fn test_format_all_opening_hours_html_with_actual_data() {
        let contact = get_contact_info();
        let html_output = format_all_opening_hours_html(&contact.hours);
        
        // Verify it contains expected elements
        assert!(html_output.contains("Mardi"));
        assert!(html_output.contains("Vendredi"));
        assert!(html_output.contains("12h00 à 14h00"));
        assert!(html_output.contains("19h00 à 21h30"));
        assert!(html_output.contains("<div class=\"mb-4\">"));
        assert!(html_output.contains("<span class=\"font-bold\">"));
        assert!(html_output.contains("<br/>"));
        
        // Should have 5 days worth of HTML (Tuesday through Saturday)
        assert_eq!(html_output.matches("<div class=\"mb-4\">").count(), 5);
        assert_eq!(html_output.matches("</div>").count(), 5);
    }
}