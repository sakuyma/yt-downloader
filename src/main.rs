use chrono::Local;
use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;
use which::which;
use yt_dlp::Youtube;
use yt_dlp::client::deps::Libraries;

#[derive(Parser)]
#[command(name = "yt-dwn")]
#[command(about = "Youtube video downloader")]
struct Cli {
    // Video url
    url: Option<String>,

    // Download video
    #[arg(short = 'd', long = "download")]
    download: bool,

    // Output dir
    #[arg(short = 'o', long = "output", default_value = "~/Downloads")]
    output_dir: PathBuf,

    // Filename
    #[arg(short = 'f', long = "filename")]
    filename: Option<String>,
}

// func for check finding system binaries
fn check_dependencies() -> Result<(PathBuf, PathBuf), String> {
    let yt_dlp = which("yt-dlp").map_err(|_| "yt-dlp not found in system PATH".to_string())?;

    let ffmpeg = which("ffmpeg").map_err(|_| "ffmpeg not found in system PATH".to_string())?;

    Ok((yt_dlp, ffmpeg))
}

fn get_dependency_paths() -> Result<(PathBuf, PathBuf), String> {
    let yt_dlp = which("yt-dlp").map_err(|_| "yt-dlp not found in system PATH".to_string())?;

    let ffmpeg = which("ffmpeg").map_err(|_| "ffmpeg not found in system PATH".to_string())?;

    Ok((yt_dlp, ffmpeg))
}

async fn download_video(
    url: String,
    output_dir: PathBuf,
    filename: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (yt_dlp_path, ffmpeg_path) = get_dependency_paths()?;

    std::fs::create_dir_all(&output_dir)?;

    // Create libs
    let libraries = Libraries::new(yt_dlp_path, ffmpeg_path);

    // Create fetcher
    let fetcher = Youtube::new(libraries, output_dir).await?;

    // Determine filename
    let final_filename = filename.unwrap_or_else(|| {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        format!("video_{}.mp4", timestamp)
    });
    // Download video
    let video_path = fetcher.download_video_from_url(url, final_filename).await?;

    println!("Video downloaded successfully!");
    println!("Saved to: {:?}", video_path);

    // Show file info
    if let Ok(metadata) = fs::metadata(&video_path) {
        let size_bytes = metadata.len();
        let size_mb = size_bytes as f64 / 1024.0 / 1024.0;
        let size_gb = size_mb / 1024.0;

        if size_gb >= 1.0 {
            println!("File size: {:.2} GB ({:.0} MB)", size_gb, size_mb);
        } else {
            println!("File size: {:.2} MB", size_mb);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if cli.download {
        // Check if dependencies already installed before downloading
        match check_dependencies() {
            Ok((_yt_dlp, _ffmpeg_path)) => {
                // Get URL
                let url = if let Some(url) = cli.url {
                    url
                } else {
                    let mut input = String::new();
                    println!("Enter YouTube video URL:");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Error while reading url line");
                    input.trim().to_string()
                };

                if url.is_empty() {
                    eprintln!("URL cannot be empty");
                    return;
                }

                // Download video
                match download_video(url, cli.output_dir, cli.filename).await {
                    Ok(_) => println!("Download Complete!"),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                eprintln!("Please install:");
                eprintln!("- yt-dlp (https://github.com/yt-dlp/yt-dlp)");
                eprintln!("- ffmpeg (https://ffmpeg.org)");
                eprintln!("And make sure they're in your PATH.");
                return;
            }
        }
    } else {
        // Show help
        println!("YouTube Video Downloader");
        println!("========================");
        println!("\nUsage:");
        println!("  yt-dwn --download <URL>    Download video");
        println!("  yt-dwn --download          Download video (interactive)");
        println!("\nOptions:");
        println!("  -o, --output DIR    Output directory (default: Downloads/)");
        println!("  -f, --filename NAME Custom filename");
        println!("\nExamples:");
        println!("  yt-dwn --download https://youtu.be/example");
        println!("  yt-dwn --download -o ~/Videos -f myvideo.mp4");
    }
}
