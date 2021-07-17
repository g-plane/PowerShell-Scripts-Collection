import java.io.File

fun main(args: Array<String>) {
    val targetDir = args.getOrElse(0) { System.getProperty("user.dir") }
    val files = File(targetDir).listFiles()

    val subtitlePaths = subtitlesFiles(files)
    val newSubtitlePaths = videoFileNames(files).map { File(targetDir, "$it.ass") }
    subtitlePaths.zip(newSubtitlePaths).forEach { (old, new) -> old.renameTo(new) }
}

fun videoFileNames(files: Array<File>) =
    files.filter { it.isFile }
        .filter {
            when (it.extension) {
                "mp4", "mkv" -> true
                else -> false
            }
        }
        .map { it.nameWithoutExtension }
        .sorted()

fun subtitlesFiles(files: Array<File>) =
    files.filter { it.isFile }
        .filter { it.extension == "ass" }
        .filterNot { it.name.contains("cht") || it.name.contains("TC") }
        .sorted()
