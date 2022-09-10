pub trait CommandPart {
    fn ingest(&self, cmd: &std::string::String);
}

pub struct PositionalCommandPart{
    cmd: std::string::String,
    action: Box<dyn Fn()>
}

impl core::fmt::Debug for PositionalCommandPart {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PositionalCommandPart")
         .field("cmd", &self.cmd)
         .field("action", &"<action>")
         .finish()
    }
}

impl CommandPart for PositionalCommandPart {
    fn ingest(&self, cmd: &std::string::String){
        if cmd.eq(&self.cmd){
            (self.action)();
        }
    }
}

pub struct Command{
    pub parts: std::vec::Vec<Box<dyn CommandPart>>
}

impl Command {
    pub fn exec(&self, cmd: &std::string::String) -> Result<(), ()>{
        for (_i, e) in self.parts.iter().enumerate() {
            e.ingest(&cmd)
        }
        return Ok(());
    }
}

pub struct Commandeer{
    // todo ideally we want a tree of command parts?
    pub cmds: std::vec::Vec<Command>,
    pub unknown: std::string::String
}

impl Commandeer{
    
    pub fn cmd(&mut self, cmd: std::string::String, action: Box<dyn Fn()>){
        self.cmds.push(Command{
            parts: vec!(
                Box::new(PositionalCommandPart{cmd: cmd, action: action})
            )
        });
    }

    pub fn unknown(&mut self, unknown: std::string::String){
        self.unknown = unknown;
    }

    pub fn exec(&mut self, cmd: std::string::String){
        let mut foundMatch = false;
        for command in self.cmds.iter(){
            match command.exec(&cmd) {
                Ok(_) => foundMatch = true,
                Err(_) => todo!()
            }

        }
        if !foundMatch {
            println!("{}", self.unknown);
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
