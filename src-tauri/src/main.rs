// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use log::warn;
use std::arch::x86_64::_CMP_EQ_OS;
use tauri_plugin_log::LogTarget;

#[tauri::command]
async fn calculate(vars: Vec<String>, symbs: Vec<char>) -> String {
    let mut equation_vector: Vec<Equation> = Vec::new();
    let vars_f64: Vec<f64> = get_clean_vars_f64(vars);
    for (idx, symb) in symbs.iter().enumerate() {
        let order: u8;
        match symb {
            '/' => order = 2,
            '*' => order = 2,
            '+' => order = 1,
            '-' => order = 1,
            _ => return String::from("something has gone wrong with parsing the equation symbols"),
        }
        let eq = Equation::init_eq(vars_f64[idx], vars_f64[idx + 1], *symb, order);
        equation_vector.push(eq)
    }
    let (equation_vector, forbid) = check_for_forbidden(equation_vector);
    if forbid {
        return String::from("ERROR");
    }
    let res = order_then_calc(equation_vector);
    return res.to_string();
}

fn check_for_forbidden(eqs: Vec<Equation>) -> (Vec<Equation>, bool) {
    for eq in &eqs {
        if (eq.second_var == 0.0) && (eq.symbol == '/') {
            return (eqs, true);
        }
    }
    (eqs, false)
}
//clean up the strings of trailing and leading 0's and make it float
fn get_clean_vars_f64(vars: Vec<String>) -> Vec<f64> {
    let mut vars_f64: Vec<f64> = Vec::new();
    for var in vars {
        let mut new_var = var.trim_start_matches('0');
        if new_var.contains(".") {
            new_var = var.trim_end_matches('0');
        }
        let mut s_new_var = new_var.to_string();

        if !s_new_var.contains(".") {
            s_new_var.push_str(".")
        }
        if s_new_var.as_bytes()[s_new_var.len() - 1] as char == '.' {
            s_new_var.push_str("0");
        }

        let var_f64: f64 = match new_var.parse::<f64>() {
            Ok(x) => x,
            Err(e) => {
                println!("error parsing {}", var);
                0.0
            }
        };
        vars_f64.push(var_f64);
    }
    vars_f64
}

fn order_then_calc(mut eqs: Vec<Equation>) -> f64 {
    let mut result: f64 = 0.0;
    let (ord_1_eqs, last) = Equation::solve_order_2(eqs);
   
    if last {
        result = ord_1_eqs[0].first_var;
        return result;
    }
    result = Equation::solve_order_1(ord_1_eqs);
    return result;
}

struct Equation {
    first_var: f64,
    second_var: f64,
    symbol: char,
    order: u8,
}
impl Equation {
    fn init_eq(first_var: f64, second_var: f64, symbol: char, order: u8) -> Equation {
        Equation {
            first_var: first_var,
            second_var: second_var,
            symbol: symbol,
            order: order,
        }
    }
    fn solve_order_2(mut eqs: Vec<Equation>) -> (Vec<Equation>, bool) {
        //find order 2 equations and save their position
        let mut ord2_idx: Vec<usize> = Vec::new();
        for (idx, eq) in eqs.iter().enumerate() {
            if (eq.order == 2) {
                ord2_idx.push(idx);
            }
        }
        //if no order 2 equations were found
        if ord2_idx.len() == 0 {
            return (eqs, false);
        }
        //loop over indexes of order 2 equations and replace the variables in the other equations
        for i in ord2_idx {
            let eq = &eqs[i];
            let mut res: f64;
            match eq.symbol {
                '/' => res = eq.first_var / eq.second_var,
                '*' => res = eq.first_var * eq.second_var,
                _ => res = 0.0,
            }
            if eqs.len() > 1 {
                if i > 0 && i < eqs.len() - 1 {
                    eqs[i - 1].second_var = res;
                    eqs[i + 1].first_var = res;
                } else if i == 0 {
                    eqs[i + 1].first_var = res;
                } else if i == eqs.len() - 1 {
                    eqs[i - 1].second_var = res;
                } else {
                    println!("something has gone horribly wrong here while calculating the order 2 equations")
                }
                eqs.remove(i);
            } else {
                eqs[i].first_var = res;
                eqs[i].second_var = 0.0;
                return (eqs, true);
            }
        }
        return (eqs, false);
    }
    fn solve_order_1(mut eqs: Vec<Equation>) -> f64 {
        let mut res: f64 = 0.0;
        for i in 0..eqs.len() {
            let eq = &eqs[i];
            match eq.symbol {
                '+' => res = eq.first_var + eq.second_var,
                '-' => res = eq.first_var - eq.second_var,
                _ => res = 0.0,
            }

            if eqs.len() > 1 {
                if i > 0 && i < eqs.len() - 1 {
                    eqs[i - 1].second_var = res;
                    eqs[i + 1].first_var = res;
                } else if i == 0 {
                    eqs[i + 1].first_var = res;
                } else if i == eqs.len() - 1 {
                    eqs[i - 1].second_var = res;
                } else {
                    println!("something has gone horribly wrong here while calculating the order 2 equations")
                }
                eqs.remove(i);
            }
        }
        return res;
    }
    fn Display(eqs: &Vec<Equation>) -> String {
        let mut stringfied: String = String::from("Equations :\n");
        for eq in eqs {
            let str = format!(
                "{{\nfirst = {} \nsecond  = {} \nsymb = {} \norder = {}\n}}\n",
                eq.first_var, eq.second_var, eq.symbol, eq.order
            );
            stringfied.push_str(&str)
        }
        return stringfied;
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
