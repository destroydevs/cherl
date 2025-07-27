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

use anyhow::{Error, Ok};
use indicatif::ProgressBar;

use crate::remote::Remote;

pub trait Download {
    fn download(&self, file: String) -> impl Future<Output = Result<(), Error>> + Send;
}

impl Download for Remote {
    async fn download(&self, file: String) -> Result<(), Error> {
        let bar = ProgressBar::new(6);

        bar.set_style(cherl_util::progress_bar::style());

        bar.inc(1);

        let response = self.get_from_template(file.clone()).await?;
        bar.inc(1);

        let reader = response.text().await?;
        bar.inc(1);

        let mut reader = reader.as_bytes();
        bar.inc(1);

        let mut file = std::fs::File::create(file)?;
        bar.inc(1);

        std::io::copy(&mut reader, &mut file)?;
        bar.inc(1);

        println!("File saved successfuly!");

        Ok(())
    }
}
