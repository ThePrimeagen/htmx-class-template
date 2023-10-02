use std::time::Duration;

use anyhow::Result;
use log::warn;
use tokio::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum TimingType {
    Movement,
    Consume,
}

#[derive(Debug, Clone)]
pub struct Timing {
    pub timing_type: TimingType,
	pub start: u64,
	pub stop: u64,
	pub id: usize,
}

impl Timing {
    pub fn duration(&self) -> u64 {
        return self.stop - self.start;
    }

    pub fn duration_with_dir(&self) -> isize {
        return if self.timing_type == TimingType::Movement {
            self.duration() as isize
        } else {
            -(self.duration() as isize)
        };
    }
}

#[derive(Debug, Clone)]
pub struct Data {
    timings: Vec<Timing>,
    current_running: Option<Timing>,
    id: usize,
}

impl Data {
    fn clear(&mut self) {
        self.timings.clear();
        self.current_running = None;
        self.id = 0;
    }
}

fn now() -> u64 {
    return std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(Duration::default())
        .as_millis() as u64;
}

lazy_static::lazy_static! {
    static ref DATA: Mutex<Data> = Mutex::new(Data {
        timings: Vec::new(),
        current_running: None,
        id: 0,
    });
}

pub async fn push_timing(timing_type: TimingType) -> Result<()> {
    let mut data = DATA.lock().await;
    if let Some(mut timing) = data.current_running.take() {
        if timing.timing_type == timing_type {
            timing.stop = now();
            data.timings.push(timing);
            return Ok(());
        }
        data.current_running = Some(timing);
        return Err(anyhow::anyhow!("Mismatched timing type"));
    }

    let timing = Timing {
        timing_type,
        start: now(),
        stop: 0,
        id: data.id,
    };

    data.id += 1;
    data.current_running = Some(timing);

    return Ok(());
}

pub async fn get_timings() -> Vec<Timing> {
    let data = DATA.lock().await;
    let mut out = vec![];
    if let Some(timing) = &data.current_running {
        out.push(timing.clone());
    }

    out.extend(data.timings.clone());
    return out;
}

pub async fn clear_timings() {
    let mut data = DATA.lock().await;
    data.clear();
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use crate::db::TimingType;
    use crate::db::clear_timings;
    use crate::db::get_timings;
    use crate::db::push_timing;


    #[tokio::test]
    async fn add_timings() -> Result<(), anyhow::Error> {
        clear_timings().await;

        push_timing(TimingType::Consume).await?;
        assert_eq!(get_timings().await.len(), 1);
        tokio::time::sleep(Duration::from_secs(1)).await;

        push_timing(TimingType::Consume).await?;
        let timings = get_timings().await;
        assert_eq!(timings.len(), 1);
        assert!(timings[0].duration() >= 1000);
        assert!(timings[0].duration_with_dir() <= -1000);

        return Ok(());
    }

}


