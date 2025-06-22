
extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Comma};


#[proc_macro_derive(Show)]
pub fn show_derive(input: TokenStream) -> TokenStream { 

  let ast: syn::DeriveInput = syn::parse(input).unwrap();
  let data = ast.data;

  let impl_show = |name: syn::Ident, punct: Punctuated<syn::Field, Comma>| {

    let show = |punct: &Punctuated<syn::Field, Comma>| {
      
      let stmts = punct
        .iter()
        .map(|field| field.clone())
        .fold(
          quote! { }, 
          |mut q, field| {            
            let ident = field.ident.unwrap();
            let tokens = quote! {
              imgui::text(self.#ident);
            };
            q.extend(tokens);
            q
          }
        );

      quote! {
        unsafe fn show(&self, id: &mut i32) { 
          #stmts
        }
      }
    };

    let show = show(&punct);

    quote! {
      impl imgui::Show for #name {
        #show
      }
    }
  };

  let fields = match data {
    syn::Data::Struct(data) => {
      match data.fields {
        syn::Fields::Named(fields) => fields.named,
        syn::Fields::Unnamed(fields) => fields.unnamed,
        syn::Fields::Unit => panic!(),
      }
    },
    syn::Data::Enum(_) => {
      unimplemented!()
    },
    syn::Data::Union(_) => {
      unimplemented!()
    },
  };

  let mut q = quote! { };
  q.extend(impl_show(ast.ident, fields.clone()));

  q.into()
}


#[proc_macro_attribute]
pub fn assert_imgui_context(args: TokenStream, input: TokenStream) -> TokenStream {

  assert!(args.is_empty());

  let ast: syn::ImplItemFn = syn::parse(input).unwrap();

  let mut func = ast.clone();
  func.block = {
    let mut block = ast.block;
    //block.stmts.insert(
    //  0, 
    //  syn::parse(quote! { println!("Function Called, Wow!"); }.into()).unwrap(),
    //);
    block
  };

  quote! { #func }.into()
}





