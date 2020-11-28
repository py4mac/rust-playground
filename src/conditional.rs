pub fn run() {
    let age = 18;
    let check_id: bool= true;
    let knows_person_of_age = true;

    if age >= 21 && check_id ||  knows_person_of_age{
        println!("Bartender: what whould you like to drink ?")
    }

    let is_age = if age >=21 {true} else {false};
    println!("{}", is_age);
}