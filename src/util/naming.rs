use uuid::Uuid;

static EMPTY_ID: &str = "00000000-0000-0000-0000-000000000000";

// pub fn uuid_to_label(id: &Uuid) -> String {
//     id.to_simple().to_string()
// }

pub fn label_to_uuid(id: &str) -> Result<Uuid, String> {
    match Uuid::parse_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err("Invalid uuid form".to_owned()),
    }
}

pub fn empty_uuid() -> Uuid {
    label_to_uuid(EMPTY_ID).unwrap()
}
