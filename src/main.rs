use onnx_helpers::*;
use onnx_pb::tensor_proto::DataType;

fn main() {

    let x_input: onnx_pb::ValueInfoProto = make_tensor_value_info("X", DataType::Float, vec![1, 10], None);
    let inputs = vec![
        x_input
    ];

    let node1 = make_node(
        "abc",
        vec!["X"],
        vec!["Z"],
        None,
        None,
        None,
        vec![make_attribute("axes", vec![1i64].into())],
    );

    let graph = make_graph(
        vec![node1],
        "name",
//        inputs,
        vec![],
        vec![],
        vec![],
        None,
    );

    let model = make_model::<&str>(graph,
                           None,
                           None,
                           None,
                           None,
                           None,
                           None,
                           None,
    );
}
