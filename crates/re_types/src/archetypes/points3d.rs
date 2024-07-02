// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/archetypes/points3d.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

use ::re_types_core::external::arrow2;
use ::re_types_core::ComponentName;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, MaybeOwnedComponentBatch};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: A 3D point cloud with positions and optional colors, radii, labels, etc.
///
/// ## Examples
///
/// ### Randomly distributed 3D points with varying color and radius
/// ```ignore
/// use rand::{distributions::Uniform, Rng as _};
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_points3d_random").spawn()?;
///
///     let mut rng = rand::thread_rng();
///     let dist = Uniform::new(-5., 5.);
///
///     rec.log(
///         "random",
///         &rerun::Points3D::new(
///             (0..10).map(|_| (rng.sample(dist), rng.sample(dist), rng.sample(dist))),
///         )
///         .with_colors((0..10).map(|_| rerun::Color::from_rgb(rng.gen(), rng.gen(), rng.gen())))
///         .with_radii((0..10).map(|_| rng.gen::<f32>())),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/point3d_random/7e94e1806d2c381943748abbb3bedb68d564de24/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/point3d_random/7e94e1806d2c381943748abbb3bedb68d564de24/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/point3d_random/7e94e1806d2c381943748abbb3bedb68d564de24/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/point3d_random/7e94e1806d2c381943748abbb3bedb68d564de24/1200w.png">
///   <img src="https://static.rerun.io/point3d_random/7e94e1806d2c381943748abbb3bedb68d564de24/full.png" width="640">
/// </picture>
/// </center>
///
/// ### Log points with radii given in UI points
/// ```ignore
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let rec = rerun::RecordingStreamBuilder::new("rerun_example_points3d_ui_radius").spawn()?;
///
///     // Two blue points with scene unit radii of 0.1 and 0.3.
///     rec.log(
///         "scene_units",
///         &rerun::Points3D::new([(0.0, 1.0, 0.0), (1.0, 1.0, 1.0)])
///             // By default, radii are interpreted as world-space units.
///             .with_radii([0.1, 0.3])
///             .with_colors([rerun::Color::from_rgb(0, 0, 255)]),
///     )?;
///
///     // Two red points with ui point radii of 40 and 60.
///     // UI points are independent of zooming in Views, but are sensitive to the application UI scaling.
///     // For 100% ui scaling, UI points are equal to pixels.
///     rec.log(
///         "ui_points",
///         &rerun::Points3D::new([(0.0, 0.0, 0.0), (1.0, 0.0, 1.0)])
///             // rerun::Radius::new_ui_points produces a radius that the viewer interprets as given in ui points.
///             .with_radii([
///                 rerun::Radius::new_ui_points(40.0),
///                 rerun::Radius::new_ui_points(60.0),
///             ])
///             .with_colors([rerun::Color::from_rgb(255, 0, 0)]),
///     )?;
///
///     Ok(())
/// }
/// ```
/// <center>
/// <picture>
///   <source media="(max-width: 480px)" srcset="https://static.rerun.io/point3d_ui_radius/e051a65b4317438bcaea8d0eee016ac9460b5336/480w.png">
///   <source media="(max-width: 768px)" srcset="https://static.rerun.io/point3d_ui_radius/e051a65b4317438bcaea8d0eee016ac9460b5336/768w.png">
///   <source media="(max-width: 1024px)" srcset="https://static.rerun.io/point3d_ui_radius/e051a65b4317438bcaea8d0eee016ac9460b5336/1024w.png">
///   <source media="(max-width: 1200px)" srcset="https://static.rerun.io/point3d_ui_radius/e051a65b4317438bcaea8d0eee016ac9460b5336/1200w.png">
///   <img src="https://static.rerun.io/point3d_ui_radius/e051a65b4317438bcaea8d0eee016ac9460b5336/full.png" width="640">
/// </picture>
/// </center>
#[derive(Clone, Debug, PartialEq)]
pub struct Points3D {
    /// All the 3D positions at which the point cloud shows points.
    pub positions: Vec<crate::components::Position3D>,

    /// Optional radii for the points, effectively turning them into circles.
    pub radii: Option<Vec<crate::components::Radius>>,

    /// Optional colors for the points.
    pub colors: Option<Vec<crate::components::Color>>,

    /// Optional text labels for the points.
    pub labels: Option<Vec<crate::components::Text>>,

    /// Optional class Ids for the points.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    pub class_ids: Option<Vec<crate::components::ClassId>>,

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    pub keypoint_ids: Option<Vec<crate::components::KeypointId>>,
}

impl ::re_types_core::SizeBytes for Points3D {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.positions.heap_size_bytes()
            + self.radii.heap_size_bytes()
            + self.colors.heap_size_bytes()
            + self.labels.heap_size_bytes()
            + self.class_ids.heap_size_bytes()
            + self.keypoint_ids.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <Vec<crate::components::Position3D>>::is_pod()
            && <Option<Vec<crate::components::Radius>>>::is_pod()
            && <Option<Vec<crate::components::Color>>>::is_pod()
            && <Option<Vec<crate::components::Text>>>::is_pod()
            && <Option<Vec<crate::components::ClassId>>>::is_pod()
            && <Option<Vec<crate::components::KeypointId>>>::is_pod()
    }
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 1usize]> =
    once_cell::sync::Lazy::new(|| ["rerun.components.Position3D".into()]);

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.Points3DIndicator".into(),
        ]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 3usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Text".into(),
            "rerun.components.ClassId".into(),
            "rerun.components.KeypointId".into(),
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentName; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            "rerun.components.Position3D".into(),
            "rerun.components.Radius".into(),
            "rerun.components.Color".into(),
            "rerun.components.Points3DIndicator".into(),
            "rerun.components.Text".into(),
            "rerun.components.ClassId".into(),
            "rerun.components.KeypointId".into(),
        ]
    });

impl Points3D {
    /// The total number of components in the archetype: 1 required, 3 recommended, 3 optional
    pub const NUM_COMPONENTS: usize = 7usize;
}

/// Indicator component for the [`Points3D`] [`::re_types_core::Archetype`]
pub type Points3DIndicator = ::re_types_core::GenericIndicatorComponent<Points3D>;

impl ::re_types_core::Archetype for Points3D {
    type Indicator = Points3DIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.archetypes.Points3D".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Points 3D"
    }

    #[inline]
    fn indicator() -> MaybeOwnedComponentBatch<'static> {
        static INDICATOR: Points3DIndicator = Points3DIndicator::DEFAULT;
        MaybeOwnedComponentBatch::Ref(&INDICATOR)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentName]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let positions = {
            let array = arrays_by_name
                .get("rerun.components.Position3D")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.archetypes.Points3D#positions")?;
            <crate::components::Position3D>::from_arrow_opt(&**array)
                .with_context("rerun.archetypes.Points3D#positions")?
                .into_iter()
                .map(|v| v.ok_or_else(DeserializationError::missing_data))
                .collect::<DeserializationResult<Vec<_>>>()
                .with_context("rerun.archetypes.Points3D#positions")?
        };
        let radii = if let Some(array) = arrays_by_name.get("rerun.components.Radius") {
            Some({
                <crate::components::Radius>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points3D#radii")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points3D#radii")?
            })
        } else {
            None
        };
        let colors = if let Some(array) = arrays_by_name.get("rerun.components.Color") {
            Some({
                <crate::components::Color>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points3D#colors")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points3D#colors")?
            })
        } else {
            None
        };
        let labels = if let Some(array) = arrays_by_name.get("rerun.components.Text") {
            Some({
                <crate::components::Text>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points3D#labels")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points3D#labels")?
            })
        } else {
            None
        };
        let class_ids = if let Some(array) = arrays_by_name.get("rerun.components.ClassId") {
            Some({
                <crate::components::ClassId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points3D#class_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points3D#class_ids")?
            })
        } else {
            None
        };
        let keypoint_ids = if let Some(array) = arrays_by_name.get("rerun.components.KeypointId") {
            Some({
                <crate::components::KeypointId>::from_arrow_opt(&**array)
                    .with_context("rerun.archetypes.Points3D#keypoint_ids")?
                    .into_iter()
                    .map(|v| v.ok_or_else(DeserializationError::missing_data))
                    .collect::<DeserializationResult<Vec<_>>>()
                    .with_context("rerun.archetypes.Points3D#keypoint_ids")?
            })
        } else {
            None
        };
        Ok(Self {
            positions,
            radii,
            colors,
            labels,
            class_ids,
            keypoint_ids,
        })
    }
}

impl ::re_types_core::AsComponents for Points3D {
    fn as_component_batches(&self) -> Vec<MaybeOwnedComponentBatch<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            Some((&self.positions as &dyn ComponentBatch).into()),
            self.radii
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.colors
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.labels
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.class_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
            self.keypoint_ids
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch).into()),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl Points3D {
    /// Create a new `Points3D`.
    #[inline]
    pub fn new(
        positions: impl IntoIterator<Item = impl Into<crate::components::Position3D>>,
    ) -> Self {
        Self {
            positions: positions.into_iter().map(Into::into).collect(),
            radii: None,
            colors: None,
            labels: None,
            class_ids: None,
            keypoint_ids: None,
        }
    }

    /// Optional radii for the points, effectively turning them into circles.
    #[inline]
    pub fn with_radii(
        mut self,
        radii: impl IntoIterator<Item = impl Into<crate::components::Radius>>,
    ) -> Self {
        self.radii = Some(radii.into_iter().map(Into::into).collect());
        self
    }

    /// Optional colors for the points.
    #[inline]
    pub fn with_colors(
        mut self,
        colors: impl IntoIterator<Item = impl Into<crate::components::Color>>,
    ) -> Self {
        self.colors = Some(colors.into_iter().map(Into::into).collect());
        self
    }

    /// Optional text labels for the points.
    #[inline]
    pub fn with_labels(
        mut self,
        labels: impl IntoIterator<Item = impl Into<crate::components::Text>>,
    ) -> Self {
        self.labels = Some(labels.into_iter().map(Into::into).collect());
        self
    }

    /// Optional class Ids for the points.
    ///
    /// The class ID provides colors and labels if not specified explicitly.
    #[inline]
    pub fn with_class_ids(
        mut self,
        class_ids: impl IntoIterator<Item = impl Into<crate::components::ClassId>>,
    ) -> Self {
        self.class_ids = Some(class_ids.into_iter().map(Into::into).collect());
        self
    }

    /// Optional keypoint IDs for the points, identifying them within a class.
    ///
    /// If keypoint IDs are passed in but no class IDs were specified, the class ID will
    /// default to 0.
    /// This is useful to identify points within a single classification (which is identified
    /// with `class_id`).
    /// E.g. the classification might be 'Person' and the keypoints refer to joints on a
    /// detected skeleton.
    #[inline]
    pub fn with_keypoint_ids(
        mut self,
        keypoint_ids: impl IntoIterator<Item = impl Into<crate::components::KeypointId>>,
    ) -> Self {
        self.keypoint_ids = Some(keypoint_ids.into_iter().map(Into::into).collect());
        self
    }
}
