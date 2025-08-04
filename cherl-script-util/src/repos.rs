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

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Repositories {
    repos: Vec<Repository>,
}

impl Repositories {
    pub fn new(json_data: &str) -> Result<Self, anyhow::Error> {
        let raw: Value = serde_json::from_str(json_data)?;
        
        let repos_obj = raw.get("repos")
            .and_then(Value::as_object)
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'repos' field"))?;
        
        let repos = repos_obj.iter()
            .map(|(name, url)| Repository::new(
                name.clone(),
                url.as_str().unwrap_or_default().to_string()
            ))
            .collect();
        
        Ok(Repositories { repos })
    }

    // TODO: add method to get repos by name
    // TODO: get repos by owner
    // TODO: search repos
}

#[derive(Debug, Serialize, Deserialize)]
struct Repository {
    name: String,
    url: String,
}

impl Repository {
    pub fn new(name: String, url: String) -> Self {
        Repository { name, url }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_url(&self) -> &String {
        &self.url
    }
}