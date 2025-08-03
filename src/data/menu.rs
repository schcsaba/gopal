use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MenuItem {
    pub name: String,
    pub price: f32,
    pub description: String,
    pub category: MenuCategory,
    pub is_vegan: bool,
    pub is_vegetarian: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum MenuCategory {
    Formule,
    Dessert,
    Boisson,
}

impl MenuItem {
    pub fn new(
        name: &str,
        price: f32,
        description: &str,
        category: MenuCategory,
        is_vegan: bool,
        is_vegetarian: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            price,
            description: description.to_string(),
            category,
            is_vegan,
            is_vegetarian,
        }
    }

    pub fn format_price(&self) -> String {
        format!("{:.2} €", self.price).replace(".", ",")
    }

    pub fn is_diet_compatible(&self, vegan_only: bool) -> bool {
        if vegan_only {
            self.is_vegan
        } else {
            self.is_vegetarian || self.is_vegan
        }
    }
}

pub fn get_menu_items() -> Vec<MenuItem> {
    vec![
        // Formules
        MenuItem::new(
            "Assiette Gourmande",
            9.00,
            "samosa aux légumes + salade composée",
            MenuCategory::Formule,
            true,
            true,
        ),
        MenuItem::new(
            "Plat du jour",
            13.00,
            "riz parfumé + curry ou boulettes de légumes ou soupe (au choix) + galette papadam",
            MenuCategory::Formule,
            true,
            true,
        ),
        MenuItem::new(
            "Menu « Gopal »",
            16.00,
            "salade composée + riz parfumé + curry + boulettes de légumes + soupe + galette papadam",
            MenuCategory::Formule,
            true,
            true,
        ),
        MenuItem::new(
            "Spéciale du chef",
            14.00,
            "lasagne royale, calzoni, fougasse, etc... + salade composée",
            MenuCategory::Formule,
            false,
            true,
        ),
        // Desserts
        MenuItem::new(
            "Cheesecake",
            6.50,
            "caramel/chocolat, mangue ou fruits rouges...",
            MenuCategory::Dessert,
            false,
            true,
        ),
        MenuItem::new(
            "Cupcake",
            5.80,
            "ex : chocolat, crème de marron, framboise...",
            MenuCategory::Dessert,
            false,
            true,
        ),
        MenuItem::new(
            "Crumble",
            6.50,
            "pomme cannelle",
            MenuCategory::Dessert,
            true,
            true,
        ),
        MenuItem::new(
            "Mousse choco vegan",
            5.80,
            "",
            MenuCategory::Dessert,
            true,
            true,
        ),
        MenuItem::new(
            "Pannacotta vegan",
            5.80,
            "",
            MenuCategory::Dessert,
            true,
            true,
        ),
        MenuItem::new(
            "Gâteau du chef vegan",
            6.50,
            "la part",
            MenuCategory::Dessert,
            true,
            true,
        ),
        MenuItem::new(
            "Halava",
            5.80,
            "gâteau semoule",
            MenuCategory::Dessert,
            true,
            true,
        ),
        MenuItem::new(
            "Tarte au citron vegan",
            6.50,
            "la part",
            MenuCategory::Dessert,
            true,
            true,
        ),
        // Boissons
        MenuItem::new(
            "Lassi",
            4.00,
            "au yaourt vegan, parfum mangue ou rose 25 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Limonade maison",
            3.50,
            "menthe fraîche, citron vert, gingembre 25 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "San Pellegrino aromatisée",
            2.00,
            "33 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Bière sans alcool",
            3.00,
            "25 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Tisane",
            2.00,
            "20 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Eau pétillante",
            2.00,
            "33 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Eau plate",
            2.00,
            "50 cl",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Café expresso",
            2.50,
            "",
            MenuCategory::Boisson,
            true,
            true,
        ),
        MenuItem::new(
            "Double expresso",
            3.00,
            "",
            MenuCategory::Boisson,
            true,
            true,
        ),
    ]
}

pub fn filter_by_category<'a>(items: &'a [MenuItem], category: &MenuCategory) -> Vec<&'a MenuItem> {
    items.iter().filter(|item| &item.category == category).collect()
}

pub fn calculate_total_price(items: &[MenuItem]) -> f32 {
    items.iter().map(|item| item.price).sum()
}

pub fn get_formules() -> Vec<MenuItem> {
    let items = get_menu_items();
    filter_by_category(&items, &MenuCategory::Formule).into_iter().cloned().collect()
}

pub fn get_desserts() -> Vec<MenuItem> {
    let items = get_menu_items();
    filter_by_category(&items, &MenuCategory::Dessert).into_iter().cloned().collect()
}

pub fn get_boissons() -> Vec<MenuItem> {
    let items = get_menu_items();
    filter_by_category(&items, &MenuCategory::Boisson).into_iter().cloned().collect()
}