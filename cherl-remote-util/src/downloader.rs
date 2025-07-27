// Copyright 2025 Evgeny K.
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     https://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::remote::Remote;

pub struct Download;

impl Download for Remote {
    fn download(&self, file: String) {
        let url = "";
        let response = reqwest::get(url).unwrap();
        let mut file = std::fs::File::create("cherl-remote-util.tar.gz").unwrap();
        std::io::copy(&mut response, &mut file).unwrap();
    }
}