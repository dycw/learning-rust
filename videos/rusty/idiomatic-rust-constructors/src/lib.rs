#[derive(Debug)]
pub enum Role {
    Guest,
    Viewer,
    Creator,
    Admin,
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub role: Role,
}
