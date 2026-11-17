//! WMK CAD Studio: logo aprovado, recursos e autoria do executavel Windows.
//! O programa continua usando as crates cadcraft da base original por compatibilidade.
use std::io::Cursor;
use std::path::PathBuf;

fn build_icon() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let bytes = include_bytes!("../../assets/wmk/logo-wmk-cad.jpg");
    let image = image::load_from_memory(bytes)?
        .resize_exact(256, 256, image::imageops::FilterType::Lanczos3);
    let mut target = Cursor::new(Vec::new());
    image.write_to(&mut target, image::ImageFormat::Png)?;
    let png = target.into_inner();
    // ICO moderno com um quadro PNG 256x256 para preservar transparencia/cores.
    let mut ico = Vec::with_capacity(22 + png.len());
    ico.extend_from_slice(&[0, 0, 1, 0, 1, 0]); // ICONDIR
    ico.extend_from_slice(&[0, 0, 0, 0]); // 256x256 (0), palette/reserved
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&32u16.to_le_bytes());
    ico.extend_from_slice(&(png.len() as u32).to_le_bytes());
    ico.extend_from_slice(&22u32.to_le_bytes());
    ico.extend_from_slice(&png);
    let path = PathBuf::from(std::env::var("OUT_DIR")?).join("wmk-cad-studio.ico");
    std::fs::write(&path, ico)?;
    Ok(path)
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/wmk/logo-wmk-cad.jpg");
    println!("cargo:rerun-if-env-changed=CADCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADCRAFT_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    // O pacote WMK nao pode ser publicado sem o icone da marca.
    // Se a imagem aprovada estiver ausente ou invalida, interromper a compilacao.
    let icon_path = match build_icon() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("ERRO WMK: nao foi possivel gerar o icone do executavel: {err}");
            std::process::exit(1);
        }
    };
    let icon_display = icon_path.to_string_lossy().into_owned();
    res.set_icon(&icon_display);
    res.set("ProductName", "WMK CAD Studio")
        .set("FileDescription", "WMK CAD Studio - Editor de desenhos tecnicos")
        .set("CompanyName", "Walney Moreira Klein - WMK")
        .set("LegalCopyright", "Personalizacao WMK: Walney Moreira Klein (walneyk@hotmail.com). Original CADCraft: ArtCraft Team e contribuidores. MIT OU Apache-2.0.")
        .set("OriginalFilename", "WMK-CAD-Studio.exe")
        .set("InternalName", "WMK-CAD-Studio");
    if let Err(e) = res.compile() {
        if std::env::var_os("CADCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("WMK: nao foi possivel incorporar recursos Windows: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=WMK CAD Studio compilado sem recursos Windows: {e}");
    }
}
