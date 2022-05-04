pub struct SharedBool(bool);

impl SharedBool {
    pub fn new(state: bool) -> Self {
        Self(state)
    }

    pub fn get_value(&self) -> bool {
        return self.0;
    }
}
