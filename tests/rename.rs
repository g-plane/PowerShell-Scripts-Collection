use std::{env::temp_dir, io::Result};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[tokio::test]
async fn mkv_files() -> Result<()> {
    let mut dir = temp_dir();
    dir.push("rsf_mkv");

    let _ = fs::remove_dir_all(&dir).await;
    fs::create_dir(&dir).await?;

    for i in 1u8..=3 {
        let mut mkv = dir.clone();
        mkv.push(format!("{}.mkv", i));
        let mut mkv = File::create(&mkv).await?;
        mkv.write_all(i.to_string().as_bytes()).await?;
    }

    for (i, alpha) in ('a'..='c').enumerate() {
        let mut ass = dir.clone();
        ass.push(format!("{}.ass", alpha));
        let mut ass = File::create(&ass).await?;
        ass.write_all((i + 1).to_string().as_bytes()).await?;
    }

    rename_subtitle_files::process(&dir).await?;

    for i in 1u8..=3 {
        let mut video = dir.clone();
        video.push(format!("{}.mkv", i));
        assert_eq!(&fs::read_to_string(&video).await?, &i.to_string());

        let mut ass = dir.clone();
        ass.push(format!("{}.ass", i));
        assert_eq!(&fs::read_to_string(&ass).await?, &i.to_string());
    }

    Ok(())
}

#[tokio::test]
async fn mp4_files() -> Result<()> {
    let mut dir = temp_dir();
    dir.push("rsf_mp4");

    let _ = fs::remove_dir_all(&dir).await;
    fs::create_dir(&dir).await?;

    for i in 1u8..=3 {
        let mut mp4 = dir.clone();
        mp4.push(format!("{}.mp4", i));
        let mut mp4 = File::create(&mp4).await?;
        mp4.write_all(i.to_string().as_bytes()).await?;
    }

    for (i, alpha) in ('a'..='c').enumerate() {
        let mut ass = dir.clone();
        ass.push(format!("{}.ass", alpha));
        let mut ass = File::create(&ass).await?;
        ass.write_all((i + 1).to_string().as_bytes()).await?;
    }

    rename_subtitle_files::process(&dir).await?;

    for i in 1u8..=3 {
        let mut video = dir.clone();
        video.push(format!("{}.mp4", i));
        assert_eq!(&fs::read_to_string(&video).await?, &i.to_string());

        let mut ass = dir.clone();
        ass.push(format!("{}.ass", i));
        assert_eq!(&fs::read_to_string(&ass).await?, &i.to_string());
    }

    Ok(())
}
