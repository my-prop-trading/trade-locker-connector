use serde_derive::{Deserialize, Serialize};

#[derive(strum::Display, Debug, Clone, Serialize, Deserialize)]
pub enum AccountType {
    #[strum(to_string = "DEMO")]
    #[serde(rename = "DEMO")]
    Demo,
    #[strum(to_string = "LIVE")]
    #[serde(rename = "LIVE")]
    Live,
}

#[cfg(test)]
mod test {
    use crate::models::AccountType;

    #[test]
    pub fn account_type_live() {
        let account_type = AccountType::Live;
        
        assert_eq!(account_type.to_string(), "LIVE".to_string());
    }

    #[test]
    pub fn account_type_demo() {
        let account_type = AccountType::Demo;

        assert_eq!(account_type.to_string(), "DEMO".to_string());
    }
}