use core::panic;

fn main() {
    let os          : String = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let out_dir     : String = std::env::var("OUT_DIR").unwrap();
    let out_ckr_dir : String = format!("{out_dir}/ckr");
    let mut c : bool = false;   
    
    eprintln!("Now Operating System: {}", std::env::var("CARGO_CFG_TARGET_OS").unwrap());
    println!("cargo:rerun-if-env-changed=CHAKRACORE_INCLUDE_DIR");
    println!("cargo:rerun-if-env-changed=CHAKRACORE_LIB_DIR");

    match std::env::var("CHAKRACORE_INCLUDE_DIR") {
        Ok( _) => {}
        Err(_) =>  { c = true; }
    }

    match std::env::var("CHAKRACORE_LIB_DIR") {
        Ok(_) => {}
        Err(_) =>  { c = true; }
    }

    if c {
        let _ = std::process::Command::new("git")
            .args(["clone", "https://github.com/chakra-core/ChakraCore", out_ckr_dir.as_str(), "--depth", "1"])
            .output()
            .expect("Failed to fetch ChakraCore: https://github.com/chakra-core/ChakraCore");
      
        build_chakracore(&out_ckr_dir, &os);

    }
   
}

/* TODO: change check_vs, need support cmd and PS. and separate about build.
 */

fn visual_studio_exists() -> bool {
    let check_vs = std::process::Command::new("vswhere")
        .args(["-latest",
            "-requires", "Microsoft.VisualStudio.Workload.NativeDesktop",
            "-property", "installationPath",])
        .output();

    match check_vs {
        Ok(o) =>{ eprintln!("{}", String::from_utf8_lossy(&o.stdout));
                  !o.stdout.is_empty()}, 
        Err(_) => false,
    }
}

fn build_chakracore(out_ckr_dir: &str, os: &str){
    match os {
        "windows" => build_windows(out_ckr_dir),
        "linux" => build_linux(out_ckr_dir),
        _ => panic!("Unsupported Operating System. {}", os),
    };
}

fn build_windows(out_ckr_dir: &str){

    match visual_studio_exists() {
        false => panic!("Doesn't exist Visual Studio \n\
            Need Visutal Studio or \n\
            winget install Microsoft.VisualStudio.2022.community \n\
            please install and reboot"),
        true => {
            let platform = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            let processor = match platform.as_str() {
                "x86_64" => "x64",
                "x86" => "Win32",
                "aarch64" => "ARM64",
                _ => panic!("Unspported Architecture. {}", platform),
            };

            let submodule_install = std::process::Command::new("git")
                .args(["submodule", "update", "--init", "--recursive"])
                .output();
            match submodule_install {
                Ok(o) => {
                    eprintln!("{}", String::from_utf8_lossy(&o.stdout));
                    println!("{}", String::from_utf8_lossy(&o.stderr));
                }
                Err(e)=> panic!("Failed to build Submodule: {}", e),
            }

            let result = std::process::Command::new("msbuild")
                .args(["Build\\Chakra.Core.sln",
                    "/p:Configuration=Release",
                    &format!("/p:Platform={}", processor),
                    "/m",
                ])
                .current_dir(out_ckr_dir)
                .output();
            match result {
                Ok(o) => {
                    eprintln!("Build Success");
                    eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                }
                Err(e) => panic!("Failed to build ChakraCore: {}", e),
        }

        }
    }
    
}

fn build_linux(out_ckr_dir: &str){
    let dst = cmake::Config::new(out_ckr_dir)
        .define("CMAKE_C_COMPILER", "clang")
        .define("CMAKE_CXX_COMPILER", "clang++")
        .define("CMAKE_ASM_COMPILER", "clang")
        .env("NUM_JOBS", "30")
        .build_target("all")
        .build()
        ;

    println!("cargo:rustc-link-search=native={}/build/", dst.display());
    println!("cargo:rustc-link-search=native={}/build/bin/ChakraCore", dst.display());
    println!("cargo:rustc-link-lib=dylib=ChakraCore");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dylib=icuuc");
    println!("cargo:rustc-link-lib=dylib=icui18n");

}
