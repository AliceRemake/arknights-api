use entity::Operator;

pub struct Query;

impl Query {
    pub async fn find_by_id(_id: i32) -> Option<Operator> {
        None
    }

    pub async fn find_by_name(_name: String) -> Option<Operator> {
        None
    }
}