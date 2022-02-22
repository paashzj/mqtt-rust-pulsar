// Licensed to the Apache Software Foundation (ASF) under one or more
// contributor license agreements.  See the NOTICE file distributed with
// this work for additional information regarding copyright ownership.
// The ASF licenses this file to You under the Apache License, Version 2.0
// (the "License"); you may not use this file except in compliance with
// the License.  You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::fmt::{Display, Formatter};

/// Configuration options for mqsar
pub struct MqsarConfig {
    pub mqtt_config: MqttConfig,
    pub pulsar_config: PulsarConfig,
}

impl Display for MqsarConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "mqttConfig: {} pulsarConfig: {}",
            self.mqtt_config, self.pulsar_config
        )
    }
}

pub struct MqttConfig {
    pub host: String,
    pub port: u16,
    pub qos1_no_wait_reply: bool,
}

impl Display for MqttConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "host: {} port: {} qos1NoWaitReply: {}",
            self.host, self.port, self.qos1_no_wait_reply
        )
    }
}

pub struct PulsarConfig {
    pub host: String,
    pub port: u16,
    pub produce_config: PulsarProduceConfig,
    pub consume_config: PulsarConsumeConfig,
}

impl Display for PulsarConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "host: {} port: {} produceConfig: {} consumeConfig: {}",
            self.host, self.port, self.produce_config, self.consume_config
        )
    }
}

pub struct PulsarProduceConfig {
    pub disable_batching: bool,
}

impl Display for PulsarProduceConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "disableBatching: {}", self.disable_batching)
    }
}

pub struct PulsarConsumeConfig {}

impl Display for PulsarConsumeConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}
