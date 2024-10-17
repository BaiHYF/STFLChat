
pub const fn generate_welcome_message() -> &'static str {
    concat!(
        r"
Welcome to 
  ______   _________  _____       ______  ____  ____       _     _________  
.' ____ \ |  _   _  ||_   _|    .' ___  ||_   ||   _|     / \   |  _   _  | 
| (___ \_||_/ | | \_|  | |     / .'   \_|  | |__| |      / _ \  |_/ | | \_| 
 _.____`.     | |      | |   _ | |         |  __  |     / ___ \     | |     
| \____) |   _| |_    _| |__/ |\ `.___.'\ _| |  | |_  _/ /   \ \_  _| |_    
 \______.'  |_____|  |________| `.____ .'|____||____||____| |____||_____| [0mv", env!("CARGO_PKG_VERSION"), 
"\n                                                                          By Baihyf (sysu-netid: 22331049)\n",
        "\n",
        "\x1b[34mA Simple Toy for Lab CHAT\x1b[0m\n",
    )
}

pub const fn generate_input_username_message() -> &'static str {
    concat!(
        "Firstly, you will need an username\n",
        "Please input your username: "
    )
}

pub fn generate_help_message() -> &'static str {
    concat!(
        "Help: \n\n",
        "  /help                        Show this message\n",
        "  /users                       List all online users\n",
        "  /exit                        Quit the program\n",
        "  /send <username> <message>   Send a message to the user\n",
    )
}

#[cfg(test)]
mod test_texts {
    use super::*;
    #[test]
    fn test_generate_welcome_message() {
        println!("{}", generate_welcome_message());
    }
}