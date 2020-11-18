#[cfg(feature = "use-bindgen")]
use bindgen;
#[cfg(feature = "use-bindgen")]
use std::env;

#[cfg(feature = "use-bindgen")]
use std::path::PathBuf;

#[cfg(feature = "use-bindgen")]
fn main() {
    let libnds_include: PathBuf = [&env::var("DEVKITPRO").unwrap(), "libnds", "include"]
        .iter()
        .collect();

    let libnds_system_include: PathBuf =
        [&env::var("DEVKITARM").unwrap(), "arm-none-eabi", "include"]
            .iter()
            .collect();
	let dkparm_path: PathBuf =
		[&env::var("DEVKITARM").unwrap(), "arm-none-eabi"]
			.iter()
			.collect();

    let bindings = bindgen::Builder::default()
		.header("wrapper.h")
        .trust_clang_mangling(false)
		.layout_tests(false)
		.disable_untagged_union()
        .use_core()
        .ctypes_prefix("libc")
        .clang_arg("-DARM9")
        .clang_arg("-DNDEBUG")
		.clang_arg("--sysroot=".to_owned() + dkparm_path.to_str().unwrap())
        .clang_arg("-isystem".to_owned() + libnds_include.to_str().unwrap())
        .clang_arg("-isystem".to_owned() + libnds_system_include.to_str().unwrap())
		.clang_arg("-march=thumbv5te")
		.clang_arg("--target=arm-none-eabi")
		.clang_arg("-Wno-macro-redefined")
		.clang_arg("-Wno-incompatible-library-redeclaration")
		.clang_arg("-DHW_RVL")
        .prepend_enum_name(false)
        .blacklist_type("u(8|16|32|64)")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings!");

    let out_path = PathBuf::from("C:/dev/nds-rs/nds-sys/src");
    bindings
        .write_to_file(out_path.join("nds.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(not(feature = "use-bindgen"))]
fn main() {

	/*let dkp_path = std::env::var("DEVKITPRO").unwrap();

	println!(
		"cargo:rustc-link-search=native={}/devkitARM/arm-none-eabi/lib",
		dkp_path
	);
	println!("cargo:rustc-link-search=native={}/libnds/lib", dkp_path);
	println!("cargo:rustc-link-lib=static=c");
	println!("cargo:rustc-link-lib=static=sysbase");
	println!("cargo:rustc-link-lib=static=nds9");
	println!("cargo:rustc-link-lib=static=filesystem");
	println!("cargo:rustc-link-lib=static=fat");
	println!("cargo:rustc-link-lib=static=mm9");*/
}
