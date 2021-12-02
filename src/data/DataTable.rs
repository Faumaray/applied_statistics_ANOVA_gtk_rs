use super::ResultVars::*;
use super::Support::p_value;
#[derive(Debug, Clone)]
pub struct DataTable
//основная структура struct == class
{
    pub groups: Vec<Variable>,
    pub count_of_groups_by_rows: Vec<usize>,    // по строкам
    pub count_of_groups_by_columns: Vec<usize>, // по столбцам
    pub sum_of_groups_by_rows: Vec<f64>,        // сумма по строкам
    pub sum_of_groups_by_columns: Vec<f64>,     // сумма по столбцам
    pub mean_of_groups_by_rows: Vec<f64>,       // среднее по строкам
    pub mean_of_groups_by_columns: Vec<f64>,    // среднее по столбцам
    pub dispersion_of_groups_by_rows: Vec<f64>, // дисперсия по строкам
    pub dispersion_of_groups_by_columns: Vec<f64>, // дисперсия по столбцам
}
impl DataTable {
    pub fn new(input: Vec<Variable>) -> Self {
        let mut count_by_columns: Vec<usize> = Vec::new();
        let mut sum_by_columns: Vec<f64> = Vec::new();
        let mut mean_by_columns: Vec<f64> = Vec::new();
        let mut dispersion_by_columns: Vec<f64> = Vec::new();
        for variable in &input {
            count_by_columns.push(variable.count);
            sum_by_columns.push(variable.sum);
            mean_by_columns.push(variable.mean);
            dispersion_by_columns.push(variable.dispersion);
        }

        let mut mean_by_rows: Vec<f64> = Vec::new();
        let mut dispersion_by_rows: Vec<f64> = Vec::new();
        let max = (&count_by_columns)
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let mut count_by_rows: Vec<usize> = vec![0; *max];
        let mut sum_by_rows: Vec<f64> = vec![0.0; *max];
        for i in 0..*max {
            let mut temp_count: usize = 0;
            let mut temp_sum: f64 = 0.0;
            for j in 0..input.len() {
                if i >= input[j].count {
                    continue;
                }
                temp_count += 1;
                temp_sum += input[j].data[i];
            }
            count_by_rows[i] = temp_count;
            sum_by_rows[i] = temp_sum;
        }

        for i in 0..sum_by_rows.len() {
            mean_by_rows.push(sum_by_rows[i] / count_by_rows[i] as f64);
        }
        let mut squares_of_deviations: Vec<Vec<f64>> = Vec::new();
        for i in 0..*max {
            let mut value: Vec<f64> = Vec::new();
            for j in 0..input.len() {
                if i >= input[j].count {
                    continue;
                }
                value.push((input[j].data[i] - mean_by_rows[i]).powi(2));
            }
            squares_of_deviations.push(value);
        }
        for value in squares_of_deviations {
            dispersion_by_rows.push(value.iter().sum::<f64>() / (input.len() - 1) as f64);
        }

        Self {
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
    pub fn print_matrix(&self)
    {
        for group in &self.groups
        {
            for value in &group.data
            {
                print!("{:.4}\t", value);
            }
            println!("");
        }
    }
    pub fn print_summary(&self)
    {
        println!("Итоги    \tСчет     \tСумма    \tСреднее  \tДисперсия");
        for i in 0..self.count_of_groups_by_rows.len()
        {
            println!("Строка {} \t{:.0}       \t{:.4}   \t{:.4}   \t{:.4}",i,self.count_of_groups_by_rows[i],self.sum_of_groups_by_rows[i],self.mean_of_groups_by_rows[i],self.dispersion_of_groups_by_rows[i]);
        }
        println!("");
        for i in 0..self.groups.len()
        {
            println!("Столбец {} \t{:.0}       \t{:.4}   \t{:.4}   \t{:.4}",i,self.count_of_groups_by_columns[i],self.sum_of_groups_by_columns[i],self.mean_of_groups_by_columns[i],self.dispersion_of_groups_by_columns[i]);
        }
    }
    pub fn two_way_without_reps(&self, alfa: f64) -> ResultTwoWayWithoutReps //многофакторный дисперсионный анализ
    {
        let sum_for_ss_columns: f64 = self.sum_of_groups_by_columns.iter().sum();
        let count_for_ss_columns: usize = self.count_of_groups_by_columns.iter().sum();
        let mean_for_ss_columns: f64 = sum_for_ss_columns / count_for_ss_columns as f64; //общая среднее
        let mut ss_columns = 0.0; //SSe
        for i in 0..self.groups.len() {
            ss_columns += self.count_of_groups_by_columns[i] as f64
                * (self.mean_of_groups_by_columns[i] - mean_for_ss_columns).powi(2);
        }

        let sum_for_ss_rows: f64 = self.sum_of_groups_by_rows.iter().sum();
        let count_for_ss_rows: usize = self.count_of_groups_by_rows.iter().sum();
        let mean_for_ss_rows: f64 = sum_for_ss_rows / count_for_ss_rows as f64; //общая среднее
        let mut ss_rows = 0.0; //SSe
        for i in 0..self.groups.len() {
            ss_rows += self.count_of_groups_by_rows[i] as f64
                * (self.mean_of_groups_by_rows[i] - mean_for_ss_rows).powi(2);
        }

        let mut ss_total = 0.0;
        let mut mean = 0.0;
        for group in &self.groups {
            for value in &group.data {
                mean += value;
            }
        }
        mean = mean
            / ((self.count_of_groups_by_columns.len() * self.count_of_groups_by_rows.len()) as f64);
        for group in &self.groups {
            for value in &group.data {
                ss_total += (value - mean).powi(2);
            }
        }
        let ss_error = ss_total - ss_columns - ss_rows;

        let ms_rows: f64 = ss_rows / ((self.count_of_groups_by_rows.len() - 1) as f64);
        let ms_columns = ss_columns / ((self.groups.len() - 1) as f64);

        let df_columns = ss_columns / ms_columns;
        let df_rows = ss_rows / ms_rows;
        let df_error = df_columns * df_rows;
        let df_total = df_rows + df_columns + df_error;

        let ms_error = ss_error / df_error;

        let f_rows = ms_rows / ms_error;
        let f_columns = ms_columns / ms_error;

        let p_rows = p_value(df_rows, df_error, f_rows);
        let p_columns = p_value(df_columns, df_error, f_columns);

        let ss = SSTwoWay {
            rows: ss_rows,
            cols: ss_columns,
            error: ss_error,
            sum: ss_total,
        };
        let df = DFTwoWay {
            rows: df_rows,
            cols: df_columns,
            error: df_error,
            sum: df_total,
        };
        let ms = MSTwoWay {
            rows: ms_rows,
            cols: ms_columns,
            error: ms_error,
        };
        let f = FTwoWay {
            rows: f_rows,
            cols: f_columns,
        };
        let p = PTwoWay {
            rows: p_rows,
            cols: p_columns,
        };
        return ResultTwoWayWithoutReps { ss, df, ms, f, p };
    }

    pub fn one_way(&self, alfa: f64, by_column: bool) -> ResultOneWay //однофакторный дисперсионный анализ
    {
        if by_column {
            let sum: f64 = self.sum_of_groups_by_columns.iter().sum();
            let count: usize = self.count_of_groups_by_columns.iter().sum();
            let mean: f64 = sum / count as f64; //общая среднее
            let mut ss_between = 0.0; //SSe
            for i in 0..self.groups.len() {
                ss_between += self.count_of_groups_by_columns[i] as f64
                    * (self.mean_of_groups_by_columns[i] - mean).powi(2);
            }
            //SSa
            let mut ss_inside = 0.0;
            for i in 0..self.groups.len() {
                let mut group_ss = 0.0;
                for j in 0..self.groups[i].data.len() {
                    group_ss +=
                        (self.groups[i].data[j] - self.mean_of_groups_by_columns[i]).powi(2);
                }
                ss_inside += group_ss;
            }

            let n: usize = self.count_of_groups_by_columns.iter().sum();
            let ms_between = ss_between / ((self.groups.len() - 1) as f64);
            let ms_inside = ss_inside / (n - self.groups.len()) as f64;

            let df_between = ss_between / ms_between;
            let df_inside = ss_inside / ms_inside;
            let f: f64 = ms_between / ms_inside;
            let p: f64 = p_value(df_between, df_inside, f);
            let df = DFOneWay {
                inside: df_inside,
                between: df_between,
                sum: df_inside + df_between,
            };
            let ms = MSOneWay {
                inside: ms_inside,
                between: ms_between,
            };
            let ss = SSOneWay {
                inside: ss_inside,
                between: ss_between,
                sum: ss_inside + ss_between,
            };
            return ResultOneWay { ss, df, ms, f, p };
        } else {
            let sum: f64 = self.sum_of_groups_by_rows.iter().sum();
            let count: usize = self.count_of_groups_by_rows.iter().sum();
            let mean: f64 = sum / count as f64; //общая среднее
            let mut ss_between = 0.0; //SSe
            for i in 0..self.groups.len() {
                ss_between += self.count_of_groups_by_rows[i] as f64
                    * (self.mean_of_groups_by_rows[i] - mean).powi(2);
            }
            //SSa
            let max = (&self.count_of_groups_by_columns)
                .into_iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let mut ss_inside = 0.0;
            for i in 0..*max {
                let mut group_ss = 0.0;
                for j in 0..self.groups.len() {
                    if i >= self.groups[j].count {
                        continue;
                    }
                    group_ss += (self.groups[j].data[i] - self.mean_of_groups_by_rows[i]).powi(2);
                }
                ss_inside += group_ss;
            }
            let n: usize = self.count_of_groups_by_columns.iter().sum();
            let ms_between: f64 = ss_between / ((self.count_of_groups_by_rows.len() - 1) as f64);
            let ms_inside: f64 = ss_inside / (n - self.count_of_groups_by_rows.len()) as f64;
            let df_between = ss_between / ms_between;
            let df_inside = ss_inside / ms_inside;
            let f: f64 = ms_between / ms_inside;
            let p: f64 = p_value(df_between, df_inside, f);
            let df = DFOneWay {
                inside: df_inside,
                between: df_between,
                sum: df_inside + df_between,
            };
            let ms = MSOneWay {
                inside: ms_inside,
                between: ms_between,
            };
            let ss = SSOneWay {
                inside: ss_inside,
                between: ss_between,
                sum: ss_inside + ss_between,
            };

            return ResultOneWay { ss, df, ms, f, p };
        }
    }
}
#[derive(Debug, Clone)]
pub struct Variable {
    pub data: Vec<f64>,
    pub count: usize,
    pub mean: f64,
    pub sum: f64,
    pub dispersion: f64,
}

impl Variable {
    pub fn new(input: Vec<f64>) -> Self {
        let sum: f64 = input.iter().sum();
        let mean: f64 = sum / input.len() as f64;
        let mut squares_of_deviations: Vec<f64> = Vec::new();
        for value in &input {
            squares_of_deviations.push((value - mean).powi(2));
        }
        Self {
            count: input.len(),
            dispersion: (squares_of_deviations.iter().sum::<f64>() / (input.len() - 1) as f64),
            data: input,
            sum: sum,
            mean: mean,
        }
    }
}
#[derive(Debug)]
pub struct ResultTwoWayWithoutReps {
    ss: SSTwoWay,
    df: DFTwoWay,
    ms: MSTwoWay,
    f: FTwoWay,
    p: PTwoWay, //f_crit: FCritTwoWay
}
impl std::fmt::Display for ResultTwoWayWithoutReps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Источники  \tSS         \tdf         \tMS         \tF          \tp-значение\nСтроки     \t{:.8}\t{:.0}        \t{:.8}\t{:.8}\t{:.8}\nСтолбцы    \t{:.8}\t{:.0}        \t{:.8}\t{:.8}\t{:.8}\nПогрешность\t{:.8}\t{:.0}        \t{:.8}\nИтого      \t{:.8}\t{:.0}", 
        self.ss.rows, self.df.rows,self.ms.rows,self.f.rows,self.p.rows,self.ss.cols, self.df.cols,self.ms.cols,self.f.cols,self.p.cols,
        self.ss.error,self.df.error,self.ms.error,self.ss.sum,self.df.sum)
    }
}
#[derive(Debug)]
pub struct ResultOneWay {
    ss: SSOneWay,
    df: DFOneWay,
    ms: MSOneWay,
    f: f64,
    p: f64,
}
impl std::fmt::Display for ResultOneWay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Источники  \tSS         \tdf         \tMS         \tF          \tp-значение\nМежду     \t{:.8}\t{:.0}        \t{:.8}\t{:.8}\t{:.8}\nВнутри    \t{:.8}\t{:.0}        \t{:.8}\nИтого      \t{:.8}\t{:.0}", 
        self.ss.inside, self.df.inside,self.ms.inside,self.f,self.p,self.ss.between, self.df.between,self.ms.between,
        self.ss.sum,self.df.sum)
    }
}
/*
Все структуры были составлены на основе результатов EXCEL, возможно нужны другие судя по "методичке"

*/
