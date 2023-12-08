#![feature(test)]

extern crate test;

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use regex::Regex;

use graphviz_rust::{
    cmd::{Layout, Format},
    exec, parse,
    printer::PrinterContext,
};

fn main() -> std::io::Result<()> {
    let input = include_str!("../../input1.txt");
    let mut file = File::create("day-08.svg")?;
    file.write_all(input_to_svg(input)?.as_slice())?;
    Ok(())
}

fn input_to_svg(input: &str) -> std::io::Result<Vec<u8>>  {
    let mut output = String::new();

    output.push_str("\
        digraph G {\n\
        graph [splines=true overlap=false]\n");


    let line_regex = Regex::new(r"([A-Z0-9]{3}) = \(([A-Z0-9]{3}), ([A-Z0-9]{3})\)").unwrap();

    let mut input_iter = input.lines();
    let _left_right: Vec<char> = input_iter.next().unwrap()
        .chars()
        .collect();
    let _ = input_iter.next().unwrap();
    let elements: HashMap<&str, (&str, &str)> = input_iter.map(|line| {
            let (_, [key, left, right]) = line_regex.captures(line).unwrap().extract();
            (key, (left, right))
        })
        .collect();
    
    for (source, (left, right)) in elements {
        output.push_str(format!("    {} -> {}\n", source, left).as_str());
        output.push_str(format!("    {} -> {}\n", source, right).as_str());
    }

    output.push_str("}\n");
    let graph = parse(&output).unwrap();

    exec(
        graph,
        &mut PrinterContext::default(),
        vec![
            Format::Svg.into(),
            Layout::Neato.into()],
    )
}
