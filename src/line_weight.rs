// Copyright (c) IxMilia.  All Rights Reserved.  Licensed under the Apache License, Version 2.0.  See License.txt in the project root for license information.

/// Represents a line weight.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LineWeight {
    raw_value: i16,
}

impl LineWeight {
    /// Creates a new `LineWeight`.
    pub fn new() -> LineWeight {
        LineWeight::from_raw_value(0)
    }
    #[doc(hidden)]
    pub fn from_raw_value(v: i16) -> LineWeight {
        LineWeight { raw_value: v }
    }
    /// Creates a new `LineWeight` that defaults back to the containing block's line weight.
    pub fn by_block() -> LineWeight {
        LineWeight::from_raw_value(-1)
    }
    /// Creates a new `LineWeight` that defaults back to the item's layer's line weight.
    pub fn by_layer() -> LineWeight {
        LineWeight::from_raw_value(-2)
    }
    /// Gets the raw value of the `LineWeight`.
    pub fn get_raw_value(&self) -> i16 {
        self.raw_value
    }
    /// Returns `true` if the `LineWeight` is BYBLOCK.
    pub fn is_by_block(&self) -> bool {
        self.raw_value == -1
    }
    /// Returns `true` if the `LineWeight` is BYLAYER.
    pub fn is_by_layer(&self) -> bool {
        self.raw_value == -2
    }
}