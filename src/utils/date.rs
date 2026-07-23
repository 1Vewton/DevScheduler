// date module checks the date to help giving project of today
pub mod date {
    use chrono::Local;

    // get_date gets the date of today in String format
    pub fn get_date() -> String {
        let format = "%Y-%m-%d";
        Local::now().format(format).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    // Test the get_date function
    fn test_get_date(){
        let date1 = date::get_date();
        let date2 = date::get_date();
        assert_eq!(date1, date2);
    }
}