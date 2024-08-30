use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SystemBase)]
pub fn derive_system_base(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("SystemBase can only be derived for structs with named fields"),
        },
        _ => panic!("SystemBase can only be derived for structs"),
    };

    let impl_system_base = quote! {
        impl<T: AtlasScalar> SystemBase for #name<T> {
            fn input_ports(&self) -> Vec<&dyn InputPortBase> {
                self.input_ports
                    .iter()
                    .map(|p| p as &dyn InputPortBase)
                    .collect()
            }

            fn input_ports_mut(&mut self) -> Vec<&mut dyn InputPortBase> {
                self.input_ports
                    .iter_mut()
                    .map(|p| p as &mut dyn InputPortBase)
                    .collect()
            }

            fn output_ports(&self) -> Vec<&dyn OutputPortBase> {
                self.output_ports
                    .iter()
                    .map(|p| p.as_ref() as &dyn OutputPortBase)
                    .collect()
            }

            fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPortBase> {
                self.output_ports
                    .iter_mut()
                    .map(|p| p.as_mut() as &mut dyn OutputPortBase)
                    .collect()
            }

            fn cache_entries(&self) -> &Vec<CacheEntry> {
                &self.cache_entries
            }

            fn cache_entries_mut(&mut self) -> &mut Vec<CacheEntry> {
                &mut self.cache_entries
            }

            fn context_sizes(&self) -> &ContextSizes {
                &self.context_sizes
            }

            fn context_sizes_mut(&mut self) -> &mut ContextSizes {
                &mut self.context_sizes
            }

            fn system_id(&self) -> &SystemId {
                &self.system_id
            }

            fn parent_service(&self) -> Option<&dyn SystemParentServiceInterface> {
                self.parent_service.as_ref().map(|p| p.as_ref())
            }
        }
    };

    impl_system_base.into()
}
