pub enum CollectionPeriod {
    Minutes, 
    Hours,
    Days,
    Weeks,
    Months,
    Years
}

impl CollectionPeriod {
    pub fn get_value(&self) -> i64 {
        match self {
            Self::Minutes => {
                return 60
            },

            Self::Hours => {
                return 3600
            },

            Self::Days => {
                return 86400
            },

            Self::Weeks => {
                return 86400 * 4
            },

            Self::Months => {
                return 86400*30
            },

            Self::Years => {
                return 86400*365
            }
        }
    }
}