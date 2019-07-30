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
        let mut allergies = Vec::new();
        if score.is_power_of_two() {
            allergies.push(score.next_power_of_two());
        } else {
            let upper_bound = score.next_power_of_two();
            
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        false
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.foods.clone()
    }
}
