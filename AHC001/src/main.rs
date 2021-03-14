use proconio::{
    input,
    // marker::Bytes, // bytes
    // marker::Chars, // vector<char>
    // marker::Isize1, // 1-index to 0-index integer
    // marker::Usize1, // 1-index to 0-index unsigned integer
};
use rand::prelude::*;
use rand_pcg::Mcg128Xsl64;

macro_rules! debug {
    ($($a:expr),*) => {
        eprintln!(concat!($(stringify!($a), " = {:?}, "),*), $($a),*);
    }
}

const W: i64 = 10000;

// from wata_admin(パクりました。すみません)
fn get_time() -> f64 {
	static mut STIME: f64 = -1.0;
	let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap();
	let ms = t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9;
	unsafe {
			if STIME < 0.0 {
					STIME = ms;
			}
			ms - STIME
	}
}

#[derive(Clone, Debug)]
struct Input {
    n: usize,
    ps: Vec<(i64, i64)>,
	size: Vec<i64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rect {
    x1: i64,
	x2: i64,
	y1: i64,
	y2: i64,
}

impl Rect {
	pub fn size(&self) -> i64 {
		(self.x2 - self.x1) * (self.y2 - self.y1)
	}
}

fn intersect(r1: &Rect, r2: &Rect) -> bool {
	r1.x2.min(r2.x2) > r1.x1.max(r2.x1) && r1.y2.min(r2.y2) > r1.y1.max(r2.y1)
}

fn calc_onepoint(input: &Input, out: &Vec<Rect>, idx: &usize) -> f64 {
	let i = idx.clone();
	if out[i].x1 < 0 || out[i].x2 > W || out[i].y1 < 0 || out[i].y2 > W {
		// eprintln!("rectangle {} is out of range", i);
		return -100.;
	}
	if out[i].x1 >= out[i].x2 || out[i].y1 >= out[i].y2 {
		// eprintln!("rectangle {} does not have positive area", i);
		return -100.;
	}
	if !(out[i].x1 <= input.ps[i].0 && input.ps[i].0 < out[i].x2 && out[i].y1 <= input.ps[i].1 && input.ps[i].1 < out[i].y2) {
		// eprintln!("rectangle {} does not contain point {}", i, i);
		return -100.;
	}
	for j in 0..out.len() {
		if i == j{ continue; }
		if intersect(&out[i], &out[j]) {
			// eprintln!("rectangles {} and {} overlap", j, i);
			return -100.;
		}
	}
	let s = out[i].size().min(input.size[i]) as f64 / out[i].size().max(input.size[i]) as f64;
	return 1.0 - (1.0 - s) * (1.0 - s);
}

fn calc_point(input: &Input, out: &Vec<Rect>) -> i64 {
	let n = input.n;
	let mut score = 0.0;
	for i in 0..n {
		let s = out[i].size().min(input.size[i]) as f64 / out[i].size().max(input.size[i]) as f64;
		score += 1.0 - (1.0 - s) * (1.0 - s);
	}
	return (1e9 * score / n as f64).round() as i64;
}

fn calc_newpoint(input: &Input, oldout: &Vec<Rect>, out: &Vec<Rect>, idx: &usize) -> i64 {
	let point = calc_onepoint(&input, &out, &idx);
	if point < 0. {
		return -1e9 as i64 + 7;
	}
	else {
		return (1e9 * (point - calc_onepoint(&input, &oldout, &idx)) / input.n as f64).round() as i64;
	}
}

fn reader() -> Input {
    input! {
        n: usize,
        xyrs: [(i64, i64, i64); n] 
    }
    let ps = xyrs.iter().map(|&(x, y, _)| (x, y)).collect::<Vec<_>>();
	let size = xyrs.iter().map(|&(_, _, s)| s).collect::<Vec<_>>();
	return Input { n, ps, size };
}

fn update_square(ps: &(i64, i64), s: &Rect, r: &Mcg128Xsl64, siz: &i64) -> Rect {
	let mut rng = (*r).clone();
	let mut new_square = Rect{x1: 0, y1: 0, x2: 0, y2: 0};
	new_square.x1 = ps.0.min(s.x1 + rng.gen_range(-siz - 1, siz));
	new_square.y1 = ps.1.min(s.y1 + rng.gen_range(-siz - 1, siz));
	new_square.x2 = (ps.0+1).max(s.x2 + rng.gen_range(-siz - 1, siz));
	new_square.y2 = (ps.1+1).max(s.y2 + rng.gen_range(-siz - 1, siz));

	new_square
}

// fn move_square(ps: &(i64, i64), s: &Rect, siz: &(i64, i64)) -> Rect {
// 	let mut new_square = Rect{x1: 0, y1: 0, x2: 0, y2: 0};
// 	new_square.x1 = ps.0.min(s.x1 + siz.0);
// 	new_square.y1 = ps.1.min(s.y1 + siz.1);
// 	new_square.x2 = (ps.0+1).max(s.x2 + siz.0);
// 	new_square.y2 = (ps.1+1).max(s.y2 + siz.1);
// 	new_square
// }

// fn gen_square(ps: &(i64, i64), s: &i64) -> Rect {
// 	if rand::thread_rng().gen_range(0., 1.) > 0.5 {
// 		let mut x1 = 0i64;
// 		if ps.0 > 1 {
// 			x1 = ps.0 - rand::thread_rng().gen_range(1i64, ps.0);
// 		}
// 		let mut x2 = W - 1;
// 		if ps.0 < W - 2 {
// 			x2 = ps.0 + rand::thread_rng().gen_range(1i64, W - ps.0 - 1);
// 		}
// 		// debug!(x1, x2);
// 		let ymax = (s + (x2 - x1) - 1) / (x2 - x1);
// 		let mut y1 = 0i64;
// 		if ps.1 > 0 {
// 			y1 = ps.1 - rand::thread_rng().gen_range(0i64, ps.1);
// 		}
// 		let mut y2 = W - 1;
// 		if ps.1 < W - 2 {
// 			y2 = rand::thread_rng().gen_range(y1, y1 + ymax);
// 		}
// 		return Rect{x1, y1, x2, y2};
// 	}
// 	else {
// 		let mut y1 = 0i64;
// 		if ps.1 > 1 {
// 			y1 = ps.1 - rand::thread_rng().gen_range(1i64, ps.1);
// 		}
// 		let mut y2 = W - 1;
// 		if ps.1 < W - 2 {
// 			y2 = ps.1 + rand::thread_rng().gen_range(1i64, W - ps.1 - 1);
// 		}
// 		let xmax = (s + (y2 - y1) - 1) / (y2 - y1);
// 		let mut x1 = 0i64;
// 		if ps.0 > 0 {
// 			x1 = ps.0 - rand::thread_rng().gen_range(0i64, ps.0);
// 		}
// 		let mut x2 = W - 1;
// 		if ps.0 < W - 2 {
// 			x2 = rand::thread_rng().gen_range(x1, x1 + xmax);
// 		}
// 		return Rect{x1, y1, x2, y2};
// 	}
// }

fn calc_origin(input: &Input) -> Vec<Rect> {
	// 1 * 1 の初期マス生成
	let mut square:Vec<Rect> = Vec::new();
	for i in 0..input.n {
		let x1 = input.ps[i].0;
		let x2 = x1 + 1;
		let y1 = input.ps[i].1;
		let y2 = y1 + 1;
		square.push(Rect{x1, y1, x2, y2});
	}
	square
}

fn print_outputs(best: &Vec<Rect>) {
	for rect in best {
		println!("{} {} {} {}", rect.x1, rect.y1, rect.x2, rect.y2);
	}
}

fn main() {
    let stime = get_time();
    let input = reader();
    // debug!(input.ps, input.size);
    const TL: f64 = 4.95;
	// const TL: f64 = 300.;
	// eprintln!("準備1: 初期四角形生成");
    let mut output = calc_origin(&input);
	// eprintln!("準備2: 計算");
    let mut point = calc_point(&input, &output);
	// eprintln!("初期解 {}", point);
	// eprintln!("生成時間 {}", get_time() - stime);

	// 色々更新するかもだけど出てきた一番いい解を保存するやつ
	let mut best = output.clone();
	let mut best_score = point.clone();

	// 焼きなましをパクってみる
	const T0: f64 = 3e10;
	const T1: f64 = 6e3;
	// let mut RNG = 25i64;
	const RNG: i64 = 25;
	let mut cnt = 0;
	let mut temp = T0;
	let mut rng = rand_pcg::Pcg64Mcg::new(314159);
	let mut iteration = 0;
	let mut flag = false;
	let mut fin = false;
	println!("{}", input.n);
    loop {
		cnt += 1;
		
		// 1000イテレーションごとに温度変化 & 時間計測
		if cnt % 1000 == 0 {
			let t = (get_time() - stime) / TL;
			if t >= 1.0 {
				break;
			}
			iteration += 1;
			if iteration % 50 == 0 {
				print_outputs(&best);
			}
			temp = T0.powf(1.0 - t) * T1.powf(t);
			if t > 0.6 && !fin{
				flag = true;
			}
		}
		let mut diff = 0;
		let mut new_output;
		if flag {
			new_output = best.clone();
			let mut points : Vec<(f64, usize)> = Vec::new();
			for i in 0..input.n {
				points.push((calc_onepoint(&input, &best, &i), i));
			}
			points.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
			// debug!(points);
			for (score, i) in points {
				for x in 0..2{
					loop {
						match x {
							0 => new_output[i].x1 -= 1,
							1 => new_output[i].y1 -= 1,
							2 => new_output[i].x2 += 1,
							_ => new_output[i].y2 += 1,
						}
						if calc_onepoint(&input, &new_output, &i) - score < 0. {
							match x {
								0 => new_output[i].x1 += 1,
								1 => new_output[i].y1 += 1,
								2 => new_output[i].x2 -= 1,
								_ => new_output[i].y2 -= 1,
							}
							break;
						}
					}
				}
				diff = calc_newpoint(&input, &best, &new_output, &i);
				// debug!(i, diff, new_output[i]);
				// debug!(best_score, best_score + diff);
				// 遷移しないけどいいスコアは保存しとく
				if best_score < best_score + diff {
					best_score = best_score + diff;
					best = new_output.clone();
				}
			}
			flag = false;
			fin = true;
		}
		else {
			new_output = output.clone();
			// 更新
			let index = rng.gen_range(0, output.len() - 1);
			new_output[index] = update_square(&input.ps[index], &output[index], &rng, &RNG);
			diff = calc_newpoint(&input, &output, &new_output, &index);
		}
		
		// 違反してないかチェック
		if diff == -1e9 as i64 + 7 {
			// out_cnt += 1;
			continue;
		}
		
		// 遷移しないけどいいスコアは保存しとく
		if best_score < point + diff {
			best_score = point + diff;
			best = new_output.clone();
		}

		// 温度関数でいい感じなら
		if diff < 0 && !rng.gen_bool(f64::exp((diff) as f64 / temp)){
			// debug!(diff);
			point += diff;
			output = new_output;
		}
		else if diff > 0 {
			point += diff;
			output = new_output;
		}
    }
    
	// for i in 0..input.n {
	// 	debug!(i, calc_onepoint(&input, &output, &i));
	// }
	// for i in 0..input.n {
	// 	eprintln!("{}: {}", i, calc_onepoint(&input, &best, &i));
	// }
	print_outputs(&best);
	
	// debug!(iteration);
	// debug!(out_cnt);
    debug!(best_score);
	eprintln!("生成時間: {}", get_time() - stime);
}
