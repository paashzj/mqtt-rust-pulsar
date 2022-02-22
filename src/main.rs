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

use mqsar::config::{
    MqsarConfig, MqttConfig, PulsarConfig, PulsarConsumeConfig, PulsarProduceConfig,
};
use mqsar::mqsar_broker::MqsarBroker;

fn main() {
    let mqsar_config = MqsarConfig {
        mqtt_config: MqttConfig {
            host: "localhost".to_owned(),
            port: 1883,
            qos1_no_wait_reply: false,
        },
        pulsar_config: PulsarConfig {
            host: "localhost".to_owned(),
            port: 6650,
            produce_config: PulsarProduceConfig {
                disable_batching: false,
            },
            consume_config: PulsarConsumeConfig {},
        },
    };
    let mqsar_broker = MqsarBroker {
        config: mqsar_config,
    };
    mqsar_broker.run();
}
