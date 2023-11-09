use std::env::current_dir;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::utils::{
    clear, CARGO_FEATURES_PLACEHOLDER, CARGO_TOML_PLACEHOLDER, DEFAULT_CONFIG, EMPTY_APP,
    PERFORMANCE_CONFIG, SIMPLE_2D, SIMPLE_3D, SIZE_CONFIG,
};
use inquire::error::InquireResult;
use inquire::{Confirm, MultiSelect, Select, Text};
use log::info;

#[inline(always)]
pub fn init() -> InquireResult<()> {
    let name = Text::new("Project Name").prompt()?;

    let template = Select::new(
        "Project Template",
        vec!["Empty App", "Simple 3D", "Simple 2D"],
    )
    .prompt()?;

    let config = Select::new("Project Config", vec!["Default", "Size", "Performance"]).prompt()?;

    let features = MultiSelect::new(
        "Bevy Features",
        vec![
            /* 0 */ "multi-threaded",
            /* 1 */ "bevy_asset",
            /* 2 */ "bevy_audio",
            /* 3 */ "bevy_gilrs", // disabled by default
            /* 4 */ "bevy_scene",
            /* 5 */ "bevy_winit",
            /* 6 */ "bevy_render",
            /* 7 */ "bevy_core_pipeline",
            /* 8 */ "bevy_gizmos",
            /* 9 */ "bevy_sprite",
            /* 10 */ "bevy_pbr",
            /* 11 */ "bevy_gltf",
            /* 12 */ "bevy_text",
            /* 13 */ "bevy_ui",
            /* 14 */ "animation",
            /* 15 */ "tonemapping_luts",
            /* 16 */ "pbr_transmission_textures", // disabled by default
            /* 17 */ "default_font",
            /* 18 */ "png",
            /* 19 */ "hdr", // disabled by default
            /* 20 */ "ktx2",
            /* 21 */ "vorbis",
            /* 22 */ "subpixel_glyph_atlas",
            /* 23 */ "serialize",
            /* 24 */ "dds", // disabled by default
            /* 25 */ "jpeg",
            /* 26 */ "webp", // disabled by default
            /* 27 */ "bmp", // disabled by default
            /* 28 */ "tga", // disabled by default
            /* 29 */ "exr", // disabled by default
            /* 30 */ "pnm", // disabled by default
            /* 31 */ "basis-universal", // disabled by default
            /* 32 */ "zlib",
            /* 33 */ "flac", // disabled by default
            /* 34 */ "mp3",
            /* 35 */ "wav",
            /* 36 */ "symphonia-all", // disabled by default
            /* 37 */ "shader_format_glsl", // disabled by default
            /* 38 */ "shader_format_spirv", // disabled by default
            /* 39 */ "x11",
            /* 40 */ "android_shared_stdcxx", // disabled by default
            /* 41 */ "webgl2", // disabled by default
            /* 42 */ "wayland",
            /* 43 */ "accesskit_unix", // disabled by default
            /* 44 */ "bevy_dynamic_plugin", // disabled by default
            /* 45 */ "dynamic_linking", // disabled by default
            /* 46 */ "trace", // disabled by default
            /* 47 */ "detailed_trace", // disabled by default
            /* 48 */ "trace_tracy", // disabled by default
            /* 49 */ "trace_tracy_memory", // disabled by default
            /* 50 */ "trace_chrome", // disabled by default
            /* 51 */ "wgpu_trace", // disabled by default
            /* 52 */ "asset_processor", // disabled by default
            /* 53 */ "async-io", // disabled by default
            /* 54 */ "bevy_ci_testing", // disabled by default
            /* 55 */ "asset_processor", // disabled by default
            /* 56 */ "debug_glam_assert", // disabled by default
            /* 57 */ "embedded_watcher", // disabled by default
            /* 58 */ "fire_watcher", // disabled by default
            /* 59 */ "glam_assert", // disabled by default
            /* 60 */ "glam_assert", // disabled by default
            /* 61 */ "minimp3", // disabled by default
        ],
    )
    .with_default(&[
        0, 1, 2, // 3,
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, // 16,
        17, 18, // 19,
        20, 21, 22, 23, // 24,
        25,
        // 26,
        // 27,
        // 28,
        // 29,
        // 30,
        // 31,
        32, // 33,
        34, 35, // 36,
        // 37,
        // 38,
        39,
        // 40,
        // 41,
        42,
        // 43,
        // 44,
        // 45,
        // 46,
        // 47,
        // 48,
        // 49,
        // 50,
        // 51,
        // 52,
        // 53,
        // 54,
        // 55,
        // 56,
        // 57,
        // 58,
        // 59,
        // 60,
        // 61,
    ])
    .prompt()?;

    clear();

    info!("Project Name: {}", name);
    info!("Project Template: {}", template);
    info!("Project Config: {}", config);
    info!("Bevy Features: {:#?}", features);

    if Confirm::new("Do you want to continue?").prompt()? {
        clear();
        info!("Generating...");

        let project_path = current_dir()
            .expect("Could not get current directory")
            .join(name);

        // generate project
        fs::create_dir(project_path.clone()).expect("Could not create directory");

        let cargo_config = match config {
            "Default" => DEFAULT_CONFIG,
            "Size" => SIZE_CONFIG,
            "Performance" => PERFORMANCE_CONFIG,
            _ => panic!("Unknown config!"),
        };

        let cargo_config_path = project_path.clone().join(".cargo");

        fs::create_dir(cargo_config_path.clone()).expect("Could not create directory");

        File::create(cargo_config_path.join("config.toml"))
            .expect("Could not create file")
            .write_all(cargo_config)
            .expect("Could not write file");

        // extract template
        match template {
            "Empty App" => {
                EMPTY_APP.extract(project_path.clone())?;
            }
            "Simple 3D" => {
                SIMPLE_3D.extract(project_path.clone())?;
            }
            "Simple 2D" => {
                SIMPLE_2D.extract(project_path.clone())?;
            }
            _ => panic!("Unknown template!"),
        }

        let cargo_toml = project_path.clone().join("Cargo.toml");

        // rename _cargo_.toml to Cargo.toml
        fs::rename(
            project_path.clone().join(CARGO_TOML_PLACEHOLDER),
            cargo_toml.clone(),
        )?;

        // apply features
        {
            let replaced = fs::read_to_string(cargo_toml.clone())?.replace(
                CARGO_FEATURES_PLACEHOLDER,
                format!("{:#?}", features).as_str(),
            );
            fs::write(cargo_toml, replaced)?;
        }
    } else {
        info!("Cancelled! Exiting...");
    }

    Ok(())
}
