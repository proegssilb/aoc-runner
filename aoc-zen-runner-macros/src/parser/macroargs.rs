use proc_macro2::Ident;
use syn::parse::Parse;

#[derive(Debug, PartialEq, Eq)]
pub struct AocMacroArgs {
    pub day_num: u32,
}

impl Parse for AocMacroArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let day_ident: Ident = input.parse()?;

        let day_part = day_ident.to_string();
        let day_part = day_part.strip_prefix("day").unwrap_or(&day_part);
        let day_part = day_part.strip_prefix("d").unwrap_or(&day_part);
        let day_num: u32 = day_part.parse().or_else(|a| {
            let msg = format!("Could not parse number from day indicator. Parsing error:\n{}", a);
            let e = syn::Error::new(day_ident.span(), msg);
            return Err(e);
        })?;

        if day_num < 1 || day_num > 25 {
            let e = syn::Error::new(day_ident.span(), "Day number is out of range of 1-25");
            return Err(e);
        }

        Ok(AocMacroArgs {day_num})
    }
}