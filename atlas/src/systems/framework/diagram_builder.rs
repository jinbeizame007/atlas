use std::collections::{HashMap, HashSet};

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::diagram::{
    Diagram, InputPortLocator, OutputPortLocator, OwnedSystems, SystemPtr,
};
use crate::systems::framework::diagram_context::DiagramContext;
use crate::systems::framework::framework_common::PortDataType;
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::system::System;

struct ExportedInputPortData<T: AtlasScalar> {
    model_input: InputPortLocator<T>,
}

#[derive(Default)]
pub struct DiagramBuilder<T: AtlasScalar> {
    input_port_ids: Vec<InputPortLocator<T>>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    exported_input_ports: Vec<ExportedInputPortData<T>>,
    connection_map: HashMap<InputPortLocator<T>, OutputPortLocator<T>>,
    systems: HashSet<SystemPtr<T>>,
    owned_systems: OwnedSystems,
    already_built: bool,
}

impl<T: AtlasScalar> DiagramBuilder<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn systems(&self) -> &HashSet<SystemPtr<T>> {
        &self.systems
    }

    pub fn systems_mut(&mut self) -> &mut HashSet<SystemPtr<T>> {
        &mut self.systems
    }

    pub fn connection_map(&self) -> &HashMap<InputPortLocator<T>, OutputPortLocator<T>> {
        &self.connection_map
    }

    pub fn connection_map_mut(
        &mut self,
    ) -> &mut HashMap<InputPortLocator<T>, OutputPortLocator<T>> {
        &mut self.connection_map
    }

    pub fn add_leaf_system<S>(&mut self, mut system: Box<S>) -> SystemPtr<T>
    where
        S: System<T, CN = LeafContext<T>>,
        T: AtlasScalar,
    {
        let system_ptr =
            SystemPtr::LeafSystemPtr(system.as_mut() as *mut dyn System<T, CN = LeafContext<T>>);

        self.owned_systems.push(system);

        system_ptr
    }

    pub fn add_diagram<S>(&mut self, mut system: Box<S>) -> SystemPtr<T>
    where
        S: System<T, CN = DiagramContext<T>>,
        T: AtlasScalar,
    {
        let system_ptr =
            SystemPtr::DiagramPtr(system.as_mut() as *mut dyn System<T, CN = DiagramContext<T>>);

        self.owned_systems.push(system);

        system_ptr
    }

    pub fn connect<O>(&mut self, output_port: &mut O, input_port: &InputPort<T>)
    where
        O: OutputPort<T>,
    {
        self.assert_if_already_built();

        let input_port_locator = InputPortLocator::<T> {
            system_ptr: input_port.system_ptr(),
            input_port_index: input_port.index().clone(),
        };
        let output_port_locator = OutputPortLocator::<T> {
            system_ptr: output_port.system_ptr(),
            output_port_index: output_port.index().clone(),
        };

        if *output_port.data_type() == PortDataType::AbstractValued {
            let model_output = output_port.allocate();
            let model_input = input_port.system_ptr().allocate_input_abstract(input_port);
            if model_output.type_id() != model_input.type_id() {
                panic!(
                    "Mismatched value types while connecting output port (type {:?}) to input port (type {:?})",
                    model_output.type_id(),
                    model_input.type_id()
                );
            }
        }
        self.connection_map
            .insert(input_port_locator, output_port_locator);
    }

    // pub fn export_input_port(&mut self, input_port: &InputPort<T>) -> InputPortLocator<T> {
    //     self.assert_if_already_built();
    //     let diagram_port_index = self.declare_input(input_port);
    // }

    // pub fn connect_input_port(&mut self, input_port: &InputPort<T>) {
    //     self.assert_if_already_built();
    //     let input_port_locator = InputPortLocator {
    //         system_ptr: input_port.system_ptr(),
    //         input_port_index: input_port.index().clone(),
    //     };

    //     let data = self.
    // }

    pub fn assert_if_already_built(&self) {
        if self.already_built {
            panic!("DiagramBuilder already built");
        }
    }
}
