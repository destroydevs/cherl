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

#[cfg(test)]
mod tests {
    use crate::regex_util;


    #[test]
    fn parse_tests() {
        let regex = regex_util::RegexUtil::new();

        let out1 = regex.parse_github_url("https://github.com/destroydevs/cherl/tree/dev/templates");
        let out2 = regex.parse_github_url("https://github.com/destroydevs/cherl");
        let out3 = regex.parse_github_url("https://github.com/destroydevs");
        let out4 = regex.parse_github_url("dont contains url");

        assert_eq!(out1.is_some(), true);

        let test1 = out1.clone().unwrap();

        assert_eq!(test1.get_branch().is_some(), true);
        assert_eq!(test1.get_name(), "cherl");
        assert_eq!(test1.get_owner(), "destroydevs");

        if let Some(branch) = test1.get_branch() {
            assert_eq!(branch, "dev");
        }

        assert_eq!(out2.is_some(), true);

        let test2 = out2.clone().unwrap();

        assert_eq!(test2.get_branch().is_none(), true);
        assert_eq!(test2.get_name(), "cherl");
        assert_eq!(test2.get_owner(), "destroydevs");

        assert_eq!(out3.is_none(), true);
        assert_eq!(out4.is_none(), true);

    }

    #[test]
    fn raw_url_tests() {
        let regex = regex_util::RegexUtil::new();

        let out1 = regex.get_raw_url("https://github.com/destroydevs/cherl/tree/dev/templates");
        let out2 = regex.get_raw_url("https://github.com/destroydevs/cherl");
        let out3 = regex.get_raw_url("https://github.com/destroydevs");

        assert_eq!(out1.is_some(), true);

        let test1 = out1.unwrap();

        assert_eq!(test1, "https://raw.githubusercontent.com/destroydevs/cherl/refs/heads/dev/templates/");
        
        assert_eq!(out2.is_some(), true);

        let test2 = out2.unwrap();

        assert_eq!(test2, "https://raw.githubusercontent.com/destroydevs/cherl/refs/heads/master/templates/");

        assert_eq!(out3.is_none(), true);

    }
}