use futures::future::try_join_all;
use std::{ffi::OsStr, io::Result, path::Path};
use tokio::fs;
use tokio_stream::{wrappers::ReadDirStream, StreamExt};

pub async fn process(target_dir: impl AsRef<Path>) -> Result<()> {
    let files = ReadDirStream::new(fs::read_dir(target_dir).await?)
        .map(|entry| {
            entry.map(
                |entry| async move { entry.file_type().await.map(|file_type| (file_type, entry)) },
            )
        })
        .collect::<Result<Vec<_>>>()
        .await?;
    let mut files = try_join_all(files)
        .await?
        .into_iter()
        .filter(|(file_type, _)| file_type.is_file())
        .map(|(_, entry)| entry.path())
        .collect::<Vec<_>>();
    files.sort();

    let video_names = filter_video_names(&files);
    let subtitle_files = filter_subtitle_files(&files);

    try_join_all(
        subtitle_files
            .zip(video_names)
            .map(|(subtitle_file, video_name)| {
                fs::rename(
                    subtitle_file,
                    subtitle_file
                        .with_file_name(video_name)
                        .with_extension("ass"),
                )
            }),
    )
    .await?;

    Ok(())
}

fn filter_video_names<P: AsRef<Path>>(files: &[P]) -> impl Iterator<Item = &OsStr> {
    files
        .iter()
        .map(|file| file.as_ref())
        .filter(|file| match file.extension() {
            Some(ext) => ext.eq_ignore_ascii_case("mkv") || ext.eq_ignore_ascii_case("mp4"),
            None => false,
        })
        .filter_map(|file| file.file_stem())
}

fn filter_subtitle_files<P: AsRef<Path>>(files: &[P]) -> impl Iterator<Item = &Path> {
    files
        .iter()
        .map(|file| file.as_ref())
        .filter(|file| {
            file.extension()
                .map(|ext| ext.eq_ignore_ascii_case("ass"))
                .unwrap_or(false)
        })
        .filter(|file| {
            let path = file.to_string_lossy();
            !path.contains("TC") && !path.contains("cht")
        })
}
