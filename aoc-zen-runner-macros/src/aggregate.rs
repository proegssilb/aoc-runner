use std::collections::HashMap;

use syn::{Type, ItemMod, Item};

use crate::{domain::{AocSolutionData, AocGeneratorData, AocSolverData}, parser::{genargs::AocGeneratorArgs, solverargs::AocSolverArgs, solutionargs::AocSolutionArgs}, partflag::AocPart};

pub struct AocSolutionsAggregation<'a> {
    pub solutions_p1: Vec<AocSolutionData<'a>>,
    pub solutions_p2: Vec<AocSolutionData<'a>>,
    pub generators: HashMap<&'a Type, Vec<AocGeneratorData<'a>>>,
    pub solvers_p1: HashMap<&'a Type, Vec<AocSolverData<'a>>>,
    pub solvers_p2: HashMap<&'a Type, Vec<AocSolverData<'a>>>,
}

impl<'a> AocSolutionsAggregation<'a> {
    pub fn new() -> Self {
        AocSolutionsAggregation { 
            solutions_p1: Vec::new(),
            solutions_p2: Vec::new(), 
            generators: HashMap::new(), 
            solvers_p1: HashMap::new(), 
            solvers_p2: HashMap::new(),
        }
    }

    pub fn p1_user_solns(&self) -> impl Iterator<Item = &AocSolutionData<'a>> {
        self.solutions_p1.iter()
    }

    pub fn p2_user_solns(&self) -> impl Iterator<Item = &AocSolutionData<'a>> {
        self.solutions_p2.iter()
    }

    pub fn p1_composed_solns(&self) -> impl Iterator<Item = (&AocGeneratorData<'a>, &AocSolverData<'a>)> {
        self.generators.iter().flat_map(|(ty, gens)| {
            if !self.solvers_p1.contains_key(*ty) {
                println!("WARNING: Generator type has no corresponding solvers:\n{:#?}", &ty);
                vec![].into_iter()
            } else {
                gens.iter().flat_map(|g| {
                    self.solvers_p1.get(ty).unwrap().iter().map(move |s| (g, s))
                }).collect::<Vec<(&AocGeneratorData<'a>, &AocSolverData<'a>)>>().into_iter()
            }
        })
    }

    pub fn p2_composed_solns(&self) -> impl Iterator<Item = (&AocGeneratorData<'a>, &AocSolverData<'a>)> {
        self.generators.iter().flat_map(|(ty, gens)| {
            if !self.solvers_p2.contains_key(*ty) {
                println!("WARNING: Generator type has no corresponding solvers:\n{:#?}", &ty);
                vec![].into_iter()
            } else {
                gens.iter().flat_map(|g| {
                    self.solvers_p2.get(ty).unwrap().iter().map(move |s| (g, s))
                }).collect::<Vec<(&AocGeneratorData<'a>, &AocSolverData<'a>)>>().into_iter()
            }
        })
    }
}

pub fn discover_mod_contents(module: &ItemMod) -> syn::Result<AocSolutionsAggregation> {
    let mut result = AocSolutionsAggregation::new();

    let Some((_, contents)) = &module.content else { return Ok(result); };
    for mod_item in contents.iter() {
        match mod_item {
            Item::Fn(fn_data) => {
                for attr in fn_data.attrs.iter() {
                    match attr.path().get_ident().map(|id| id.to_string()).as_deref() {
                        Some("generator") => {
                            let args = attr.parse_args::<AocGeneratorArgs>()?;
                            let data = AocGeneratorData::new(args, fn_data)?;
                            result.generators.entry(data.gen_type).or_default().push(data);
                        },
                        Some("solver") => {
                            let args = attr.parse_args::<AocSolverArgs>()?;
                            let data = AocSolverData::new(args, fn_data)?;
                            if data.problem_part == AocPart::Part1 {
                                result.solvers_p1.entry(data.input_type).or_default().push(data);
                            } else {
                                result.solvers_p2.entry(data.input_type).or_default().push(data);
                            }
                        },
                        Some("solution") => {
                            let args = attr.parse_args::<AocSolutionArgs>()?;
                            let data = AocSolutionData::new(args, fn_data);
                            if data.problem_part == AocPart::Part1 {
                                result.solutions_p1.push(data);
                            } else {
                                result.solutions_p2.push(data);
                            }
                        },
                        Some(_) => { continue; },
                        None => { continue; },
                    }
                }
            },
            _ => { continue; }
        }
    }

    Ok(result)
}