// encapsulates the interals of the struct by not making them public
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl Default for AveragedCollection {
    fn default() -> Self {
        AveragedCollection::new()
    }
}
// users must interact with AveragedCollection via these methods
impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
