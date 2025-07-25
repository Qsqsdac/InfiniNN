﻿use super::Context;
use crate::{NNError, macros::destruct};
use arg::{Arg, Dim};
use tensor::digit_layout::DigitLayout;

/// 计算图层张量
pub struct Tensor<T> {
    pub(super) idx: usize,
    pub(super) ctx: Context<T>,
}

impl<T> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        Self {
            idx: self.idx,
            ctx: self.ctx.clone(),
        }
    }
}

impl<T> Tensor<T> {
    #[inline]
    pub fn dt(&self) -> DigitLayout {
        self.meta().dt
    }

    #[inline]
    pub fn shape(&self) -> Box<[Dim]> {
        self.meta().shape.clone()
    }

    fn meta(&self) -> TensorMeta {
        self.ctx.get_meta(self.idx)
    }
}

impl<T> Tensor<T> {
    pub fn split(
        self,
        name: impl ToString,
        axis: usize,
        parts: impl IntoIterator<Item = Dim>,
    ) -> Result<Vec<Tensor<T>>, NNError> {
        self.ctx.clone().call(
            name,
            "split",
            Some(Arg::dict([
                ("axis".into(), Arg::int(axis)),
                ("parts".into(), Arg::arr(parts.into_iter().map(Arg::from))),
            ])),
            [self],
        )
    }

    pub fn tile(
        self,
        name: impl ToString,
        axis: usize,
        parts: impl IntoIterator<Item = Dim>,
    ) -> Result<Tensor<T>, NNError> {
        destruct!(
            [ans] = self.ctx.clone().call(
                name,
                "tile",
                Some(Arg::dict([
                    ("axis".into(), Arg::int(axis)),
                    ("tile".into(), Arg::arr(parts.into_iter().map(Arg::from))),
                ])),
                [self],
            )?
        );
        Ok(ans)
    }

    pub fn merge(
        self,
        name: impl ToString,
        start: usize,
        len: usize,
    ) -> Result<Tensor<T>, NNError> {
        destruct!(
            [ans] = self.ctx.clone().call(
                name,
                "merge",
                Some(Arg::dict([
                    ("start".into(), Arg::int(start)),
                    ("len".into(), Arg::int(len)),
                ])),
                [self],
            )?
        );
        Ok(ans)
    }
}

#[derive(Clone)]
pub struct TensorMeta {
    pub dt: DigitLayout,
    pub shape: Box<[Dim]>,
}

impl TensorMeta {
    pub fn new(dt: DigitLayout, shape: impl IntoIterator<Item = Dim>) -> Self {
        let mut shape = shape.into_iter().collect::<Box<_>>();
        let group = dt.group_size();
        if group > 1 {
            if let Some(dim) = shape.last_mut() {
                *dim = std::mem::replace(dim, Dim::from(0)) / group
            }
        }
        Self { dt, shape }
    }

    #[inline]
    pub const fn dt(&self) -> DigitLayout {
        self.dt
    }

    #[inline]
    pub fn shape(&self) -> &[Dim] {
        &self.shape
    }
}
