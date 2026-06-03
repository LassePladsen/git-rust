pub fn get_object_path(object_hash: &str) -> String {
    let dir = &object_hash[0..2];
    let filename = &object_hash[2..];
    format!(".git/objects/{dir}/{filename}")
}
