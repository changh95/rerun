// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

#[doc = "A vector in 3D space."]
#[derive(Clone, Debug, Default, Copy, PartialEq, PartialOrd)]
pub struct Vec3D(pub [f32; 3usize]);

impl<'a> From<Vec3D> for ::std::borrow::Cow<'a, Vec3D> {
    #[inline]
    fn from(value: Vec3D) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Vec3D> for ::std::borrow::Cow<'a, Vec3D> {
    #[inline]
    fn from(value: &'a Vec3D) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Datatype for Vec3D {
    #[inline]
    fn name() -> crate::DatatypeName {
        crate::DatatypeName::Borrowed("rerun.datatypes.Vec3D")
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::FixedSizeList(
            Box::new(Field {
                name: "item".to_owned(),
                data_type: DataType::Float32,
                is_nullable: false,
                metadata: [].into(),
            }),
            3usize,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::{Component as _, Datatype as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            {
                use arrow2::{buffer::Buffer, offset::OffsetsBuffer};
                let data0_inner_data: Vec<_> = data0
                    .iter()
                    .flatten()
                    .flatten()
                    .map(ToOwned::to_owned)
                    .map(Some)
                    .collect();
                let data0_inner_bitmap: Option<::arrow2::bitmap::Bitmap> = None;
                FixedSizeListArray::new(
                    {
                        _ = extension_wrapper;
                        DataType::Extension(
                            "rerun.datatypes.Vec3D".to_owned(),
                            Box::new(DataType::FixedSizeList(
                                Box::new(Field {
                                    name: "item".to_owned(),
                                    data_type: DataType::Float32,
                                    is_nullable: false,
                                    metadata: [].into(),
                                }),
                                3usize,
                            )),
                            None,
                        )
                        .to_logical_type()
                        .clone()
                    },
                    PrimitiveArray::new(
                        {
                            _ = extension_wrapper;
                            DataType::Extension(
                                "rerun.datatypes.Vec3D".to_owned(),
                                Box::new(DataType::Float32),
                                None,
                            )
                            .to_logical_type()
                            .clone()
                        },
                        data0_inner_data
                            .into_iter()
                            .map(|v| v.unwrap_or_default())
                            .collect(),
                        data0_inner_bitmap,
                    )
                    .boxed(),
                    data0_bitmap,
                )
                .boxed()
            }
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::{Component as _, Datatype as _};
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let datatype = data.data_type();
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::FixedSizeListArray>()
                .unwrap();
            let bitmap = data.validity().cloned();
            let offsets = (0..).step_by(3usize).zip((3usize..).step_by(3usize));
            let data = &**data.values();
            let data = data
                .as_any()
                .downcast_ref::<Float32Array>()
                .unwrap()
                .into_iter()
                .map(|v| v.copied())
                .map(|v| {
                    v.ok_or_else(|| crate::DeserializationError::MissingData {
                        datatype: DataType::Float32,
                    })
                })
                .collect::<crate::DeserializationResult<Vec<_>>>()?;
            offsets
                .enumerate()
                .map(move |(i, (start, end))| {
                    bitmap
                        .as_ref()
                        .map_or(true, |bitmap| bitmap.get_bit(i))
                        .then(|| {
                            data.get(start as usize..end as usize)
                                .ok_or_else(|| crate::DeserializationError::OffsetsMismatch {
                                    bounds: (start as usize, end as usize),
                                    len: data.len(),
                                    datatype: datatype.clone(),
                                })?
                                .to_vec()
                                .try_into()
                                .map_err(|_err| crate::DeserializationError::ArrayLengthMismatch {
                                    expected: 3usize,
                                    got: (end - start) as usize,
                                    datatype: datatype.clone(),
                                })
                        })
                        .transpose()
                })
                .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?
                .into_iter()
        }
        .map(|v| {
            v.ok_or_else(|| crate::DeserializationError::MissingData {
                datatype: data.data_type().clone(),
            })
        })
        .map(|res| res.map(|v| Some(Self(v))))
        .collect::<crate::DeserializationResult<Vec<Option<_>>>>()?)
    }
}
