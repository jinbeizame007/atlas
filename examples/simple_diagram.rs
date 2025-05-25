use atlas::systems::primitives::adder::Adder;
use atlas::systems::framework::diagram_builder::DiagramBuilder;
use atlas::systems::framework::framework_common::{InputPortIndex, OutputPortIndex};
use atlas::systems::framework::basic_vector::BasicVector;
use atlas::systems::framework::diagram::DiagramExt;
use atlas::systems::framework::system::System;

fn main() {
    let mut diagram_builder = DiagramBuilder::<f64>::new();

    let num_inputs = 2;
    let vector_size = 3;
    let adder1 = Adder::new(num_inputs, vector_size);
    let adder2 = Adder::new(num_inputs, vector_size);
    let adder3 = Adder::new(num_inputs, vector_size);

    let mut adder1_link = diagram_builder.add_leaf_system(&adder1);
    let mut adder2_link = diagram_builder.add_leaf_system(&adder2);
    let adder3_link = diagram_builder.add_leaf_system(&adder3);

    diagram_builder.export_input_port(adder1_link.input_port(InputPortIndex::new(0)));
    diagram_builder.export_input_port(adder1_link.input_port(InputPortIndex::new(1)));
    diagram_builder.export_input_port(adder2_link.input_port(InputPortIndex::new(0)));
    diagram_builder.export_input_port(adder2_link.input_port(InputPortIndex::new(1)));

    diagram_builder.connect(
        adder1_link.output_port_mut(OutputPortIndex::new(0)),
        adder3_link.input_port(InputPortIndex::new(0)),
    );
    diagram_builder.connect(
        adder2_link.output_port_mut(OutputPortIndex::new(0)),
        adder3_link.input_port(InputPortIndex::new(1)),
    );

    diagram_builder.export_output_port(adder3_link.output_port(OutputPortIndex::new(0)));

    let diagram = diagram_builder.build();
    let diagram_context = diagram.borrow_mut().create_default_context();

    let inputs = [
        BasicVector::<f64>::from_vec(vec![1.0, 2.0, 3.0]),
        BasicVector::<f64>::from_vec(vec![4.0, 5.0, 6.0]),
        BasicVector::<f64>::from_vec(vec![7.0, 8.0, 9.0]),
        BasicVector::<f64>::from_vec(vec![10.0, 11.0, 12.0]),
    ];

    for (i, input) in inputs.iter().enumerate() {
        diagram
            .input_port_mut(&InputPortIndex::new(i))
            .fix_value(diagram_context.borrow_mut(), input.clone());
    }

    let sum = diagram
        .diagram_output_port(&OutputPortIndex::new(0))
        .eval::<BasicVector<f64>>(diagram_context.borrow());
    println!("sum: {:?}", sum);
}
