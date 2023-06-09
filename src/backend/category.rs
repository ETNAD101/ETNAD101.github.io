use std::fmt;

#[derive(Clone, PartialEq)]
pub enum Category {
    Appetizer,
    Entree,
    Drink,
    Dessert,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Appetizer => write!(f, "Appetizer"),
            Category::Entree => write!(f, "Entree"),
            Category::Drink => write!(f, "Drink"),
            Category::Dessert => write!(f, "Dessert"),
        }
    }
}