use proc_macro2::Ident;
use syn::{Type, ItemFn, ReturnType, spanned::Spanned};

use crate::{partflag::AocPart, parser::{genargs::AocGeneratorArgs, solverargs::AocSolverArgs, solutionargs::AocSolutionArgs}};

#[derive(Debug, PartialEq, Eq)]
pub struct AocGeneratorData<'a> {
    pub display_slug: Ident,
    pub gen_type: &'a Type,
    pub source: &'a ItemFn,
}

impl<'a> AocGeneratorData<'a> {
    pub fn new(args: AocGeneratorArgs, source_fn: &'a ItemFn) -> syn::Result<AocGeneratorData<'a>> {
        let ReturnType::Type(_, ty_data) = &source_fn.sig.output else {
            let e = syn::Error::new(source_fn.sig.output.span(), "Generators must have a return type that can be passed to a solver function.");
            return Err(e);
        };
        Ok(AocGeneratorData { 
            display_slug: args.display_slug, 
            gen_type: ty_data.as_ref(), 
            source: &source_fn 
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AocSolverData<'a> {
    pub problem_part: AocPart,
    pub display_slug: Ident,
    pub input_type: &'a Type,
    pub source: &'a ItemFn,
}

impl<'a> AocSolverData<'a> {
    pub fn new(args: AocSolverArgs, source_fn: &'a ItemFn) -> syn::Result<AocSolverData<'a>> {
        if source_fn.sig.inputs.len() != 1 {
            let e = syn::Error::new(source_fn.sig.inputs.span(), "Solvers must accept exactly one argument, the data from the generator. This argument may be a tuple, struct, or other type.");
            return Err(e);
        } else {
            let Some(solve_type) = source_fn.sig.inputs.first() else {
                panic!("This code should be unreachable.");
            };
            let syn::FnArg::Typed(solve_type) = solve_type else {
                let e = syn::Error::new(solve_type.span(), "Solvers cannot be methods which take a self param");
                return Err(e);
            };
            let solve_type = &solve_type.ty;
            return Ok(AocSolverData { 
                problem_part: args.problem_part, 
                display_slug: args.display_slug, 
                input_type: solve_type.as_ref(), 
                source: source_fn,
            });
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AocSolutionData<'a> {
    pub problem_part: AocPart,
    pub display_slug: Ident,
    pub source: &'a ItemFn,
}

impl<'a> AocSolutionData<'a> {
    pub fn new(args: AocSolutionArgs, source_fn: &'a ItemFn) -> AocSolutionData<'a> {
        AocSolutionData {
            problem_part: args.problem_part,
            display_slug: args.display_slug,
            source: source_fn,
        }
    }
}