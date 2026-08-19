use crate::objects::commit::create_commit;

pub fn save(message: String) {
    create_commit(&message);
}
