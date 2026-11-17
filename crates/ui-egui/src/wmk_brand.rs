//! Identidade WMK no editor. Textura criada apenas uma vez por contexto egui.
//! Utiliza a imagem aprovada do repositorio sem depender da rede.
use egui::{ColorImage, Context, Id, Rect, TextureHandle, TextureOptions, Vec2, pos2};

fn texture(ctx: &Context) -> Option<TextureHandle> {
    let key = Id::new("wmk_cad_logo_textura_v1");
    if let Some(existing) = ctx.data(|d| d.get_temp::<TextureHandle>(key)) {
        return Some(existing);
    }
    let bytes = include_bytes!("../../../assets/wmk/logo-wmk-cad.jpg");
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let ci = ColorImage::from_rgba_unmultiplied(size, image.as_raw());
    let handle = ctx.load_texture("WMK CAD Studio", ci, TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(key, handle.clone()));
    Some(handle)
}

/// Pinta a logo real aprovada pelo usuario em uma area definida.
pub(crate) fn paint(ctx: &Context, painter: &egui::Painter, where_: Rect) {
    if let Some(logo) = texture(ctx) {
        painter.image(
            logo.id(),
            where_,
            Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }
}

/// Desenha o logo em paineis e janelas com layout automatico.
pub(crate) fn show(ui: &mut egui::Ui, size: Vec2) {
    if let Some(logo) = texture(ui.ctx()) {
        ui.image((logo.id(), size));
    }
}
