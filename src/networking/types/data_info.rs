//! Module defining the `DataInfo` struct, which represents incoming and outgoing packets and bytes.

use chrono::{DateTime, Local};
use std::ops::AddAssign;

use crate::networking::types::traffic_direction::TrafficDirection;

/// Amount of exchanged data (packets and bytes) incoming and outgoing, with the timestamp of the latest occurrence
// data fields are private to make them only editable via the provided methods: needed to correctly refresh timestamps
#[derive(Clone, Default, Copy)]
pub struct DataInfo {
    /// Incoming packets
    incoming_packets: u128,
    /// Outgoing packets
    outgoing_packets: u128,
    /// Incoming bytes
    incoming_bytes: u128,
    /// Outgoing bytes
    outgoing_bytes: u128,
    /// Latest time of occurrence
    pub final_timestamp: DateTime<Local>,
}

impl DataInfo {
    pub fn incoming_packets(&self) -> u128 {
        self.incoming_packets
    }

    pub fn outgoing_packets(&self) -> u128 {
        self.outgoing_packets
    }

    pub fn incoming_bytes(&self) -> u128 {
        self.incoming_bytes
    }

    pub fn outgoing_bytes(&self) -> u128 {
        self.outgoing_bytes
    }

    pub fn tot_packets(&self) -> u128 {
        self.incoming_packets + self.outgoing_packets
    }

    pub fn tot_bytes(&self) -> u128 {
        self.incoming_bytes + self.outgoing_bytes
    }

    pub fn add_packet(&mut self, bytes: u128, traffic_direction: TrafficDirection) {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            self.outgoing_packets += 1;
            self.outgoing_bytes += bytes;
        } else {
            self.incoming_packets += 1;
            self.incoming_bytes += bytes;
        }
        self.final_timestamp = Local::now();
    }

    pub fn new_with_first_packet(bytes: u128, traffic_direction: TrafficDirection) -> Self {
        if traffic_direction.eq(&TrafficDirection::Outgoing) {
            Self {
                incoming_packets: 0,
                outgoing_packets: 1,
                incoming_bytes: 0,
                outgoing_bytes: bytes,
                final_timestamp: Local::now(),
            }
        } else {
            Self {
                incoming_packets: 1,
                outgoing_packets: 0,
                incoming_bytes: bytes,
                outgoing_bytes: 0,
                final_timestamp: Local::now(),
            }
        }
    }

    #[cfg(test)]
    pub fn new_for_tests(
        incoming_packets: u128,
        outgoing_packets: u128,
        incoming_bytes: u128,
        outgoing_bytes: u128,
    ) -> Self {
        Self {
            incoming_packets,
            outgoing_packets,
            incoming_bytes,
            outgoing_bytes,
            final_timestamp: Default::default(),
        }
    }
}

impl AddAssign for DataInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.incoming_packets += rhs.incoming_packets;
        self.outgoing_packets += rhs.outgoing_packets;
        self.incoming_bytes += rhs.incoming_bytes;
        self.outgoing_bytes += rhs.outgoing_bytes;
        self.final_timestamp = Local::now();
    }
}
