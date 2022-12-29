use protobuf_codegen::Codegen;

fn main() {
     Codegen::new()
        .pure()
        .cargo_out_dir("generated_with_pure")
        .input("src/protos/otpm.proto")
        .include("src/protos")
        .run_from_script();
}
