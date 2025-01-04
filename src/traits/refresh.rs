use bevy::asset::{Asset, AssetId, Assets};

pub trait Refresh<T> {
	fn refresh(&mut self, value: T);
}

impl<TAsset> Refresh<AssetId<TAsset>> for Assets<TAsset>
where
	TAsset: Asset,
{
	fn refresh(&mut self, id: AssetId<TAsset>) {
		/* FIXME
		 * We simply get the handle. This seems to trigger bevy to do some asset updates.
		 * This is important for our distortion shaders, which seem to be stuck with an outdated
		 * first pass image when resizing that image.
		 */
		self.get_mut(id);
	}
}
