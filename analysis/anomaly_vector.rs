pub struct AnomalyVector;

impl AnomalyVector {
    pub fn score(events: &Vec<String>) -> f64 {
        let mut score = 0.0;

        for e in events {
            if e.contains("exec") {
                score += 0.2;
            }
            if e.contains("net") {
                score += 0.1;
            }
        }

        if score > 1.0 {
            1.0
        } else {
            score
        }
    }
}
