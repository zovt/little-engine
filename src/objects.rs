use transform::Transform;

#[derive(Hash)]
pub enum ObjectID {
	String(String),
	Num(u32),
}

pub struct Object {
	pub id: ObjectID,
}
