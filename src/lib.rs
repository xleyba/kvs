
pub struct KvStore{}

impl KvStore {
    pub fn new() -> KvStore {
    panic!()
    }

    pub fn set(&mut self, key: String, value: String) {}

    pub fn get(&mut self, key: String) -> Option<String> {
        panic!()
    }

    pub fn remove(&mut self, key: String) {
    }

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
