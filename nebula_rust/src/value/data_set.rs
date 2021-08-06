/* Copyright (c) 2021 vesoft inc. All rights reserved.
 *
 * This source code is licensed under Apache 2.0 License,
 * attached with Common Clause Condition 1.0, found in the LICENSES directory.
 */

use common::types::DataSet;

pub trait DataSetValue {
    /// Construct data set with name of columns
    fn new(col_names: &[String]) -> Self;

    /// Construct data set from vec of columns name
    fn from_columns_name(col_names: std::vec::Vec<String>) -> Self;

    /// push one row into back of data set
    fn push(&mut self, row: common::types::Row);

    /// Get rows size
    fn len(&self) -> usize;
}

impl DataSetValue for DataSet {
    fn new(col_names: &[String]) -> Self {
        let cols = col_names.to_vec();
        let cols_bytes = cols.into_iter().map(|s| s.as_bytes().to_vec()).collect();
        DataSet {
            column_names: cols_bytes,
            rows: vec![],
        }
    }

    fn from_columns_name(col_names: std::vec::Vec<String>) -> Self {
        let cols_bytes = col_names
            .into_iter()
            .map(|s| s.as_bytes().to_vec())
            .collect();
        DataSet {
            column_names: cols_bytes,
            rows: vec![],
        }
    }

    #[inline]
    fn push(&mut self, row: common::types::Row) {
        self.rows.push(row);
    }

    #[inline]
    fn len(&self) -> usize {
        self.rows.len()
    }
}
