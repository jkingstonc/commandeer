

pub fn main(){


    let commandeer = commandeer::Commandeer{};

    commandeer.cmd(std::string::String::from("help"));
    commandeer.exec(std::string::String::from("my command!"));



}