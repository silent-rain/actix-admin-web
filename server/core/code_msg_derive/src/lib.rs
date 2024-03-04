use quote::quote;
use syn::token::Comma;
use syn::{punctuated::Punctuated, Expr, Ident, LitInt, LitStr, Variant};

// parse attrs
// #[status(code = 0, msg = "ok")]
fn get_attrs(
    enum_name: Ident,
    variants: Punctuated<Variant, Comma>,
) -> syn::Result<(Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>)> {
    let mut code_arms = vec![];
    let mut msg_arms = vec![];

    for variant in variants {
        let variant_ident = &variant.ident;
        let mut code: Option<u16> = None;
        let mut msg: Option<String> = None;

        variant
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("status"))
            .try_for_each(|attr| {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("code") {
                        let lit: u16 = meta.value()?.parse::<LitInt>()?.base10_parse()?;
                        code = Some(lit);
                    } else if meta.path.is_ident("msg") {
                        let lit: String = meta.value()?.parse::<LitStr>()?.value();
                        msg = Some(lit);
                    } else {
                        // Reads the value expression to advance the parse stream.
                        // Some parameters, such as `primary_key`, do not have any value,
                        // so ignoring an error occurred here.
                        let _: Option<Expr> = meta.value().and_then(|v| v.parse()).ok();
                    }

                    Ok(())
                })
            })
            .expect("Failed to parse `status` attribute");

        code_arms.push(quote! {
            #enum_name::#variant_ident => #code,
        });

        msg_arms.push(quote! {
            #enum_name::#variant_ident => #msg,
        });
    }

    Ok((code_arms, msg_arms))
}

#[proc_macro_derive(CodeMessage, attributes(status))]
pub fn code_msg_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let item_enum: syn::ItemEnum = syn::parse2(input)
        .expect("failed to parse proc macro2 token stream into the selected syntax tree node");
    let enum_name = item_enum.ident;

    let variants = item_enum.variants;

    let (code_arms, msg_arms) =
        get_attrs(enum_name.clone(), variants).expect("failed to obtain attribute value");

    let code_fn = quote! {
        pub fn code(&self) -> u16 {
            match self {
                #(#code_arms)*
            }
        }
    };

    let msg_fn = quote! {
        pub fn msg(&self) -> &'static str {
            match self {
                #(#msg_arms)*
            }
        }
    };

    let expanded = quote! {
        impl #enum_name {
            #code_fn
            #msg_fn
        }
    };

    proc_macro::TokenStream::from(expanded)
}
