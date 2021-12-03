use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn, Signature};

#[derive(Debug, FromMeta)]
struct AocArgs {
  day: u8,
  name: String,
  #[darling(default)]
  bench: bool,
}

#[proc_macro_attribute]
pub fn aoc(metadata: TokenStream, input: TokenStream) -> TokenStream {
  let AocArgs { day, name, bench } = AocArgs::from_list(&parse_macro_input!(metadata as AttributeArgs)).unwrap();
  let ItemFn { block, .. } = syn::parse::<ItemFn>(input).unwrap();

  let stream = quote! {
    #[allow(clippy::unnecessary_mut_passed)]
    fn main() {
      let start = std::time::Instant::now();

      println!("Advent of Code 2021");
      println!("Day {} • {}", #day, #name);
      println!();

      let input = #block;

      part_one(&mut input.clone());
      part_two(&mut input.clone());

      if #bench {
        let elapsed = start.elapsed();
        let millis = elapsed.subsec_nanos() as u64 / 1_000_000;
        let micros = elapsed.subsec_nanos() as u64 / 1_000;

        println!();
        println!("Took {:?}", elapsed);
      }
    }
  };

  stream.into()
}

#[derive(Debug, FromMeta)]
struct TaskArgs {
  part: u8,
  name: String,
  #[darling(default)]
  skip: bool,
}

#[proc_macro_attribute]
pub fn task(metadata: TokenStream, input: TokenStream) -> TokenStream {
  let TaskArgs { part, name, skip } = TaskArgs::from_list(&parse_macro_input!(metadata as AttributeArgs)).unwrap();

  let ItemFn { block, sig, .. } = syn::parse::<ItemFn>(input).unwrap();
  let Signature { ident, inputs, .. } = sig;

  let inner_fn_name = Ident::new(&format!("inner_{}", ident), ident.span());

  let fn_name = match part {
    1 => Ident::new("part_one", ident.span()),
    2 => Ident::new("part_two", ident.span()),
    _ => panic!("invalid task number"),
  };

  let stream = match skip {
    true => quote! {
      fn #fn_name(#inputs) {}
    },

    false => quote! {
      fn #inner_fn_name(#inputs) -> u128 {
        let output = #block;

        output as u128
      }

      fn #fn_name(#inputs) {
        let mut inner = || {
          #block
        };

        let result = inner();

        println!("{} → {}", #name, result);
      }
    },
  };

  stream.into()
}
