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
use reqwest::Response;

pub struct Remote {
    repo_url: String,
}

impl Remote {
    pub fn new(repo_url: String) -> Self {
        Remote { repo_url }
    }

    pub fn get_repo_url(&self) -> &String {
        &self.repo_url
    }

    // https://github.com/destroydevs/cherl
    pub fn get_repo_info(&self) -> Option<RepoInfo> {
        let url = self.get_repo_url().clone().replace("https://", "");

        let split = url.split("/").collect::<Vec<&str>>();

        if split.len() >= 3 {
            let owner = *split.get(1).unwrap();
            let name = *split.get(2).unwrap();

            return Some(RepoInfo {
                owner: owner.to_string(),
                name: name.to_string(),
            });
        }
        None
    }

    // https://raw.githubusercontent.com/destroydevs/cherl/refs/heads/master/templates/git-init.toml
    pub async fn get_from_template(&self, file: String) -> Result<Response, anyhow::Error> {
        let mut url = String::from("https://raw.githubusercontent.com/");

        let repo_info = self.get_repo_info();

        if let Some(repo_info) = repo_info {
            url.push_str(repo_info.owner.as_str());

            url.push('/');

            url.push_str(repo_info.name.as_str());
            url.push_str("/refs/heads/master/templates/");
            url.push_str(file.as_str());
        } else {
            Err(Error::msg("repo info not found"))?
        }

        let response = reqwest::get(url).await?;

        Ok(response)
    }

    pub fn has_mime(file: String) -> bool {
        file.ends_with(".toml") || file.ends_with(".lua") || file.ends_with(".json")
    }

    pub async fn get_templates(&self) -> Option<Vec<String>> {
        let templates: Vec<String> = Vec::new();

        let mut repo = self.get_repo_url().clone();

        repo.push_str("/tree/master/templates");

        let response = reqwest::get(repo).await.unwrap();

        println!("{}", response.text().await.unwrap());

        templates.is_empty().then(|| templates)
    }
}

pub struct RepoInfo {
    pub owner: String,
    pub name: String,
}
