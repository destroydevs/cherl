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

use cherl_remote_util::downloader::Download;

#[tokio::main]
async fn main() {
    let remote =
        cherl_remote_util::remote::Remote::new("https://github.com/destroydevs/cherl".to_string());

    let _ = remote.download("git-init.toml".to_string()).await;
}
