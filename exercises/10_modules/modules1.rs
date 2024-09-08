// TODO: Fix the compiler error about calling a private function.
mod sausage_factory {
    // Don't let anybody outside of this module see this!
    pub fn make_sausage() {
        let recipe: String = get_secret_recipe();
        println!("Recipe: {}", recipe);
        println!("sausage!");
    }
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }
}

fn main() {
    sausage_factory::make_sausage();
}
