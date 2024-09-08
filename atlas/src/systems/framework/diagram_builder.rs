use std::collections::{HashMap, HashSet};

use crate::common::atlas_scalar::AtlasScalar;
use crate::systems::framework::diagram::{
    Diagram, InputPortLocator, OutputPortLocator, OwnedSystems, SystemPtr,
};
use crate::systems::framework::diagram_context::DiagramContext;
use crate::systems::framework::framework_common::{InputPortIndex, PortDataType};
use crate::systems::framework::input_port::InputPort;
use crate::systems::framework::leaf_context::LeafContext;
use crate::systems::framework::output_port::OutputPort;
use crate::systems::framework::system::System;

struct ExportedInputPortData<T: AtlasScalar> {
    pub input_port_locator: InputPortLocator<T>,
}

#[derive(Default)]
pub struct DiagramBuilder<T: AtlasScalar> {
    input_port_ids: Vec<InputPortLocator<T>>,
    output_port_ids: Vec<OutputPortLocator<T>>,
    diagram_input_data: Vec<ExportedInputPortData<T>>,
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

    pub fn connect_input_port(
        &mut self,
        diagram_input_port_index: InputPortIndex,
        input_port: &InputPort<T>,
    ) {
        self.assert_if_already_built();
        let input_port_locator = InputPortLocator {
            system_ptr: input_port.system_ptr(),
            input_port_index: input_port.index().clone(),
        };
        self.assert_if_input_already_connected(&input_port_locator);
        self.assert_if_system_not_registered(&input_port.system_ptr());

        // Check that port types match.
        let exported_input_port_data = &self.diagram_input_data[diagram_input_port_index.value()];
        let input_port_locator = &exported_input_port_data.input_port_locator;
        let diagram_input_port = input_port_locator
            .system_ptr
            .input_port(input_port_locator.input_port_index.clone());
        if input_port.data_type() != diagram_input_port.data_type() {
            panic!(
                "DiagramBuilder::connect_input_port: Cannot mix vector-valued and abstract-valued ports while connecting input port (data type {:?}) of System to input port (data type {:?}) of Diagram",
                input_port.data_type(),
                input_port.data_type(),
            );
        }

        if *input_port.data_type() == PortDataType::AbstractValued {
            let input_port_model = input_port.system_ptr().allocate_input_abstract(input_port);
            let diagram_input_port_model = diagram_input_port
                .system_ptr()
                .allocate_input_abstract(diagram_input_port);
            if input_port_model.type_id() != diagram_input_port_model.type_id() {
                panic!(
                    "DiagramBuilder::connect_input_port: Mismatched value types while connecting input port (type {:?}) of System to input port (type {:?}) of Diagram",
                    input_port_model.type_id(),
                    diagram_input_port_model.type_id()
                );
            }
        }

        self.input_port_ids.push(input_port_locator.clone());
    }

    fn assert_if_already_built(&self) {
        if self.already_built {
            panic!("DiagramBuilder already built");
        }
    }

    fn assert_if_input_already_connected(&self, input_port_locator: &InputPortLocator<T>) {
        if self.connection_map.contains_key(input_port_locator) {
            panic!(" Input port is already connected");
        }
    }

    fn assert_if_system_not_registered(&self, system: &SystemPtr<T>) {
        if !self.systems.contains(system) {
            panic!("System has not been registered to this DiagramBuilder");
        }
    }
}
