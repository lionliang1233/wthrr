use anyhow::Result;
use regex::Regex;
use term_painter::{
	Color::{BrightBlack, NotSet},
	ToStyle,
};

use crate::{args::Forecast as ForecastArgs, weather::Weather};

use self::{current::Current, forecast::Forecast};

mod border;
mod current;
mod forecast;
mod greeting;
mod weathercode;
mod wind;

pub struct Product {
	pub address: String,
	pub weather: Weather,
}

pub const MIN_WIDTH: usize = 34;

impl Product {
	pub async fn render(&self, forecast: &Option<ForecastArgs>, include_greeting: bool, lang: &str) -> Result<()> {
		greeting::render(include_greeting, lang).await?;

		if forecast.is_some() {
			Forecast::render(self, lang, forecast.as_ref().unwrap().week).await?;
		} else {
			Current::render(self, lang).await?;
		}

		// Disclaimer
		BrightBlack.with(|| println!(" Weather data by Open-Meteo.com"));

		// Reset colors
		NotSet.with(|| println!());

		Ok(())
	}

	pub fn trunc_address(mut address: String, max_width: usize) -> Result<String> {
		let address_len = address.chars().count();

		address = if address_len > max_width {
			// For many places with overlong names the results seem better when partially removing text
			// between first and second comma instead of removing it between penultimate and last comma
			// let last_comma = title.matches(',').count();
			let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
			let re = Regex::new(&prep_re).unwrap();

			re.replace(&address, "$1$2").to_string()
		} else {
			address
		};

		if address_len > max_width {
			address = Self::trunc_address(address, max_width)?;
		}

		Ok(address)
	}
}
