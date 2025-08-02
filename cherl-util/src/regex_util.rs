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

use regex::Regex;

pub struct RegexUtil {
    regex: Regex,
}

#[derive(Clone)]
pub struct RepoInfo {
    owner: String,
    name: String,
    branch: Option<String>,
}

impl RepoInfo {
    pub fn new(owner: String, name: String, branch: Option<String>) -> RepoInfo {
        RepoInfo { owner, name, branch }
    }

    pub fn get_owner(&self) -> &String {
        &self.owner
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_branch(&self) -> &Option<String> {

        &self.branch
    }
}

impl RegexUtil {
    pub fn new() -> RegexUtil {
        RegexUtil { regex: Regex::new(r"^https?://github\.com/([^/]+)/([^/]+)(?:/tree/([^/]+))?/?").unwrap(), }
    }

    pub fn parse_github_url<F: AsRef<str>>(&self, url: F) -> Option<RepoInfo> {
        let url = url.as_ref();
        let captures = self.regex.captures(url)?;

        let owner = captures.get(1)?.as_str().to_string();
        let name = captures.get(2)?.as_str().to_string();
        let branch = captures.get(3).map(|m| m.as_str().to_string());

        Some(RepoInfo::new(owner, name, branch))
    }

    pub fn get_raw_url<F: AsRef<str>>(&self, url: F) -> Option<String> {
        let repo_info = match self.parse_github_url(url) {
            Some(info) => info,
            None => {
                log::error!("Invalid GitHub URL");
                return None
            }
        };
        if let Some(branch) = &repo_info.get_branch() {
            Some(format!("https://raw.githubusercontent.com/{}/{}/refs/heads/{}/templates/", repo_info.get_owner(), repo_info.get_name(), branch))
        } else {
            Some(format!("https://raw.githubusercontent.com/{}/{}/refs/heads/master/templates/", repo_info.get_owner(), repo_info.get_name()))
        }
    }
}