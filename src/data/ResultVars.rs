#[derive(Debug)]
pub struct SSTwoWay{
    pub rows: f64,
    pub cols: f64,
    pub error: f64,
    pub sum: f64
}
#[derive(Debug)]
pub struct DFTwoWay{
    pub rows: f64,
    pub cols: f64,
    pub error: f64,
    pub sum: f64
}
#[derive(Debug)]
pub struct MSTwoWay{
    pub rows: f64,
    pub cols: f64,
    pub error: f64
}
#[derive(Debug)]
pub struct FTwoWay{
    pub rows: f64,
    pub cols: f64
}
#[derive(Debug)]
pub struct PTwoWay{
    pub rows: f64,
    pub cols: f64
}
#[derive(Debug)]
pub struct FCritTwoWay{
    pub rows: f64,
    pub cols: f64
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
