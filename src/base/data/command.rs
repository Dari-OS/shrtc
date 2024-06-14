pub struct Command {
    name: String,
    command: String,
    default_command: bool,
}


impl Command {
    pub fn new(name: &str, command: &str, is_default: bool) -> Command {
        Command {
            name: String::from(name),
            command: String::from(command),
            default_command: is_default
        }
    }

    pub fn get_name(&mut self) -> &mut String {
        return  &mut self.name;
    }

    pub fn get_command(&mut self) -> &mut String {
        return &mut self.name;
    }

    pub fn is_default(&mut self) -> &mut bool {
        return &mut self.default_command;
    }

    pub fn execute(&self, args: &[&str]) {
        for arg in args {
            
        }
    }

}