use std::{fs, path::PathBuf};

use cairo_lang_sierra::program::VersionedProgram;
use cairo_runner::{process_args, run, Args};

fn main() {
    let node_0_sierra_file = "model/cairo_programs/node_0/target/dev/node_0.sierra.json";
    let node_0_program_args = "model/program_args/node_0.args";
    let node_0_result_path = "model/vm_results/node_0";

    let node_1_sierra_file = "model/cairo_programs/node_1/target/dev/node_1.sierra.json";
    let node_1_program_args = "model/program_args/node_1.args";
    let node_1_result_path = "model/vm_results/node_1";

    println!("Running Node 0 in proof mode...");
    let program_args = fs::read_to_string(&node_0_program_args)
        .expect("Failed to read the program arguments file");
    run_vm(node_0_sierra_file, &program_args, node_0_result_path);
    println!("Node 0 executed successfully in proof mode ✅");

    println!("Running Node 1 in proof mode...");
    let program_args = fs::read_to_string(&node_1_program_args)
        .expect("Failed to read the program arguments file");
    run_vm(node_1_sierra_file, &program_args, node_1_result_path);
    println!("Node 1 executed successfully in proof mode ✅");
}

fn run_vm(sierra_file: &str, program_args: &str, result_path: &str) {
    let trace_path = format!("{result_path}/program.trace");
    let memory_path = format!("{result_path}/program.memory");

    let sierra_content = std::fs::read(sierra_file).expect("Failed to read Sierra file");
    let versioned_program = serde_json::from_slice::<VersionedProgram>(&sierra_content)
        .expect("Failed to create Version Program");
    let program = versioned_program
        .into_v1()
        .expect("Failed to create Program");
    let program = program.program;
    let program_args = process_args(program_args).expect("Failed to process provided arguments");

    let args = Args {
        trace_file: Some(PathBuf::from(trace_path)),
        memory_file: Some(PathBuf::from(memory_path)),
        layout: "all_cairo".to_string(),
        proof_mode: true,
        air_public_input: None,
        air_private_input: None,
        cairo_pie_output: None,
        args: program_args,
        print_output: false,
        append_return_values: false,
    };

    let _run =
        run(program, args).map_err(|e| format!("Encountered an error with Cairo runner: {:?}", e));
}
