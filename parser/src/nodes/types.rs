
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Float32(f32),
    Float64(f64),
    String(String),
}
