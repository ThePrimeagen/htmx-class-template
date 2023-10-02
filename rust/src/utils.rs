pub fn human_time(time: isize) -> String {
    let dir = if time >= 0 { 1 } else { -1 };
    let time = time.abs();

    let mut out = vec![format!("{}ms", time % 1000)];

    let time = time / 1000;

    if time > 0 {
        out.push(format!("{}s", time % 60));
    }

    let time = time / 60;

    if time > 0 {
        out.push(format!("{}m", time % 60));
    }
    let time = time / 60;

    if time > 0 {
        out.push(format!("{}h", time));
    }

    let out_time = out.join(" ");
    if dir == -1 {
        return format!("-{}", out_time);
    }
    return out_time;
}


