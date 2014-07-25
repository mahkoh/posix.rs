pub enum Global {
    Typedef(Typedef),
    Struct(Struct),
    Constant(Constant),
}

pub struct Typedef {
    pub name: String,
    pub dst: String,
}

pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub ty: String,
    pub public: bool,
}

pub struct Constant {
    pub name: String,
    pub ty: String,
    pub val: String,
}
