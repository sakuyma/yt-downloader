use std::io;
use std::path::PathBuf;
use yt_dlp::client::deps::Libraries;
use yt_dlp::Youtube;

pub async fn check_dependencies() -> bool {
    let yt_dlp = PathBuf::from("libs/yt-dlp");
    let ffmpeg = PathBuf::from("libs/ffmpeg");
    
    yt_dlp.exists() && ffmpeg.exists()
}

pub async fn download_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    let executables_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let _fetcher = Youtube::with_new_binaries(executables_dir, output_dir).await?;
    Ok(())
}

pub async fn download_video(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from("output");

    let youtube = libraries_dir.join("yt-dlp");
    let ffmpeg = libraries_dir.join("ffmpeg");

    let libraries = Libraries::new(youtube, ffmpeg);
    let fetcher = Youtube::new(libraries, output_dir).await?;

    let video_path = fetcher.download_video_from_url(url, "my-video.mp4".to_string()).await?;
    println!("Video Downloaded: {:?}", video_path);
    Ok(())
}

#[tokio::main]
async fn main() {
    if !check_dependencies().await {
        println!("Installing Dependencies...");
        match download_dependencies().await {
            Ok(_) => println!("Dependencies installed"),
            Err(_e) => {
                eprintln!("Error installing dependencies");
                return;
            }
        }
    }

    let mut url = String::new();

    io::stdin()
        .read_line(&mut url)
        .expect("Error while reading url line");
    let url = url.trim().to_string();

    match download_video(url).await {
        Ok(_) => println!("Download Complete!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
