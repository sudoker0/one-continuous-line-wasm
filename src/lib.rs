use std::{cmp, collections::{HashMap, HashSet}};
use wasm_bindgen::prelude::*;

type Graph = HashMap<i32, HashSet<i32>>;
type Point = (i32, i32);

fn deserialize_v_pair(serialized: &str) -> Vec<Point> {
    let mut result = Vec::new();
    let pairs: Vec<&str> = serialized.split('/').collect();

    for pair_str in pairs {
        let mut pair_iss = pair_str.split(',');
        let first: i32 = pair_iss
            .next().unwrap()
            .parse().unwrap();
        let second: i32 = pair_iss
            .next().unwrap()
            .parse().unwrap();

        result.push((first, second));
    }

    result
}

fn serialize_v_vector(data: &[Vec<i32>]) -> String {
    let mut result = String::new();

    for (i, vec) in data.iter().enumerate() {
        for (j, &num) in vec.iter().enumerate() {
            result += &num.to_string();
            if j != vec.len() - 1 {
                result.push(',');
            }
        }

        if i != data.len() - 1 {
            result.push('/');
        }
    }

    result
}

fn trace(
    graph: &Graph,
    graph_point: &Vec<Point>,
    begin_point: i32,
    visited_points: HashSet<Point>,
    path: &Vec<i32>,
    max_solutions: i32,
    recursion_level: i32,
    solutions_cnt: &mut i32,
    solutions: &mut Vec<Vec<i32>>
) -> () {
    if max_solutions != 0 && *solutions_cnt >= max_solutions {
        return;
    }

    let data = graph
        .get(&begin_point)
        .cloned()
        .unwrap_or_default();
    let mut p_copy = path.clone();

    if data.len() == 0 {
        return;
    }

    p_copy.push(begin_point);

    for &data_it in &data {
        let mut vp_copy = visited_points.clone();
        let point_data = (
            cmp::min(begin_point, data_it),
            cmp::max(begin_point, data_it));

        if vp_copy.contains(&point_data) {
            continue;
        }

        vp_copy.insert(point_data);

        trace(
            graph,
            graph_point,
            data_it,
            vp_copy,
            &p_copy,
            max_solutions,
            recursion_level + 1,
            solutions_cnt,
            solutions
        );
    }

    if p_copy.len() >= graph_point.len() + 1 {
        solutions.push(p_copy);
        *solutions_cnt += 1;
    }
}

fn _one_line_solver(
    graph_point: &Vec<Point>,
    start_point: i32,
    max_solutions: i32
) -> Vec<Vec<i32>> {
    let mut graph: Graph = Default::default();
    let path: Vec<i32> = Default::default();
    let visited_points: HashSet<Point> = Default::default();
    let mut solutions: Vec<Vec<i32>> = Default::default();
    let mut solutions_cnt = 0;

    for &(begin, end) in graph_point {
        graph
            .entry(begin)
            .or_insert_with(HashSet::new)
            .insert(end);

        graph
            .entry(end)
            .or_insert_with(HashSet::new)
            .insert(begin);
    }

    // //? generate a list of nodes which have an odd number of edges
    let odd_nodes_list: Vec<(i32, HashSet<i32>)> = graph
        .clone()
        .into_iter()
        .filter(|(_, v)| (v.len() % 2 != 0))
        .collect();

    let mut list_of_odd_nodes = odd_nodes_list
        .clone()
        .into_iter()
        .map(|(k, _)| k);

    if odd_nodes_list.len() == 0 {
        //? condition check 1: no nodes have an odd number of edges
    } else if
        odd_nodes_list.len() == 2 &&
        list_of_odd_nodes.any(|x| x == start_point) {
        //? condition check 2: only two nodes have an odd number of edges
        //* condition check 2.1: those two nodes can only be the start point */
    } else {
        return solutions;
    }

    trace(
        &graph,
        graph_point,
        start_point,
        visited_points.clone(),
        &path,
        max_solutions,
        0,
        &mut solutions_cnt,
        &mut solutions
    );

    solutions
}

#[wasm_bindgen]
pub fn one_line_solver(
    graph_point: &str,
    start_point: i32,
    max_solutions: i32
) -> String {
    let out = _one_line_solver(
        &deserialize_v_pair(graph_point),
        start_point,
        max_solutions);
    serialize_v_vector(&out)
}