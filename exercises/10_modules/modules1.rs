mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // public 함수를 이용해 접근
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
