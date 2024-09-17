use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, Sink};

use crate::config::config::InitConfig;

/// [DBCOnfig] 用于存储共享状态的DataBase

lazy_static! {
    pub static ref RING:Arc<RwLock<Sink>>=init_local_ring();
}


 fn init_local_ring() -> Arc<RwLock<Sink>> {
    let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
    _path.push(".benben\\ring.mp3");
    // 异步加载音频文件
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(_path).unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    Arc::new(RwLock::new(sink))
}
pub fn get_local_ring() -> Sink {
    let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
    _path.push(".benben\\ring.mp3");
    // 异步加载音频文件
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(_path).unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink
}

pub fn local_ring(){
    let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
    _path.push(".benben\\ring.mp3");
    // 异步加载音频文件
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(_path).unwrap());
    let sink = Sink::try_new(&stream_handle).unwrap();
    let source = Decoder::new(file).unwrap();
    sink.append(source);
    sink.play();
    std::thread::sleep(std::time::Duration::from_secs(10));
}







#[cfg(test)]
mod tests {
    use std::fs::File;

    use rodio::{Sink, Source};

    use super::*;

    #[test]
    fn test_music() {
        let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
        _path.push(".benben\\ring.mp3");
        // 异步加载音频文件
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(_path).unwrap());
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples());

        std::thread::sleep(std::time::Duration::from_secs(50));
    }


    #[test]
    fn test_stop_music() {
        let mut _path: std::path::PathBuf = std::env::current_dir().unwrap();
        _path.push(".benben\\ring.mp3");
        // 异步加载音频文件
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open(_path).unwrap());
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.play();
        std::thread::sleep(std::time::Duration::from_secs(5));
        sink.pause();
        std::thread::sleep(std::time::Duration::from_secs(15));
    }
    #[test]
    fn test_control_get_music() {
        local_ring();
        std::thread::sleep(std::time::Duration::from_secs(15));
    }


}