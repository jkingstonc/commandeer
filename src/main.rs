

pub fn main(){


    let mut commandeer = commandeer::Commandeer{cmds: vec!(), unknown: std::string::String::from("")};
    commandeer.cmd(
        std::string::String::from("help"),
        Box::new(||println!("press help for help!",)));

    commandeer.unknown(std::string::String::from("unknown command, press help for help"));

    commandeer.exec(std::string::String::from("help"));



}