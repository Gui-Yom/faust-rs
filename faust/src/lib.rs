use litrs::Literal;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use std::path::Path;
use std::{env, fs, process};

#[proc_macro]
pub fn inline_dsp_module(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    eprintln!("{input:#?}");
    let mut tokens = TokenStream::from(input).into_iter().collect::<Vec<_>>();
    if tokens.len() != 3 {
        panic!("Expected 2 arguments separated by ','");
    }
    let dsp_name = match &tokens[0] {
        TokenTree::Ident(ident) => ident.clone(),
        _ => panic!("Expected an identifier as first argument"),
    };
    eprintln!("{dsp_name}");
    let faust_code = match litrs::Literal::try_from(tokens.remove(2)) {
        Ok(Literal::String(lit)) => lit.value().to_owned(),
        Err(e) => return e.to_compile_error(),
        _ => panic!("Expected a string literal"),
    };
    eprintln!("{faust_code}");

    let dir = env::var("OUT_DIR").unwrap();
    let dsp_fname = format!("{dir}/{dsp_name}.dsp");
    let gen = format!("{dir}/{dsp_name}.rs");
    fs::write(&dsp_fname, faust_code.as_bytes()).unwrap();

    let status = process::Command::new("C:\\W\\PR\\faust\\build\\bin\\Debug\\faust")
        .args(["-I", "C:\\W\\PR\\faust\\libraries", "-lang", "rust", "-o"])
        .arg(&gen)
        .arg(&dsp_fname)
        .status()
        .unwrap();

    eprintln!("Compilation status : {status}");

    proc_macro::TokenStream::from(quote! {
        use faust_traits::*;

        include!(#gen);
    })
}
