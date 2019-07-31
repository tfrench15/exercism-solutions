#[derive(Clone)]
pub struct Allergies {
    cause: Vec<Allergen>,
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
        if score == 0 {
            return Allergies { cause: Vec::new() }
        } else {
            match score.is_power_of_two() {
                true => { 
                    let allergy = compute_allergies_for_power_of_two(score);
                    Allergies { cause: vec![allergy] }
                },
                false => { 
                    let causes = compute_allergies(score);
                    Allergies { cause: causes }
                },
            }
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.cause.contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        self.cause.clone()
    }
}

fn compute_allergies(mut score: u32) -> Vec<Allergen> {
    let mut allergies = Vec::new();
    while !score.is_power_of_two() {
        let prev_pow = previous_power_of_two(score);
        let new_allergy = compute_allergies_for_power_of_two(prev_pow);
        allergies.push(new_allergy);
        score = score % prev_pow;
    }
    let final_allergy = compute_allergies_for_power_of_two(score);
    allergies.push(final_allergy);
    
    allergies
}

fn compute_allergies_for_power_of_two(score: u32) -> Allergen {
    match score {
        1 => { Allergen::Eggs },
        2 => { Allergen::Peanuts },
        4 => { Allergen::Shellfish },
        8 => { Allergen::Strawberries },
        16 => { Allergen::Tomatoes },
        32 => { Allergen::Chocolate },
        64 => { Allergen::Pollen },
        128 => { Allergen::Cats },
        _ => { compute_allergies_for_power_of_two(score/2) },
    }
}

fn previous_power_of_two(score: u32) -> u32 {
    score.next_power_of_two() / 2
}