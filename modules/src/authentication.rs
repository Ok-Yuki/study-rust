pub struct User {
    username: String,
    password_hash: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(&password.to_owned()),
        }
    }
    pub fn get_username(&self) -> &String {
        &self.username
    }
    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(&new_password.to_owned())
    }
}
fn hash_password(input: &String) -> String {
    use crypto::sha2::Sha256;
    use crypto::digest::Digest;

    let mut sha256 = Sha256::new();
    sha256.input_str(&input);
    sha256.result_str()
}