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
        match score.is_power_of_two() {
            true => { 
                match score.next_power_of_two() {
                    1 => { allergies.push(Allergen::Eggs); },
                    2 => { allergies.push(Allergen::Peanuts); },
                    4 => { allergies.push(Allergen::Shellfish); },
                    8 => { allergies.push(Allergen::Strawberries); },
                    16 => { allergies.push(Allergen::Tomatoes); },
                    32 => { allergies.push(Allergen::Chocolate); },
                    64 => { allergies.push(Allergen::Pollen); },
                    128 => { allergies.push(Allergen::Cats); },
                    _ => { continue },
                };
            },
            false => { 
                match score {
                    0 => { continue },
                    3 => { 
                        allergies.push(Allergen::Eggs);
                        allergies.push(Allergen::Peanuts);
                    },
                    5..=7 {
                        allergies.push(Allergen::Shellfish);
                        score -= 
                    }
                }
            }
        }

        Allergies { foods: allergies }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.foods.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.foods.clone()
    }
}

fn compute_allergies(score: u32, allergies: Vec<Allergen>) -> Vec<Allergen> {
    let prev_pow = previous_power_of_two(score);

    match prev_pow 
}

fn previous_power_of_two(score: u32) -> u32 {
    score.next_power_of_two() / 2
}