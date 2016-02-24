extern crate gcc;
extern crate pkg_config;

use std::env;
use std::fs;
use std::path::Path;


fn build_libtcod_objects(mut config: gcc::Config, sources: &[&str]) {
    config.include("libtcod/include");
    config.include("libtcod/src/zlib");
    for c_file in sources {
        config.file(c_file);
    }
    config.cargo_metadata(false);
    config.compile("libtcod.a");
}


fn compile_config(config: gcc::Config) {
    let mut cmd = config.get_compiler().to_command();
    println!("Compiling: {:?}", cmd);
    match cmd.output() {
        Ok(output) => {
            println!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            println!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            if !output.status.success() {
                panic!("Compilation failed.");
            }
        }
        Err(e) => {
            panic!("Failed to run the compilation command {}.", e);
        }
    }
}


fn main() {
    let is_crater = option_env!("CRATER_TASK_TYPE");

    if is_crater.is_some() {
        return;
    }

    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dst_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    let src = Path::new(&src_dir);
    let dst = Path::new(&dst_dir);
    let sdl_lib_dir = src.join("libtcod/dependencies/SDL-1.2.15/lib").join(&target);
    let sdl_include_dir = src.join("libtcod/dependencies/SDL-1.2.15/include").join(&target);

    let libz_sources = &[
        "libtcod/src/zlib/adler32.c",
	    "libtcod/src/zlib/crc32.c",
	    "libtcod/src/zlib/deflate.c",
	    "libtcod/src/zlib/infback.c",
	    "libtcod/src/zlib/inffast.c",
	    "libtcod/src/zlib/inflate.c",
	    "libtcod/src/zlib/inftrees.c",
	    "libtcod/src/zlib/trees.c",
	    "libtcod/src/zlib/zutil.c",
	    "libtcod/src/zlib/compress.c",
	    "libtcod/src/zlib/uncompr.c",
	    "libtcod/src/zlib/gzclose.c",
	    "libtcod/src/zlib/gzlib.c",
	    "libtcod/src/zlib/gzread.c",
	    "libtcod/src/zlib/gzwrite.c",
    ];
    gcc::compile_library("libz.a", libz_sources);

    let libtcod_sources = &[
 	    "libtcod/src/bresenham_c.c",
	    "libtcod/src/bsp_c.c",
	    "libtcod/src/color_c.c",
	    "libtcod/src/console_c.c",
	    "libtcod/src/fov_c.c",
	    "libtcod/src/fov_circular_raycasting.c",
	    "libtcod/src/fov_diamond_raycasting.c",
	    "libtcod/src/fov_recursive_shadowcasting.c",
	    "libtcod/src/fov_permissive2.c",
	    "libtcod/src/fov_restrictive.c",
	    "libtcod/src/heightmap_c.c",
	    "libtcod/src/image_c.c",
	    "libtcod/src/lex_c.c",
	    "libtcod/src/list_c.c",
	    "libtcod/src/mersenne_c.c",
	    "libtcod/src/noise_c.c",
	    "libtcod/src/parser_c.c",
	    "libtcod/src/path_c.c",
	    "libtcod/src/sys_c.c",
	    "libtcod/src/sys_opengl_c.c",
	    "libtcod/src/sys_sdl_c.c",
	    "libtcod/src/sys_sdl_img_bmp.c",
	    "libtcod/src/sys_sdl_img_png.c",
	    "libtcod/src/tree_c.c",
	    "libtcod/src/txtfield_c.c",
	    "libtcod/src/wrappers.c",
	    "libtcod/src/zip_c.c",
	    "libtcod/src/namegen_c.c",
	    "libtcod/src/png/lodepng.c",
    ];

    if target.contains("linux") {
        // Build the *.o files:
        {
            let mut config = gcc::Config::new();
            config.flag("-fno-strict-aliasing");
            config.flag("-ansi");
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Config::new();
        config.flag("-shared");
        config.flag("-Wl,-soname,libtcod.so");
        config.flag("-o");
        config.flag(dst.join("libtcod.so").to_str().unwrap());
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag("-lSDL");
        config.flag("-lGL");
        config.flag("-lX11");
        config.flag("-lm");
        config.flag("-ldl");
        config.flag("-lpthread");

        compile_config(config);
        assert!(dst.join("libtcod.so").is_file());

        pkg_config::find_library("sdl").unwrap();
        pkg_config::find_library("gl").unwrap();
        pkg_config::find_library("x11").unwrap();

    } else if target.contains("darwin") {
        // Build the *.o files
        {
            let mut config = gcc::Config::new();
            config.flag("-fno-strict-aliasing");
            config.flag("-ansi");
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Config::new();
        config.flag("-shared");
        config.flag("-o");
        config.flag(dst.join("libtcod.dylib").to_str().unwrap());
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag(src.join("libtcod/osx/macsupport.m").to_str().unwrap());
        config.flag("-lSDL");
        config.flag("-lSDLmain");
        config.flag("-framework");
        config.flag("OpenGL");
        config.flag("-framework");
        config.flag("Cocoa");
        config.flag("-lm");
        config.flag("-ldl");
        config.flag("-lpthread");

        compile_config(config);
        assert!(dst.join("libtcod.dylib").is_file());

        pkg_config::find_library("sdl").unwrap();
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");

    } else if target.contains("windows-gnu") {
        assert!(sdl_lib_dir.is_dir());
        assert!(sdl_include_dir.is_dir());
        fs::copy(&sdl_lib_dir.join("SDL.dll"), &dst.join("SDL.dll")).unwrap();

        // Build the *.o files:
        {
            let mut config = gcc::Config::new();
            config.include(sdl_include_dir.to_str().unwrap());
            config.flag("-fno-strict-aliasing");
            config.flag("-ansi");
            config.define("LIBTCOD_EXPORTS", None);
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Config::new();
        config.flag("-o");
        config.flag(dst.join("libtcod.dll").to_str().unwrap());
        config.flag("-shared");
        fs::create_dir(dst.join("lib")).unwrap();
        config.flag(&format!("-Wl,--out-implib,{}", dst.join("lib/libtcod.a").display()));
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag("-mwindows");
        config.flag("-L");
        config.flag(sdl_lib_dir.to_str().unwrap());
        config.flag("-lSDL.dll");
        config.flag("-lopengl32");
        config.flag("-static-libgcc");
        config.flag("-static-libstdc++");

        compile_config(config);
        assert!(dst.join("libtcod.dll").is_file());

        println!("cargo:rustc-link-lib=dylib={}", "SDL.dll");
        println!("cargo:rustc-link-lib=dylib={}", "opengl32");
        println!("cargo:rustc-link-search=native={}", sdl_lib_dir.display());
        println!("cargo:rustc-link-search=native={}", dst.display());

    } else if target.contains("windows-msvc") {
        assert!(sdl_lib_dir.is_dir());
        assert!(sdl_include_dir.is_dir());
        fs::copy(&sdl_lib_dir.join("SDL.dll"), &dst.join("SDL.dll")).unwrap();
        fs::copy(&sdl_lib_dir.join("SDL.lib"), &dst.join("SDL.dll.lib")).unwrap();

        // Build the *.o files
        {
            let mut config = gcc::Config::new();
            config.include(sdl_include_dir.to_str().unwrap());
            config.define("LIBTCOD_EXPORTS", None);
            config.define("NO_OPENGL", None);
            build_libtcod_objects(config, libtcod_sources);
        }

        // Build the DLL
        let mut config = gcc::Config::new();
        for c_file in libtcod_sources {
            config.flag(dst.join(c_file).with_extension("o").to_str().unwrap());
        }
        config.flag(dst.join("libz.a").to_str().unwrap());
        config.flag("User32.lib");
        config.flag("SDL.dll.lib");
        config.flag("/LD");
        config.flag("/link");
        config.flag(&format!("/LIBPATH:{}", dst.to_str().unwrap()));
        config.flag("/DLL");
        config.flag(&format!("/OUT:{}", dst.join("libtcod.dll").display()));

        compile_config(config);
        assert!(dst.join("libtcod.dll").is_file());

        println!("cargo:rustc-link-search={}", dst.display());
        println!("cargo:rustc-link-lib=dylib=SDL.dll");
        println!("cargo:rustc-link-lib=User32");
    }

    println!("cargo:rustc-link-search={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=tcod");
}
