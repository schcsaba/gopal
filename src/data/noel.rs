//! The Christmas take-away menu and bûches. Seasonal: delete this file,
//! `pages/noel.rs`, the `/noel` route and `NoelBanner` once the season is over.

/// The online shop where the menu and the bûches are ordered and paid for.
pub const SHOP_URL: &str = "https://gopal.sumupstore.com/";
/// The "Bûches de Noël" category of the shop, where both bûches are ordered.
pub const BUCHES_URL: &str =
    "https://gopal.sumupstore.com/produits?category=6d7bf43d-09cf-4adc-b5ed-461ace9602ec";
pub const PRICE: &str = "48 €";
pub const BUCHE_PRICE: &str = "38 €";
pub const BUCHE_SIZE: &str = "25 cm, pour 5 à 6 parts";
pub const ORDER_DEADLINE: &str = "16 décembre";
pub const PICKUP: &str = "jeudi 24 décembre, de 11h00 à 16h30";
/// The number given on the poster and the shop for Christmas orders.
pub const PHONE: &str = "+33768783285";
pub const PHONE_DISPLAY: &str = "07 68 78 32 85";

pub struct Dish {
    pub name: &'static str,
    pub description: &'static str,
    pub is_vegan: bool,
}

pub struct Course {
    pub title: &'static str,
    pub choices: [Dish; 2],
}

pub fn get_noel_courses() -> [Course; 4] {
    [
        Course {
            title: "Les amuse-bouches",
            choices: [
                Dish {
                    name: "Blinis au fromage frais végétal",
                    description: "faumon et aneth",
                    is_vegan: true,
                },
                Dish {
                    name: "Toast gourmand au pâté végétal",
                    description: "et confiture de figues",
                    is_vegan: true,
                },
            ],
        },
        Course {
            title: "Les entrées",
            choices: [
                Dish {
                    name: "Velouté de Noël",
                    description: "au potimarron rôti et aux châtaignes",
                    is_vegan: true,
                },
                Dish {
                    name: "Salade de pommes de terre crémeuse",
                    description: "à la new-yorkaise",
                    is_vegan: true,
                },
            ],
        },
        Course {
            title: "Les plats",
            choices: [
                Dish {
                    name: "Rôti de seitan farci",
                    description: "aux champignons, marrons et petits légumes, accompagné de sa sauce gourmande et légumes rôtis",
                    is_vegan: true,
                },
                Dish {
                    name: "Cannellonis aux trois fromages",
                    description: "légumes grillés, tomates séchées et fines herbes (contient des produits laitiers)",
                    is_vegan: false,
                },
            ],
        },
        Course {
            title: "Les desserts",
            choices: [
                Dish {
                    name: "Médovik aux pécans",
                    description: "le célèbre gâteau russe aux fines couches fondantes",
                    is_vegan: true,
                },
                Dish {
                    name: "Mousse au chocolat",
                    description: "accompagnée de son biscuit cranberry et pistache",
                    is_vegan: true,
                },
            ],
        },
    ]
}

pub struct Buche {
    pub name: &'static str,
    pub description: &'static str,
    pub is_vegan: bool,
    /// The bûche's page in the online shop.
    pub url: &'static str,
}

pub fn get_buches() -> [Buche; 2] {
    [
        Buche {
            name: "Bûche Ferrero",
            description: "Un brownie au chocolat, un croustillant au praliné, un insert crémeux au praliné et une délicieuse mousse au chocolat.",
            is_vegan: true,
            url: "https://gopal.sumupstore.com/product/buche-ferrero",
        },
        Buche {
            name: "Bûche Tropicale",
            description: "Un biscuit moelleux à la coco, un insert crémeux à la mangue et une mousse à la coco et vanille.",
            is_vegan: true,
            url: "https://gopal.sumupstore.com/product/buche-tropicale",
        },
    ]
}
