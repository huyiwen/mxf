pub struct OrderService;

impl OrderService {
    pub fn init() -> Self {
        Self
    }

    pub fn lease(&self) {
        println!("lease");
    }
}
