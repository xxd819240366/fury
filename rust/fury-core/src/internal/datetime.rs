// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::Error;
use crate::read_state::ReadState;
use crate::serializer::Serializer;
use crate::types::{FieldType, FuryGeneralList};
use crate::write_state::WriteState;
use chrono::{DateTime, Days, NaiveDate, NaiveDateTime};
use std::mem;

impl Serializer for NaiveDateTime {
    fn read(deserializer: &mut ReadState) -> Result<Self, Error> {
        let timestamp = deserializer.reader.u64();
        let ret = DateTime::from_timestamp_millis(timestamp as i64).map(|dt| dt.naive_utc());
        match ret {
            Some(r) => Ok(r),
            None => Err(Error::NaiveDateTime),
        }
    }

    fn write(&self, serializer: &mut WriteState) {
        serializer
            .writer
            .u64(self.and_utc().timestamp_millis() as u64);
    }

    fn reserved_space() -> usize {
        mem::size_of::<u64>()
    }

    fn ty() -> FieldType {
        FieldType::TIMESTAMP
    }
}

impl FuryGeneralList for NaiveDateTime {}

lazy_static::lazy_static!(
    static ref EPOCH: NaiveDate = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
);

impl Serializer for NaiveDate {
    fn write(&self, serializer: &mut WriteState) {
        let days_since_epoch = self.signed_duration_since(*EPOCH).num_days();
        serializer.writer.u64(days_since_epoch as u64);
    }

    fn reserved_space() -> usize {
        mem::size_of::<u64>()
    }

    fn read(serializer: &mut ReadState) -> Result<Self, Error> {
        let days = serializer.reader.u64();
        match EPOCH.checked_add_days(Days::new(days)) {
            Some(value) => Ok(value),
            None => Err(Error::NaiveDate),
        }
    }

    fn ty() -> FieldType {
        FieldType::DATE
    }
}

impl FuryGeneralList for NaiveDate {}
