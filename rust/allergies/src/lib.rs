#[derive(Clone)]
pub struct Allergies {
    foods: Vec<Allergen>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self { 
        Allergies { foods: Vec::new() }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.foods.clone()
    }
}
