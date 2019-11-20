#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            scores: scores,
        }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        match self.scores.last() {
            Some(score) => { Some(*score) },
            None => { None },
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None
        }

        let mut best_score = self.scores.first().unwrap();
        let scores = self.scores.iter().skip(1);

        for score in scores {
            if score > best_score {
                best_score = score;
            }
        }

        Some(*best_score)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores.to_vec();
        scores.sort();
        scores.reverse();

        let mut top_three = Vec::new();

        for score in scores.iter().take(3) {
            top_three.push(*score);
        }

        top_three
    }
}
