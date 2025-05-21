use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SystemBase)]
pub fn derive_system_base(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let _fields = match &input.data {
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

            fn set_implicit_time_derivatives_residual_size(&mut self, size: usize) {
                self.implicit_time_derivatives_residual_size = Some(size);
            }
        }
    };

    impl_system_base.into()
}

#[proc_macro_derive(AbstractSystem)]
pub fn derive_abstract_system(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let _fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("SystemBase can only be derived for structs with named fields"),
        },
        _ => panic!("SystemBase can only be derived for structs"),
    };

    let impl_abstract_system = quote! {
        impl<T: AtlasScalar> AbstractSystem for #name<T> {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };

    impl_abstract_system.into()
}

#[proc_macro_derive(System)]
pub fn derive_system(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let _fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("SystemBase can only be derived for structs with named fields"),
        },
        _ => panic!("SystemBase can only be derived for structs"),
    };

    let impl_system = quote! {
        impl<T: AtlasScalar> System<T> for #name<T> {
            type CN = LeafContext<T>;

            fn input_ports(&self) -> Vec<&InputPort<T>> {
                self.input_ports.iter().collect()
            }

            fn input_ports_mut(&mut self) -> Vec<&mut InputPort<T>> {
                self.input_ports.iter_mut().collect()
            }

            fn input_port(&self, index: &InputPortIndex) -> &InputPort<T> {
                &self.input_ports[index]
            }

            fn input_port_mut(&mut self, index: &InputPortIndex) -> &mut InputPort<T> {
                &mut self.input_ports[index]
            }

            fn add_input_port(&mut self, input_port: InputPort<T>) {
                self.input_ports.push(input_port);
            }

            fn output_ports(&self) -> Vec<&dyn OutputPort<T, CN = Self::CN>> {
                self.output_ports
                    .iter()
                    .map(|p| p.as_ref() as &dyn OutputPort<T, CN = Self::CN>)
                    .collect()
            }

            fn output_ports_mut(&mut self) -> Vec<&mut dyn OutputPort<T, CN = Self::CN>> {
                self.output_ports
                    .iter_mut()
                    .map(|p| p.as_mut() as &mut dyn OutputPort<T, CN = Self::CN>)
                    .collect()
            }

            fn output_port(&self, index: &OutputPortIndex) -> &dyn OutputPort<T, CN = Self::CN> {
                self.output_ports[index].as_ref()
            }

            fn output_port_mut(
                &mut self,
                index: &OutputPortIndex,
            ) -> &mut dyn OutputPort<T, CN = Self::CN> {
                self.output_ports[index].as_mut()
            }

            fn system_weak_link(&self) -> SystemWeakLink<T> {
                self.system_weak_link.clone().unwrap()
            }

            fn time_derivatives_cache_index(&self) -> &CacheIndex {
                &self.time_derivatives_cache_index
            }

            fn allocate_context(&self) -> Box<Self::CN> {
                LeafSystem::<T>::allocate_context(self)
            }

            fn do_allocate_input(&self, input_port: &InputPort<T>) -> Box<dyn AbstractValue> {
                LeafSystem::<T>::do_allocate_input(self, input_port)
            }

            fn allocate_time_derivatives(&mut self) -> Box<<<Self::CN as Context<T>>::S as State<T>>::CS> {
                LeafSystem::<T>::allocate_time_derivatives(self)
            }

            fn set_default_state(&self, context: &mut Self::CN) {
                LeafSystem::<T>::set_default_state(self, context)
            }
        }
    };

    impl_system.into()
}

#[proc_macro_derive(LeafSystem)]
pub fn derive_leaf_system(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let _fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("SystemBase can only be derived for structs with named fields"),
        },
        _ => panic!("SystemBase can only be derived for structs"),
    };

    let impl_leaf_system = quote! {
        impl<T: AtlasScalar> LeafSystem<T> for #name<T> {
            fn model_input_values(&self) -> &ModelValues {
                &self.model_input_values
            }

            fn model_input_values_mut(&mut self) -> &mut ModelValues {
                &mut self.model_input_values
            }

            fn model_continuous_state_vector(&self) -> &BasicVector<T> {
                &self.model_continuous_state_vector
            }

            fn model_continuous_state_vector_mut(&mut self) -> &mut BasicVector<T> {
                &mut self.model_continuous_state_vector
            }

            fn leaf_output_port(&self, output_port_index: &OutputPortIndex) -> &LeafOutputPort<T> {
                &self.output_ports[output_port_index]
            }

            fn leaf_output_port_mut(
                &mut self,
                output_port_index: &OutputPortIndex,
            ) -> &mut LeafOutputPort<T> {
                &mut self.output_ports[output_port_index]
            }

            fn add_output_port(&mut self, output_port: Box<LeafOutputPort<T>>) {
                self.output_ports.push(output_port);
            }
        }
    };

    impl_leaf_system.into()
}
