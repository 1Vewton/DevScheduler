// random_tools contain tools for random generation related
mod random_tools {

    // WeightsData is a data structure that stores the temporary data of weights during the construction of table
    struct WeightsData{
        idx: i64,
        weight: f64,
    }

    #[derive(Clone)]
    // WAM is a data structure that can select random idx weightedly from a given list
    pub struct WAM {
        weights: Vec<f64>,
        prob: Vec<f64>,
        alias: Vec<i64>,
    }

    // new_wam creates a new WAM data struct
    pub fn new_wam(
        weights: Vec<f64>,
    ) -> WAM {
        return WAM{
            weights: weights.clone(),
            prob: Vec::new(),
            alias: Vec::new(),
        }.clone();
    }

    impl WAM {
        // construct_table constructs a table for random generation
        pub fn construct_table(&mut self) {
            let mut new_possibility: Vec<WeightsData> = Vec::new();
            for (idx, num) in (*&self.weights).iter().enumerate() {
                let length = *&self.weights.len() as f64;
                new_possibility.push(
                    WeightsData{
                        idx: idx as i64,
                        weight: *num*length,
                    }
                );
            }
            return;
        }
    }
}
