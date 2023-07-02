pub enum ErrorMode {
    Strict,
    Permissive
} 
pub struct NodeConfiguration {
    max_calculated_fee: u32,
    error_logging_mode: ErrorMode
}
