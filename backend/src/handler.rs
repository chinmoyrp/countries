use axum::{Json, response::IntoResponse, Extension};
use serde::{Serialize, Deserialize};
use regex::Regex;

use std::{sync::Arc};
use crate::{csv::{Countries}, country::Country};

#[derive(Debug, Serialize, Deserialize)]
pub struct Search {
    pub search: String,
    pub sortby: String,
    pub orderby: String,
    pub rows: usize
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ordering {
    pub sortby: String,
    pub orderby: String,
    pub rows: usize
}

#[derive(Debug, Serialize, Deserialize)]
struct ResultCountries {
    countries: Vec<Countries>
}

pub async fn countries(Json(Ordering{sortby, orderby, rows}): Json<Ordering>, Extension(countries): Extension<Arc<Countries>>) -> impl IntoResponse {
    let mut res = countries.to_vec();
    order(&mut res, &sortby, &orderby);

    let mut response = vec![];
    for chunk in res.chunks(rows) {
        response.push(chunk.to_vec());
    }

    if response.is_empty() {
        let undefined = "undefined".to_string();
        response.push(vec![
                            Country { 
                                id: undefined.clone(),
                                name: undefined.clone(),
                                capital: undefined.clone(),
                                timezone: undefined.clone(),
                                population: undefined
                            }]);
    }

    Json(ResultCountries{ countries : response })
}

pub async fn search(Json(Search{search, sortby, orderby, rows}): Json<Search>, Extension(countries): Extension<Arc<Countries>>) -> impl IntoResponse {
    let mut res = countries.to_vec();

    let filters:Vec<_> = search.split(' ').map(|item| item.trim()).collect();

    let filter_op = Regex::new(r"^([a-z]+)\.([a-z]+):(.+)$").unwrap();
    let filter_blank = Regex::new(r"^([a-z]+):(.+)$").unwrap();

    for f in filters {
        for cap in filter_op.captures_iter(f) {
            let field = (&cap[1]).to_string();
            let op = (&cap[2]).to_string();
            let value = (&cap[3]).to_string();

            res = res.into_iter()
                     .filter( |country| {
                         match field.as_str() {
                            "countryname" => do_filter(&country.name, false, &op, &value),
                            "capital" => do_filter(&country.capital, false, &op, &value),
                            "id" => do_filter(&country.id, true, &op, &value),
                            "population" => do_filter(&country.population, true, &op, &value),
                            _ => true
                         }
                     })
                     .collect();
        }

        for cap in filter_blank.captures_iter(f) {
            let field = (&cap[1]).to_string();
            let op = String::from("eq");
            let value = (&cap[2]).to_string();

            res = res.into_iter()
                     .filter( |country| {
                         match field.as_str() {
                            "countryname" => do_filter(&country.name, false, &op, &value),
                            "capital" => do_filter(&country.capital, false, &op, &value),
                            "id" => do_filter(&country.id, true, &op, &value),
                            "population" => do_filter(&country.population, true, &op, &value),
                            _ => true
                         }
                     })
                     .collect();
            
        }
    }
    
    order(&mut res, &sortby, &orderby);    

    let mut response = vec![];
    for chunk in res.chunks(rows) {
        response.push(chunk.to_vec());
    }

    if response.is_empty() {
        let undefined = "undefined".to_string();
        response.push(vec![
                            Country { 
                                id: undefined.clone(),
                                name: undefined.clone(),
                                capital: undefined.clone(),
                                timezone: undefined.clone(),
                                population: undefined
                            }]);
    }

   Json(ResultCountries{ countries : response })
}

fn do_filter(field: &str, is_number: bool, op: &str, value: &str) -> bool {

    let parse_num = |num_str: &str| {
        match num_str.parse::<u32>() {
            Ok(num) => Some(num),
            Err(_) => None
        }
    }; 

    match op {
        "eq" => {
            field == value
        }
        "gt" => {
            if is_number {
                let lhs = parse_num(field);
                let rhs = parse_num(value);

                if lhs.is_some() && rhs.is_some() {
                    return lhs.unwrap() > rhs.unwrap();
                }
                return false;
            }
            
            field > value
        }
        "lt" => {
            if is_number {
                let lhs = parse_num(field);
                let rhs = parse_num(value);

                if lhs.is_some() && rhs.is_some() {
                    return lhs.unwrap() < rhs.unwrap();
                }
                return false;
            }

            field < value
        }
        "regex" => {
            let re = match Regex::new(value) {
                Ok(r) => r,
                Err(_) => return false
            };
            re.is_match(field)
        }

        _ => true
    }
}

fn order(countries: &mut Countries, sortby: &str, orderby: &str) {
    if orderby == "desc" {
        match sortby {
            "id" => countries.sort_by(|a,b| b.id.parse::<u32>().unwrap().cmp(&a.id.parse::<u32>().unwrap())),
            "name" => countries.sort_by(|a,b| b.name.cmp(&a.name)),
            "capital" => countries.sort_by(|a,b| b.capital.cmp(&a.capital)),
            "population" => countries.sort_by(|a,b| b.population.parse::<u32>().unwrap().cmp(&a.population.parse::<u32>().unwrap())),
            _ => ()
        }
    } else if orderby == "asc" {
        match sortby {
            "id" => countries.sort_by(|a,b| a.id.parse::<u32>().unwrap().cmp(&b.id.parse::<u32>().unwrap())),
            "name" => countries.sort_by(|a,b| a.name.cmp(&b.name)),
            "capital" => countries.sort_by(|a,b| a.capital.cmp(&b.capital)),
            "population" => countries.sort_by(|a,b| a.population.parse::<u32>().unwrap().cmp(&b.population.parse::<u32>().unwrap())),
            _ => ()
        }
    }
}

