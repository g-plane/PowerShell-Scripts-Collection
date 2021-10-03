use std::{
    env::temp_dir,
    io::{ErrorKind, Result},
};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

#[tokio::test]
async fn ignore_cht() -> Result<()> {
    let mut dir = temp_dir();
    dir.push("rsf_ignore_cht");

    let _ = fs::remove_dir_all(&dir).await;
    fs::create_dir(&dir).await?;

    for i in 1u8..=3 {
        let mut mkv = dir.clone();
        mkv.push(format!("{}.mkv", i));
        let mut mkv = File::create(&mkv).await?;
        mkv.write_all(i.to_string().as_bytes()).await?;
    }

    for (i, alpha) in ('a'..='c').enumerate() {
        let mut chs = dir.clone();
        chs.push(format!("{}.chs.ass", alpha));
        let mut chs = File::create(&chs).await?;
        chs.write_all((i + 1).to_string().as_bytes()).await?;

        let mut cht = dir.clone();
        cht.push(format!("{}.cht.ass", alpha));
        let mut cht = File::create(&cht).await?;
        cht.write_all((i + 1).to_string().as_bytes()).await?;
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
    for alpha in 'a'..='c' {
        let mut chs = dir.clone();
        chs.push(format!("{}.chs.ass", alpha));
        assert_eq!(
            fs::File::open(&chs).await.unwrap_err().kind(),
            ErrorKind::NotFound
        );

        let mut cht = dir.clone();
        cht.push(format!("{}.cht.ass", alpha));
        assert!(fs::File::open(&cht).await.is_ok());
    }

    Ok(())
}

#[tokio::test]
async fn ignore_tc() -> Result<()> {
    let mut dir = temp_dir();
    dir.push("rsf_ignore_tc");

    let _ = fs::remove_dir_all(&dir).await;
    fs::create_dir(&dir).await?;

    for i in 1u8..=3 {
        let mut mkv = dir.clone();
        mkv.push(format!("{}.mkv", i));
        let mut mkv = File::create(&mkv).await?;
        mkv.write_all(i.to_string().as_bytes()).await?;
    }

    for (i, alpha) in ('a'..='c').enumerate() {
        let mut sc = dir.clone();
        sc.push(format!("{}.[SC].ass", alpha));
        let mut sc = File::create(&sc).await?;
        sc.write_all((i + 1).to_string().as_bytes()).await?;

        let mut tc = dir.clone();
        tc.push(format!("{}.[TC].ass", alpha));
        let mut tc = File::create(&tc).await?;
        tc.write_all((i + 1).to_string().as_bytes()).await?;
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
    for alpha in 'a'..='c' {
        let mut sc = dir.clone();
        sc.push(format!("{}.[SC].ass", alpha));
        assert_eq!(
            fs::File::open(&sc).await.unwrap_err().kind(),
            ErrorKind::NotFound
        );

        let mut tc = dir.clone();
        tc.push(format!("{}.[TC].ass", alpha));
        assert!(fs::File::open(&tc).await.is_ok());
    }

    Ok(())
}
