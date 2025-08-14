use risc0_build::{embed_methods_with_options, DockerOptionsBuilder, GuestOptionsBuilder};

fn main() {
    let docker_options = DockerOptionsBuilder::default()
        .root_dir(std::env::current_dir().unwrap().parent().unwrap())
        .build()
        .unwrap();
    let guest_options = GuestOptionsBuilder::default()
        .use_docker(docker_options)
        .build()
        .unwrap();

    println!("guest_options: {:?}", guest_options);
    let built_guests = embed_methods_with_options(std::collections::HashMap::from([(
        "guest",
        guest_options,
    )]));
    println!("built_guests: {:?}", built_guests);
    let image_id = built_guests[0].image_id;

    // this errs if the dir already exists, so we don't handle an error.
    let _ = std::fs::create_dir("./risc0/out");

    std::fs::write(
        "./risc0/out/riscv32im-risc0-vk",
        format!("0x{}\n", hex::encode(image_id.as_bytes())),
    )
    .expect("could not write Risc0 vk to file");
}
