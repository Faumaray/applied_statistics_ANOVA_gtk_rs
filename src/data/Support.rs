use std::result::Result;
use std::io::Error;
const GAMMA_DK: &[f64] = &[
    2.48574089138753565546e-5,
    1.05142378581721974210,
    -3.45687097222016235469,
    4.51227709466894823700,
    -2.98285225323576655721,
    1.05639711577126713077,
    -1.95428773191645869583e-1,
    1.70970543404441224307e-2,
    -5.71926117404305781283e-4,
    4.63399473359905636708e-6,
    -2.71994908488607703910e-9,
];

const LN_PI: f64 = 1.1447298858494001741434273513530587116472948129153;
const LN_2_SQRT_E_OVER_PI: f64 = 0.6207822376352452223455184457816472122518527279025978;
const F64_PREC: f64 = 0.00000000000000011102230246251565;
const GAMMA_R: f64 = 10.900511;
pub fn p_value(freedom_1: f64, freedom_2: f64, x: f64) -> f64 {
    if x < 0.0 {
        0.0
    } else if x.is_infinite() {
        1.0
    } else {
        1.0-checked_beta_reg(
            freedom_1 / 2.0,
            freedom_2 / 2.0,
            freedom_1 * x / (freedom_1 * x + freedom_2),
        ).unwrap()
    }
}
fn checked_beta_reg(a: f64, b: f64, x: f64) -> Result<f64, Error> {
    if a <= 0.0 {
        Err(Error::new(std::io::ErrorKind::InvalidInput, "a must be positive")) // Попра
    } else if b <= 0.0 {
        Err(Error::new(std::io::ErrorKind::InvalidInput, "b must be positive"))
    } else if !(0.0..=1.0).contains(&x) {
        Err(Error::new(std::io::ErrorKind::InvalidInput, "x must be >=0.0 and <=1.0"))
    } else {
        let bt = if x==0.0 || x==1.0 {
            0.0
        } else {
            (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b)
                + a * x.ln()
                + b * (1.0 - x).ln())
            .exp()
        };
        let symm_transform = x >= (a + 1.0) / (a + b + 2.0);
        let eps = F64_PREC;
        let fpmin = f64::MIN_POSITIVE / eps;

        let mut a = a;
        let mut b = b;
        let mut x = x;
        if symm_transform {
            let swap = a;
            x = 1.0 - x;
            a = b;
            b = swap;
        }

        let qab = a + b;
        let qap = a + 1.0;
        let qam = a - 1.0;
        let mut c = 1.0;
        let mut d = 1.0 - qab * x / qap;

        if d.abs() < fpmin {
            d = fpmin;
        }
        d = 1.0 / d;
        let mut h = d;

        for m in 1..141 {
            let m = f64::from(m);
            let m2 = m * 2.0;
            let mut aa = m * (b - m) * x / ((qam + m2) * (a + m2));
            d = 1.0 + aa * d;

            if d.abs() < fpmin {
                d = fpmin;
            }

            c = 1.0 + aa / c;
            if c.abs() < fpmin {
                c = fpmin;
            }

            d = 1.0 / d;
            h = h * d * c;
            aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
            d = 1.0 + aa * d;

            if d.abs() < fpmin {
                d = fpmin;
            }

            c = 1.0 + aa / c;

            if c.abs() < fpmin {
                c = fpmin;
            }

            d = 1.0 / d;
            let del = d * c;
            h *= del;

            if (del - 1.0).abs() <= eps {
                return if symm_transform {
                    Ok(1.0 - bt * h / a)
                } else {
                    Ok(bt * h / a)
                };
            }
        }

        if symm_transform {
            Ok(1.0 - bt * h / a)
        } else {
            Ok(bt * h / a)
        }
    }
}

fn ln_gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        LN_PI
            - (std::f64::consts::PI * x).sin().ln()
            - s.ln()
            - LN_2_SQRT_E_OVER_PI
            - (0.5 - x) * ((0.5 - x + GAMMA_R) / std::f64::consts::E).ln()
    } else {
        let s = GAMMA_DK
            .iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s.ln()
            + LN_2_SQRT_E_OVER_PI
            + (x - 0.5) * ((x - 0.5 + GAMMA_R) / std::f64::consts::E).ln()
    }
}