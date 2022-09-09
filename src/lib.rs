pub struct Commandeer{

}

impl Commandeer{

    pub fn cmd(&self, cmd: std::string::String){
        println!("registering command `{}`.", cmd);
    }

    pub fn exec(&self, cmd: std::string::String){
        println!("parsing `{}`.", cmd);
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
