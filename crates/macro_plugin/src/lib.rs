use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn rtools_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(item);
    let struct_name = &item_struct.ident;

    let ts = quote! {
        #[no_mangle]
        pub fn _plugin_create(config: &plugin::Config, options: String) -> std::sync::Arc<dyn plugin::Plugin> {
          std::sync::Arc::new(#struct_name::new(config, options))
        }

        #item_struct
    };

    ts.into()
}
