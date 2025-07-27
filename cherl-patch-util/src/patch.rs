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

pub struct Patch {
    os_name: String,
    patch: String,
    cherl_version: String,
}

impl Patch {
    pub fn new(os_name: String, patch: String) -> Self {
        Patch {
            os_name,
            patch,
            cherl_version: cherl_util::version::get_version().to_string(),
        }
    }
}
