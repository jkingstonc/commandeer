

pub fn main(){


    let mut commandeer = commandeer::Commandeer{cmds: vec!()};
    commandeer.cmd(
        std::string::String::from("help"),
        std::string::String::from("press h for help"));
    commandeer.exec(std::string::String::from("help"));



}