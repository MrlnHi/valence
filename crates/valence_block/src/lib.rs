#![doc = include_str!("../README.md")]
#![allow(clippy::all)] // TODO: block build script creates many warnings.
#![deny(
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    rustdoc::invalid_html_tags
)]
#![warn(
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    unreachable_pub,
    clippy::dbg_macro
)]

use std::fmt;
use std::fmt::Display;
use std::io::Write;
use std::iter::FusedIterator;
use std::str::FromStr;

use thiserror::Error;
use anyhow::Context;
use valence_core::ident;
use valence_core::ident::Ident;
use valence_core::item::ItemKind;
use valence_core::packet::var_int::VarInt;
use valence_core::packet::{Decode, Encode};

include!(concat!(env!("OUT_DIR"), "/block.rs"));

impl fmt::Debug for BlockState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_block_state(*self, f)
    }
}

impl Display for BlockState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt_block_state(*self, f)
    }
}

fn fmt_block_state(bs: BlockState, f: &mut fmt::Formatter) -> fmt::Result {
    let kind = bs.to_kind();

    write!(f, "{}", kind.to_str())?;

    let props = kind.props();

    if !props.is_empty() {
        let mut list = f.debug_list();
        for &p in kind.props() {
            struct KeyVal<'a>(&'a str, &'a str);

            impl<'a> fmt::Debug for KeyVal<'a> {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "{}={}", self.0, self.1)
                }
            }

            list.entry(&KeyVal(p.to_str(), bs.get(p).unwrap().to_str()));
        }
        list.finish()
    } else {
        Ok(())
    }
}

impl Encode for BlockState {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        VarInt(self.to_raw() as i32).encode(w)
    }
}

impl Decode<'_> for BlockState {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let id = VarInt::decode(r)?.0;
        let errmsg = "invalid block state ID";

        BlockState::from_raw(id.try_into().context(errmsg)?).context(errmsg)
    }
}

impl Encode for BlockKind {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        VarInt(self.to_raw() as i32).encode(w)
    }
}

impl Decode<'_> for BlockKind {
    fn decode(r: &mut &[u8]) -> anyhow::Result<Self> {
        let id = VarInt::decode(r)?.0;
        let errmsg = "invalid block kind ID";

        BlockKind::from_raw(id.try_into().context(errmsg)?).context(errmsg)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseBlockStateError {
    #[error("unknown block kind '{0}'")]
    UnknownBlockKind(String),
    #[error("invalid prop string '{0}'")]
    InvalidPropString(String),
    #[error("unknown prop name '{0}'")]
    UnknownPropName(String),
    #[error("unknown prop value '{0}'")]
    UnknownPropValue(String),
}

impl FromStr for BlockState {
    type Err = ParseBlockStateError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let state = match s.split_once('[') {
            Some((kind, props)) => {
                let Some(kind) = BlockKind::from_str(kind) else {
                    return Err(ParseBlockStateError::UnknownBlockKind(kind.to_string()));
                };
                props[..props.len() - 1]
                    .split(',')
                    .map(|prop| prop.trim())
                    .try_fold(kind.to_state(), |state, prop| {
                        let Some((name, val)) = prop.split_once('=') else {
                            return Err(ParseBlockStateError::InvalidPropString(prop.to_string()));
                        };
                        let Some(name) = PropName::from_str(name) else {
                            return Err(ParseBlockStateError::UnknownPropName(name.to_string()));
                        };
                        let Some(val) = PropValue::from_str(val) else {
                            return Err(ParseBlockStateError::UnknownPropValue(val.to_string()));
                        };
                        Ok(state.set(name, val))
                    })?
            }
            None => match BlockKind::from_str(s) {
                Some(kind) => kind.to_state(),
                None => return Err(ParseBlockStateError::UnknownBlockKind(s.to_string())),
            },
        };

        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_consistency() {
        for kind in BlockKind::ALL {
            let block = kind.to_state();

            for &prop in kind.props() {
                let new_block = block.set(prop, block.get(prop).unwrap());
                assert_eq!(new_block, block);
            }
        }
    }

    #[test]
    fn blockstate_to_wall() {
        assert_eq!(BlockState::STONE.wall_block_id(), None);
        assert_eq!(
            BlockState::OAK_SIGN.wall_block_id(),
            Some(BlockState::OAK_WALL_SIGN)
        );
        assert_eq!(
            BlockState::GREEN_BANNER.wall_block_id(),
            Some(BlockState::GREEN_WALL_BANNER)
        );
        assert_ne!(
            BlockState::GREEN_BANNER.wall_block_id(),
            Some(BlockState::GREEN_BANNER)
        );
    }
}