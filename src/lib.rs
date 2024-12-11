use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// This macro automatically adds tests marked with #[test] to the test collection.
/// Tests then can be run with libtest_mimic_collect::TestCollection::run().
#[proc_macro_attribute]
pub fn test(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn { sig, block, .. } = parse_macro_input!(input as ItemFn);

    let ident = &sig.ident;
    let test_name = ident.to_string();
    let test_name_str = LitStr::new(&test_name, Span::call_site());
    let ctor_name = format!("__{}_add_test", test_name);
    let ctor_ident = Ident::new(&ctor_name, Span::call_site());

    (quote! {
        #sig #block

        #[::libtest_mimic_collect::ctor]
        fn #ctor_ident() {
            use ::libtest_mimic_collect::ConvertResult;
            ::libtest_mimic_collect::TestCollection::add_test(
                ::libtest_mimic::Trial::test(
                    #test_name_str,
                    || -> Result<(), ::libtest_mimic::Failed> {
                        match ::std::panic::catch_unwind(|| {
                            ::libtest_mimic_collect::TestCollection::convert_result(#ident())
                        }) {
                            Ok(Ok(())) => Ok(()),
                            Ok(Err(err)) => Err(err),
                            Err(err) => Err(format!("{:?}", err).into()),
                        }
                    }
                )
            );
        }
    })
    .into()
}
