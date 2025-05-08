use clap::{Arg, Command, ValueEnum};
use std::{
    env, fmt,
    path::{Path, PathBuf},
    process,
    str::FromStr,
};

pub const CONTAINER_REGISTRY_URL: &str = "sc2cr.io/applications";

#[derive(Clone, Debug, ValueEnum)]
pub enum Functions {
    Fio,
    HelloWorld,
    TfInference,
}

impl fmt::Display for Functions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Functions::Fio => write!(f, "fio"),
            Functions::HelloWorld => write!(f, "hello-world"),
            Functions::TfInference => write!(f, "tf-inference"),
        }
    }
}

impl FromStr for Functions {
    type Err = ();

    fn from_str(input: &str) -> Result<Functions, Self::Err> {
        match input {
            "fio" => Ok(Functions::Fio),
            "hello-world" => Ok(Functions::HelloWorld),
            "tf-inference" => Ok(Functions::TfInference),
            _ => Err(()),
        }
    }
}

impl Functions {
    pub fn iter_variants() -> std::slice::Iter<'static, Functions> {
        static VARIANTS: [Functions; 3] = [
            Functions::Fio,
            Functions::HelloWorld,
            Functions::TfInference,
        ];
        VARIANTS.iter()
    }
}

/// This enum describes the different image tags we may have:
/// - unencrypted: regular docker image
/// - unencrypted-nydus: nydusified docker image (unencrypted)
#[derive(Clone, Debug, ValueEnum)]
pub enum ImageTags {
    Unencrypted,
    UnencryptedNydus,
}

impl fmt::Display for ImageTags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageTags::Unencrypted => write!(f, "unencrypted"),
            ImageTags::UnencryptedNydus => write!(f, "unencrypted-nydus"),
        }
    }
}

impl FromStr for ImageTags {
    type Err = ();

    fn from_str(input: &str) -> Result<ImageTags, Self::Err> {
        match input {
            "unencrypted" => Ok(ImageTags::Unencrypted),
            "unencrypted-nydus" => Ok(ImageTags::UnencryptedNydus),
            _ => Err(()),
        }
    }
}

impl ImageTags {
    pub fn iter_variants() -> std::slice::Iter<'static, ImageTags> {
        static VARIANTS: [ImageTags; 2] = [ImageTags::Unencrypted, ImageTags::UnencryptedNydus];
        VARIANTS.iter()
    }
}

pub fn do_docker_build(dockerfile: String, full_image_tag: String, image_path: String) {
    // ----- Build image -----

    let mut cmd = process::Command::new("docker");
    cmd.env("DOCKER_BUILDKIT", "1");
    cmd.arg("build")
        .arg("-t")
        .arg(full_image_tag.clone())
        .arg("-f")
        .arg(dockerfile)
        .arg(image_path);

    cmd.stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .output()
        .unwrap();

    // ----- Push image -----

    let mut cmd = process::Command::new("docker");
    cmd.env("DOCKER_BUILDKIT", "1");
    cmd.arg("push").arg(full_image_tag);

    cmd.stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .output()
        .unwrap();
}

pub fn do_nydusify(docker_tag: String, nydus_tag: String) {
    let nydusify_bin = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("deploy")
        .join("bin")
        .join("nydusify");

    let mut cmd = process::Command::new(nydusify_bin);
    cmd.arg("convert")
        .arg("--source-insecure")
        .arg("--source")
        .arg(docker_tag)
        .arg("--target-insecure")
        .arg("--target")
        .arg(nydus_tag)
        .env(
            "PATH",
            format!(
                "{}:/opt/confidential-containers/bin",
                env::var("PATH").unwrap()
            ),
        );

    cmd.stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .output()
        .unwrap();
}

pub fn build_fn_images(functions: Vec<Functions>) {
    for function in functions {
        let mut dockerfile_path = env::current_dir().unwrap();
        dockerfile_path.push("functions");
        dockerfile_path.push(format!("{function}"));
        dockerfile_path.push("Dockerfile");

        for image_tag in ImageTags::iter_variants() {
            let full_image_tag = format!("{CONTAINER_REGISTRY_URL}/{function}:{image_tag}");
            println!("Building and pushing {full_image_tag}...");

            match image_tag {
                ImageTags::Unencrypted => {
                    do_docker_build(
                        dockerfile_path.to_string_lossy().into_owned(),
                        full_image_tag,
                        dockerfile_path
                            .parent()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
                ImageTags::UnencryptedNydus => {
                    do_nydusify(
                        format!(
                            "{CONTAINER_REGISTRY_URL}/{function}:{}",
                            ImageTags::Unencrypted
                        ),
                        full_image_tag,
                    );
                }
            }
        }
    }
}

fn main() {
    // Sanity check
    if !Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("deploy")
        .exists()
    {
        println!("ERROR: sc2-sys deploy repository not found in ../deploy");
        println!("ERROR: clone the repository and run 'inv sc2.deploy --clean'");
        process::exit(1);
    }

    // Define the command-line interface
    let matches = Command::new("Argument Parser")
        .version("1.0")
        .arg(
            Arg::new("push")
                .long("push")
                .help("Push the image to the container registry")
                .num_args(0),
        )
        .arg(
            Arg::new("no-cache")
                .long("no-cache")
                .help("Ignore the docker build cache")
                .num_args(0),
        )
        .arg(
            Arg::new("function")
                .long("function")
                .value_name("FUNCTION_NAME")
                .help("Specifies the function name")
                .value_parser(clap::builder::EnumValueParser::<Functions>::new())
                .num_args(1)
                .required(false),
        )
        .get_matches();

    // Check if the --push flag is present
    if matches.contains_id("push") {
        println!("--push flag is set");
    }

    // Retrieve the value of the --function argument, if provided
    let functions = if let Some(function_name) = matches.get_one::<Functions>("function") {
        vec![function_name.clone()]
    } else {
        vec![
            Functions::Fio,
            Functions::HelloWorld,
            Functions::TfInference,
        ]
    };

    build_fn_images(functions);
}
