#[derive(Debug, PartialEq, Clone)]
pub enum RenderPass {
	FirstPass,
	SecondPass,
}

pub trait GetMaterialRenderPass {
	fn render_pass() -> RenderPass;
}
