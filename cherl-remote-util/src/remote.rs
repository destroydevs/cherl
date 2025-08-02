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

use anyhow::Error;
use cherl_util::regex_util::RegexUtil;
use reqwest::Response;

pub struct Remote {
    repo_url: String,
    regex_util: RegexUtil,
}

impl Remote {
    pub fn new(repo_url: String) -> Self {
        Remote { repo_url, regex_util: RegexUtil::new() }
    }

    pub fn get_repo_url(&self) -> &String {
        &self.repo_url
    }

    // getting file content from repo
    pub async fn get_from_template(&self, file: String) -> Result<Response, anyhow::Error> {
        let repo_info = self.regex_util.get_raw_url(self.get_repo_url());

        let mut url = String::new();

        if let Some(_) = repo_info {
            url.push_str(file.as_str());
        } else {
            Err(Error::msg("repo info not found"))?
            // TODO: handle error
        }

        let response = reqwest::get(url).await?;

        Ok(response)
    }

    pub fn has_mime(file: String) -> bool {
        file.ends_with(".toml") || file.ends_with(".lua") || file.ends_with(".json")
    }

    // TODO: check file with mime in repo, if contains return it
    // 1) .lua
    // 2) .json
    // 3) .toml
    pub async fn get_templates(&self, file: String) -> Option<Vec<String>> {
        let templates: Vec<String> = Vec::new();

        let mut repo = self.get_repo_url().clone();

        repo.push_str("/tree/master/templates");

        let response = reqwest::get(repo).await.unwrap();

        println!("{}", response.text().await.unwrap());

        templates.is_empty().then(|| templates)
    }
}