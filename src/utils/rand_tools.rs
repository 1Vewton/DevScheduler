// random_tools contain tools for random generation related
mod random_tools {
    use std::collections::VecDeque;

    #[derive(Clone)]
    // WeightsData is a data structure that stores the temporary data of weights during the construction of table
    struct WeightsData{
        idx: i64,
        weight: f64,
        alias: i64,
        prob: f64,
    }

    #[derive(Clone)]
    // WAM is a data structure that can select random idx weightedly from a given list
    pub struct WAM {
        weights: Vec<f64>,
        result: VecDeque<WeightsData>,
    }

    // new_wam creates a new WAM data struct
    pub fn new_wam(
        weights: Vec<f64>,
    ) -> WAM {
        // Check if the sum is equal to 1.0
        let mut tmp_weights: Vec<f64> = weights.clone();
        let mut sum: f64 = 0.0;
        for i in tmp_weights {
            sum += i;
        }
        if sum != 1.0{
            panic!("The total sum of the weights is not 1.0!")
        }
        // Return WAM
        return WAM{
            weights: weights.clone(),
            // The probability of selecting the original weight
            result: VecDeque::new()
        }.clone();
    }

    impl WAM {
        // construct_table constructs a table for random generation
        pub fn construct_table(&mut self) {
            let mut new_possibility: Vec<WeightsData> = Vec::new();
            let length = *&self.weights.len() as f64;
            for (idx, num) in (*&self.weights).iter().enumerate() {
                new_possibility.push(
                    WeightsData{
                        idx: idx as i64,
                        weight: *num*length,
                        alias: idx as i64,
                        prob: 1.0,
                    }
                );
            }
            let mut l: VecDeque<WeightsData> = VecDeque::new();
            let mut g: VecDeque<WeightsData> = VecDeque::new();
            let mut e: VecDeque<WeightsData> = VecDeque::new();
            for i in new_possibility {
                if i.weight == 1.0{
                    e.push_back(i)
                }else if i.weight < 1.0 {
                    l.push_back(i);
                }else if i.weight > 1.0 {
                    g.push_back(i);
                }
            }
            while !l.is_empty() && !g.is_empty() {
                let mut lesser: WeightsData;
                let mut greater: WeightsData;
                let mut lesser_diff: f64;
                if let Some(value) = l.pop_front(){
                    lesser = value;
                    lesser_diff = 1.0 - lesser.weight
                }else{
                    panic!("The l is empty but g is not empty");
                }
                if let Some(value) = g.pop_front(){
                    greater = value;
                }else {
                    panic!("The g is empty but l is not empty");
                }
                greater.weight -= lesser_diff;
                lesser.prob = lesser.weight;
                lesser.weight = 1.0;
                lesser.alias = greater.idx;
                e.push_back(lesser);
                if greater.weight > 1.0 {
                    g.push_back(greater);
                }else if greater.weight == 1.0{
                    e.push_back(greater);
                }else{
                    l.push_back(greater);
                }
            }
            for i in g {
                let mut ni: WeightsData = i.clone();
                ni.prob = 1.0;
                ni.alias = ni.idx;
                e.push_back(ni);
            }
            for i in l {
                let mut ni: WeightsData = i.clone();
                ni.prob = 1.0;
                ni.alias = ni.idx;
                e.push_back(ni);
            }
            self.result = e.clone();
            return;
        }
    }
}
