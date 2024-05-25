use std::path::PathBuf;

use anyhow::Result;
use frankenstein::AsyncApi;
use frankenstein::AsyncTelegramApi;
use frankenstein::SendAudioParams;
use frankenstein::SendMessageParams;
// use frankenstein::SendVideoParams;
use rusty_ytdl::Video;
use rusty_ytdl::VideoOptions;
use rusty_ytdl::VideoQuality;
use rusty_ytdl::VideoSearchOptions;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = std::env::args().collect();
    let url = &args[1];
    let chat_id = &args[2];
    let token = &args[3];
    let api = AsyncApi::new(token);
    // 下载高质量音频格式文件
    let file = down_m4a(url, VideoQuality::Highest).await?;
    // 构造发送音频参数
    let params = SendAudioParams::builder()
        .chat_id(chat_id.to_owned())
        .audio(file.clone())
        .build();
    // 如果发送失败则下载低质量音频发送
    if let Err(_error) = api.send_audio(&params).await {
        let _ = std::fs::remove_file(file);
        // 高品质音频超过50MB会发送失败，将尝试下载低品质音频
        let file_low = down_m4a(url, VideoQuality::Lowest).await?;
        // 构造发送音频参数
        let params = SendAudioParams::builder()
            .chat_id(chat_id.to_owned())
            .audio(file_low.clone())
            .build();
        // 如果发送失败则发送一条消息提示
        if let Err(error) = api.send_audio(&params).await {
            let _ = std::fs::remove_file(file_low);
            let message = format!("Failed to upload Audio: {error:?}");
            let mes = SendMessageParams::builder()
                .chat_id(chat_id.to_owned())
                .text(message)
                .build();
            let _ = api.send_message(&mes).await;
        } else {
            let _ = std::fs::remove_file(file_low);
        }
    } else {
        let _ = std::fs::remove_file(file);
    }
    Ok(())
}

// todo:自建TG BOT API SERVER后发送视频版，以解决发送文件大小限制，50MB -> 2GB

async fn down_m4a(url: &String, video_quality: VideoQuality) -> Result<PathBuf, anyhow::Error> {
    // 构建下载音频参数
    let video_options = VideoOptions {
        quality: video_quality.clone(),
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let audio = Video::new_with_options(url, video_options)?;
    // 获取链接标题
    let mut title = audio.get_info().await?.video_details.title;
    let mut chars: Vec<char> = title.chars().collect();
    // 某些链接标题过长会导致发送失败，进行截断
    if chars.len() > 25 {
        chars.truncate(25);
        title = chars.into_iter().collect();
    }

    // 如果是低质量音频则在文件名后缀添加_low标识
    let file_name = match video_quality {
        VideoQuality::Highest => format!("./{title}.m4a"),
        VideoQuality::Lowest => format!("./{title}_low.m4a"),
        _ => format!("./{title}.m4a"),
    };
    let file = std::path::PathBuf::from(&file_name);
    audio.download(&file).await?;
    Ok(file)
}
