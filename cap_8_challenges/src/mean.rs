#[derive(Debug)]
pub struct Values {
    pub values: Vec<String>,
}

impl Values {
    fn convert(&self) -> Vec<f64> {
        self.values[2..].iter().map(|x| -> f64{x.parse().unwrap()}).collect()
    }
    pub fn mean(&self) -> f64 {
        let vec_float = self.convert();
        let vec_sum: f64 = vec_float.iter().sum();

        vec_sum / vec_float.len() as f64
        
    }
}
