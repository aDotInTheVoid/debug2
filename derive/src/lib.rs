use quote::quote;
use syn::Fields;
use synstructure::{decl_derive, AddBounds};

decl_derive!([Debug, attributes(debug_skip)] => derive_debug);

// Based on
// https://github.com/panicbit/custom_debug/blob/master/custom_debug_derive/src/lib.rs

fn derive_debug(mut s: synstructure::Structure) -> proc_macro2::TokenStream {
    s.add_bounds(AddBounds::Generics);

    let variants = s.each_variant(|variant| {
        let name = variant.ast().ident.to_string();

        let debug_helper = match variant.ast().fields {
            Fields::Named(_) | Fields::Unit => quote! {debug_struct},
            Fields::Unnamed(_) => quote! {debug_tuple},
        };

        let variant_body = variant.bindings().iter().map(|b| {
            let format = quote! {#b};

            if let Some(ref name) = b.ast().ident.as_ref().map(<_>::to_string) {
                quote! {
                    s.field(#name, #format);
                }
            } else {
                quote! {
                    s.field(#format);
                }
            }
        });

        quote! {
            let mut s = f.#debug_helper(#name);
            #(#variant_body)*
            s.finish()
        }
    });

    s.gen_impl(quote! {
        gen impl debug2::Debug for @Self {
            fn fmt(&self, f: &mut debug2::Formatter<'_>) -> std::fmt::Result {
                match self { #variants }
            }
        }
    })
}
