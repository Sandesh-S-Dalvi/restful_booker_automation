/// Miscellaneous helpers.
pub fn random_suffix() -> String {
    uuid::Uuid::new_v4().to_string()[..8].to_string()
}
