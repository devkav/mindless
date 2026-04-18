use crate::objects::commit::create_commit;



pub fn save(message: String) {
    println!("Saving directory state with message: {}", message);
    create_commit(&message);
}
