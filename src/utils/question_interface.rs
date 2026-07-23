// question_interface provides the interface for asking
pub mod question_interface {
    use std::io;

    // yes_or_no_question gets the input of the question in boolean form
    pub fn yes_or_no_question(
        default_choice: bool,
        tip: String,
    ) -> bool {
        let choice_tip: String;
        if default_choice {
            choice_tip = String::from("Y/n");
        }else{
            choice_tip = String::from("n/Y");
        }
        println!("{} ({})", tip, choice_tip);
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Failed to get response");
        if response.to_lowercase().trim() == "y" {
            true
        }else if response.to_lowercase().trim() == "n" {
            false
        }else{
            default_choice
        }
    }
}
