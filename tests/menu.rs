// Tests for the actual menu data module from src/data/menu.rs
use gopal::data::menu::{get_menu_items, filter_by_category, calculate_total_price, get_formules, get_desserts, get_boissons, MenuItem, MenuCategory};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem::new(
            "Test Item",
            12.50,
            "Test description",
            MenuCategory::Formule,
            true,
            true,
        );

        assert_eq!(item.name, "Test Item");
        assert_eq!(item.price, 12.50);
        assert_eq!(item.description, "Test description");
        assert_eq!(item.category, MenuCategory::Formule);
        assert!(item.is_vegan);
        assert!(item.is_vegetarian);
    }

    #[test]
    fn test_price_formatting() {
        let item = MenuItem::new("Test", 12.50, "Description", MenuCategory::Formule, true, true);
        assert_eq!(item.format_price(), "12,50 €");

        let item2 = MenuItem::new("Test", 9.00, "Description", MenuCategory::Formule, true, true);
        assert_eq!(item2.format_price(), "9,00 €");
    }

    #[test]
    fn test_diet_compatibility() {
        let vegan_item = MenuItem::new("Vegan", 10.0, "Vegan dish", MenuCategory::Formule, true, true);
        let vegetarian_item = MenuItem::new("Veggie", 10.0, "Vegetarian dish", MenuCategory::Formule, false, true);

        // Vegan diet should only accept vegan items
        assert!(vegan_item.is_diet_compatible(true));
        assert!(!vegetarian_item.is_diet_compatible(true));

        // Vegetarian diet should accept both vegan and vegetarian items
        assert!(vegan_item.is_diet_compatible(false));
        assert!(vegetarian_item.is_diet_compatible(false));
    }

    #[test]
    fn test_get_menu_items_from_src() {
        let items = get_menu_items();
        assert_eq!(items.len(), 21);

        // Check that we have items from different categories
        let formules = filter_by_category(&items, &MenuCategory::Formule);
        let desserts = filter_by_category(&items, &MenuCategory::Dessert);
        let boissons = filter_by_category(&items, &MenuCategory::Boisson);

        assert_eq!(formules.len(), 4);
        assert_eq!(desserts.len(), 8);
        assert_eq!(boissons.len(), 9);
    }

    #[test]
    fn test_get_formules_from_src() {
        let formules = get_formules();
        assert_eq!(formules.len(), 4);
        assert!(formules.iter().all(|item| item.category == MenuCategory::Formule));
    }

    #[test]
    fn test_get_desserts_from_src() {
        let desserts = get_desserts();
        assert_eq!(desserts.len(), 8);
        assert!(desserts.iter().all(|item| item.category == MenuCategory::Dessert));
    }

    #[test]
    fn test_get_boissons_from_src() {
        let boissons = get_boissons();
        assert_eq!(boissons.len(), 9);
        assert!(boissons.iter().all(|item| item.category == MenuCategory::Boisson));
    }

    #[test]
    fn test_filter_by_category() {
        let items = get_menu_items();
        let formules = filter_by_category(&items, &MenuCategory::Formule);

        assert!(formules.iter().all(|item| item.category == MenuCategory::Formule));
        assert!(formules.len() > 0);
    }

    #[test]
    fn test_calculate_total_price() {
        let items = vec![
            MenuItem::new("Item 1", 10.0, "Description", MenuCategory::Formule, true, true),
            MenuItem::new("Item 2", 15.5, "Description", MenuCategory::Dessert, true, true),
            MenuItem::new("Item 3", 8.25, "Description", MenuCategory::Formule, true, true),
        ];

        let total = calculate_total_price(&items);
        assert_eq!(total, 33.75);
    }

    #[test]
    fn test_menu_item_equality() {
        let item1 = MenuItem::new("Test", 10.0, "Description", MenuCategory::Formule, true, true);
        let item2 = MenuItem::new("Test", 10.0, "Description", MenuCategory::Formule, true, true);
        let item3 = MenuItem::new("Different", 10.0, "Description", MenuCategory::Formule, true, true);

        assert_eq!(item1, item2);
        assert_ne!(item1, item3);
    }

    #[test]
    fn test_all_items_are_vegetarian_or_vegan() {
        let items = get_menu_items();
        assert!(items.iter().all(|item| item.is_vegetarian || item.is_vegan));
    }

    #[test]
    fn test_price_ranges() {
        let items = get_menu_items();
        
        // Check that formule prices are within expected range
        let formules = filter_by_category(&items, &MenuCategory::Formule);
        assert!(formules.iter().all(|item| item.price >= 9.0 && item.price <= 20.0));

        // Check that dessert prices are within expected range
        let desserts = filter_by_category(&items, &MenuCategory::Dessert);
        assert!(desserts.iter().all(|item| item.price >= 5.0 && item.price <= 10.0));
    }
}