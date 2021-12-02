pub struct SSTwoWay{
    rows: f64,
    cols: f64,
    error: f64,
    sum: f64
}
pub struct DFTwoWay{
    rows: usize,
    cols: usize,
    error: usize,
    sum: usize
}
pub struct MSTwoWay{
    rows: f64,
    cols: f64,
    error: f64
}
pub struct FTwoWay{
    rows: f64,
    cols: f64
}
pub struct PTwoWay{
    rows: f64,
    cols: f64
}
pub struct FCritTwoWay{
    rows: f64,
    cols: f64
}
#[derive(Debug)]
pub struct SSOneWay{
    pub inside: f64,
    pub between: f64,
    pub sum: f64
}
#[derive(Debug)]
pub struct DFOneWay{
    pub inside: f64,
    pub between: f64,
    pub sum: f64
}
#[derive(Debug)]
pub struct MSOneWay{
    pub inside: f64,
    pub between: f64
}
