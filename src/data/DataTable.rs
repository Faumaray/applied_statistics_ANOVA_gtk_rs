use super::ResultVars::*;
use super::Support::p_value;
#[derive(Debug,Clone)]
pub struct DataTable // Временная имплементация структуры 
{
    pub groups: Vec<Variable>,
    pub count_of_groups_by_rows: Vec<usize>,// по строкам
    pub count_of_groups_by_columns: Vec<usize>,// по столбцам
    pub sum_of_groups_by_rows: Vec<f64>,// сумма по строкам
    pub sum_of_groups_by_columns: Vec<f64>,// сумма по столбцам
    pub mean_of_groups_by_rows: Vec<f64>,// среднее по строкам
    pub mean_of_groups_by_columns: Vec<f64>,// среднее по столбцам
    pub dispersion_of_groups_by_rows: Vec<f64>,// дисперсия по строкам
    pub dispersion_of_groups_by_columns: Vec<f64>,// дисперсия по столбцам
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
#[derive(Debug,Clone)]
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
#[derive(Debug)]
pub struct ResultOneWay
{
    ss: SSOneWay,
    df: DFOneWay,
    ms: MSOneWay,
    f: f64,
    p: f64
}

pub fn two_way_without_reps(table: DataTable, alfa:f64) -> ResultTwoWayWithoutReps //многофакторный дисперсионный анализ
{
    /*
    MSa = SSa/(k-1)
    MSb = SSb/(j-1)
    MSab = SSab/(k-1)(j-1)
    MSw = SSw/N-(k*j)
    Fa= MSa/MSw
    Fb= MSb/MSw
    Fab= MSab/MSw
    SSAB(Interaction)= SSt-SSw-SSA-SSB
    */
    unimplemented!();
    
}

pub fn one_way(table: DataTable, alfa:f64, by_column: bool) -> ResultOneWay //однофакторный дисперсионный анализ
{
    if by_column
    {
        let sum: f64 = table.sum_of_groups_by_columns.iter().sum();
        let count: usize = table.count_of_groups_by_columns.iter().sum();
        let mean: f64 = sum/count as f64;//общая среднее
        let mut ss_between = 0.0;//SSe
        for i in 0..table.groups.len()
        {
            ss_between += table.count_of_groups_by_columns[i] as f64*(table.mean_of_groups_by_columns[i]-mean).powi(2);
        }
        //SSa
        let mut ss_inside = 0.0;
        for i in 0..table.groups.len()
        {
            let mut group_ss = 0.0;
            for j in 0..table.groups[i].data.len()
            {
                group_ss += (table.groups[i].data[j] - table.mean_of_groups_by_columns[i]).powi(2);
            }
            ss_inside+=group_ss;
        }
        
        let n: usize = table.count_of_groups_by_columns.iter().sum();
        let ms_between = ss_between/((table.groups.len()-1) as f64);
        let ms_inside = ss_inside/(n-table.groups.len()) as f64;

        let df_between = ss_between/ms_between;
        let df_inside = ss_inside/ms_inside;
        let f: f64 = ms_between/ms_inside;
        let p: f64 = p_value(df_between, df_inside, f);
        let df = DFOneWay{
            inside: df_inside,
            between: df_between,
            sum: df_inside+df_between
        };
        let ms = MSOneWay{
            inside: ms_inside,
            between: ms_between
        };
        let ss = SSOneWay{
            inside: ss_inside,
            between: ss_between,
            sum: ss_inside+ss_between
        };
        return ResultOneWay{
            ss,
            df,
            ms,
            f,
            p
        }
    }
    else
    {
        let sum: f64 = table.sum_of_groups_by_rows.iter().sum();
        let count: usize = table.count_of_groups_by_rows.iter().sum();
        let mean: f64 = sum/count as f64;//общая среднее
        let mut ss_between = 0.0;//SSe
        for i in 0..table.groups.len()
        {
            ss_between += table.count_of_groups_by_rows[i] as f64*(table.mean_of_groups_by_rows[i]-mean).powi(2);
        }
        //SSa
        let max = (&table.count_of_groups_by_columns).into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let mut ss_inside = 0.0;
        for j in 0..*max
        {
            let mut group_ss = 0.0;
            for k in 0..table.groups.len()
            {
                if j >= table.groups[k].count
                {
                    continue;
                }
                group_ss += (table.groups[k].data[j] - table.mean_of_groups_by_rows[j]).powi(2);
            }
            ss_inside+=group_ss;
        }
        let n: usize = table.count_of_groups_by_columns.iter().sum();
        let ms_between:f64 = ss_between/((table.count_of_groups_by_rows.len()-1) as f64);
        let ms_inside:f64 = ss_inside/(n-table.count_of_groups_by_rows.len()) as f64;
        let df_between = ss_between/ms_between;
        let df_inside = ss_inside/ms_inside;
        let f: f64 = ms_between/ms_inside;
        let p: f64 = p_value(df_between, df_inside, f);
        let df = DFOneWay{
            inside: df_inside,
            between: df_between,
            sum: df_inside+df_between
        };
        let ms = MSOneWay{
            inside: ms_inside,
            between: ms_between
        };
        let ss = SSOneWay{
            inside: ss_inside,
            between: ss_between,
            sum: ss_inside+ss_between
        };
        
        return ResultOneWay{
            ss,
            df,
            ms,
            f,
            p
        }
    }
}
/*
Все структуры были составлены на основе результатов EXCEL, возможно нужны другие судя по "методичке"

*/