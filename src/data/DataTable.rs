use super::ResultVars::*;
use super::Support::p_value;
pub struct DataTable // Временная имплементация структуры 
{
    pub groups: Vec<Variable>,
    pub count_of_groups_by_rows: Vec<usize>,
    pub count_of_groups_by_columns: Vec<usize>,
    pub sum_of_groups_by_rows: Vec<f64>,
    pub sum_of_groups_by_columns: Vec<f64>,
    pub mean_of_groups_by_rows: Vec<f64>,
    pub mean_of_groups_by_columns: Vec<f64>,
    pub dispersion_of_groups_by_rows: Vec<f64>,
    pub dispersion_of_groups_by_columns: Vec<f64>,
}
impl DataTable
{
    pub fn new(input: Vec<Variable>) -> Self
    {
        let mut count_by_columns: Vec<usize> = Vec::new();
        let mut sum_by_columns: Vec<f64> = Vec::new();
        let mut mean_by_columns: Vec<f64> = Vec::new();
        let mut dispersion_by_columns: Vec<f64> = Vec::new();
        for variable in &input
        {
            count_by_columns.push(variable.count);
            sum_by_columns.push(variable.sum);
            mean_by_columns.push(variable.mean);
            dispersion_by_columns.push(variable.dispersion);
        }


        
        let mut mean_by_rows: Vec<f64> = Vec::new();
        let mut dispersion_by_rows: Vec<f64> = Vec::new();
        let max = (&count_by_columns).into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let mut count_by_rows: Vec<usize> = vec![0; *max];
        let mut sum_by_rows: Vec<f64> = vec![0.0;*max];
            for j in 0..*max
            {
                let mut temp_count: usize = 0;
                let mut temp_sum: f64 = 0.0;
                for k in 0..input.len()
                {
                    if j >= input[k].count
                    {
                        continue;
                    }
                    temp_count +=1;
                    temp_sum += input[k].data[j];
                }
                count_by_rows[j] =temp_count;
                sum_by_rows[j] = temp_sum;
            }

        for i in 0..sum_by_rows.len()
        {
            mean_by_rows.push(sum_by_rows[i]/count_by_rows[i] as f64);
        }
        let mut squares_of_deviations: Vec<Vec<f64>> = Vec::new();
        for j in 0..*max
        {
            let mut value: Vec<f64> = Vec::new();
            for k in 0..input.len()
            {
                if j >= input[k].count
                {
                    value.push(0.0);
                    continue;
                }
                value.push((input[k].data[j]-mean_by_rows[j]).powi(2));
            }
            squares_of_deviations.push(value);
        }
        for value in squares_of_deviations
        {
            dispersion_by_rows.push(value.iter().sum::<f64>()/(input.len()-1) as f64);
        }
        
        Self{
            groups: input,
            count_of_groups_by_columns: count_by_columns,
            sum_of_groups_by_columns: sum_by_columns,
            mean_of_groups_by_columns: mean_by_columns,
            dispersion_of_groups_by_columns: dispersion_by_columns,
            count_of_groups_by_rows: count_by_rows,
            sum_of_groups_by_rows: sum_by_rows,
            mean_of_groups_by_rows: mean_by_rows,
            dispersion_of_groups_by_rows: dispersion_by_rows,
        }
    }
}
#[derive(Debug)]
pub struct Variable
{
    pub data: Vec<f64>,
    pub count: usize,
    pub mean: f64,
    pub sum: f64,
    pub dispersion: f64
}

impl Variable
{
    pub fn new(input: Vec<f64>) -> Self
    {
        let sum: f64 = input.iter().sum();
        let mean: f64 = sum/input.len() as f64;
        let mut squares_of_deviations: Vec<f64> = Vec::new();
        for value in &input
        {
            squares_of_deviations.push((value-mean).powi(2));
        }
        Self{
            count: input.len(),
            dispersion: (squares_of_deviations.iter().sum::<f64>()/(input.len()-1) as f64),
            data: input,   
            sum: sum,
            mean: mean
        }
    }
}

pub struct ResultTwoWayWithoutReps
{
    ss: SSTwoWay,
    df: DFTwoWay,
    ms: MSTwoWay,
    f: FTwoWay,
    p: PTwoWay,
    f_crit: FCritTwoWay
}
pub struct ResultOneWay
{
    ss: SSOneWay,
    df: DFOneWay,
    ms: MSOneWay,
    f: FOneWay,
    p: f64
}

pub fn two_way_without_reps(table: DataTable, alfa:f64) -> ResultTwoWayWithoutReps
{
    unimplemented!();
}

pub fn one_way(table: DataTable, alfa:f64) -> ResultOneWay
{
    unimplemented!();
}
/*
Все структуры были составлены на основе результатов EXCEL, возможно нужны другие судя по "методичке"

*/