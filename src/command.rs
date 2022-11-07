pub struct Command {
    pub name: String,
    pub arguments: Vec<String>,
    pub hidden_arguments: Vec<String>,
}
impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Command {{ name:{}, arguments: [{}], hidden_arguments:[{}] }}", self.name, self.arguments.join(", "), self.hidden_arguments.join(", "))
    }
}