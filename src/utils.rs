use std::str::FromStr;

pub fn lerp(x0: f32, x1: f32, t: f32) -> f32 {
    t * x0 + (1. - t) * x1
}

pub fn parse_pair<T: FromStr>(s: &str, delimiter: char) -> Option<(T,T)> {
	match s.find(delimiter) {
		Some(index) => {
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
				(Ok(l), Ok(r)) => Some((l, r)),
				_ => None,
			}
		},
		None => None,
	}
}
