pub struct Command{
    pub cmd: std::string::String,
    pub action: std::string::String
}

pub struct Commandeer{
    pub cmds: std::vec::Vec<Command>
}

impl Commandeer{
    
    pub fn cmd(&mut self, cmd: std::string::String, action: std::string::String){
        println!("registering command `{}`.", cmd);
        self.cmds.push(Command{cmd: cmd, action: action});
    }

    pub fn exec(&mut self, cmd: std::string::String){
        println!("executing `{}`.", cmd);

        for val in self.cmds.iter(){
            if val.cmd.eq(&cmd){
                println!("{}", val.action);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
