use crate::api::ResourceCategory;

/// The type of a resource.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    /// Resources (`.json`) in `assets/<namespace>/blockstates/`.
    BlockStates,

    /// Resources (`.json`) in `assets/<namespace>/models/block/`.
    BlockModel,

    /// Resources (`.json`) in `assets/<namespace>/models/item/`.
    ItemModel,

    /// Resources (`.png`) in `assets/<namespace>/textures/`.
    Texture,

    /// Resources (`.png.mcmeta`) in `assets/<namespace>/textures/`.
    TextureMeta,

    /// Resources (`.json`) in `data/<namespace>/worldgen/biome/`.
    WorldGen_Biome,
}

impl ResourceKind {
    /// Returns the category of this resource type (assets or data).
    pub fn category(&self) -> ResourceCategory {
        match self {
            Self::BlockStates
            | Self::BlockModel
            | Self::ItemModel
            | Self::Texture
            | Self::TextureMeta => ResourceCategory::Assets,
            Self::WorldGen_Biome => ResourceCategory::Data,
        }
    }

    /// Returns the file extension used for this resource's file.
    ///
    /// # Example
    ///
    /// ```
    /// # use minecraft_assets::api::*;
    /// let kind = ResourceKind::BlockStates;
    /// assert_eq!(kind.extension(), "json");
    ///
    /// let kind = ResourceKind::Texture;
    /// assert_eq!(kind.extension(), "png");
    ///
    /// let kind = ResourceKind::TextureMeta;
    /// assert_eq!(kind.extension(), "png.mcmeta");
    /// ```
    pub fn extension(&self) -> &'static str {
        match self {
            Self::BlockStates | Self::BlockModel | Self::ItemModel | Self::WorldGen_Biome => "json",
            Self::Texture => "png",
            Self::TextureMeta => "png.mcmeta",
        }
    }

    /// Returns the path relative to `assets/<namespace>/` or
    /// `data/<namespace>/` in which resources of this type reside.
    pub fn directory(&self) -> &'static str {
        match self {
            Self::BlockStates => "blockstates",
            Self::BlockModel => "models/block",
            Self::ItemModel => "models/item",
            Self::Texture | Self::TextureMeta => "textures",
            Self::WorldGen_Biome => "worldgen/biome",
        }
    }
}
