use std::process::Output;
use std::process::{Command, ExitStatus};

fn execute_hdiffz(args: &[&str]) -> Result<Output, std::io::Error> {
    Command::new("hdiffz").args(args).output()
}

// Diff Function
// Command: hdiffz oldPath newPath outDiffFile
pub fn diff_files(
    old_path: &str,
    new_path: &str,
    out_diff_file: &str,
) -> Result<Output, std::io::Error> {
    execute_hdiffz(&[old_path, new_path, out_diff_file])
}

// Compress Function
// Command: hdiffz -c-... "" newPath outDiffFile
pub fn compress(new_path: &str, out_diff_file: &str) -> Result<Output, std::io::Error> {
    execute_hdiffz(&["-c-...", "", new_path, out_diff_file])
}

// Test Function
// Command: hdiffz -t oldPath newPath testDiffFile
pub fn test(
    old_path: &str,
    new_path: &str,
    test_diff_file: &str,
) -> Result<Output, std::io::Error> {
    execute_hdiffz(&["-t", old_path, new_path, test_diff_file])
}

// Resave Function
// Command: hdiffz -c-... diffFile outDiffFile
pub fn resave(diff_file: &str, out_diff_file: &str) -> Result<Output, std::io::Error> {
    execute_hdiffz(&["-c-...", diff_file, out_diff_file])
}

// Print Info Function
// Command: hdiffz -info diffFile
pub fn print_info(diff_file: &str) -> Result<Output, std::io::Error> {
    execute_hdiffz(&["-info", diff_file])
}

// Get Manifest Function
// Command: hdiffz -g#... -C-checksumType inputPath -M#outManifestTxtFile
pub fn get_manifest(input_path: &str, out_manifest_file: &str) -> Result<Output, std::io::Error> {
    execute_hdiffz(&[
        "-g#...",
        "-C-checksumType",
        input_path,
        "-M#",
        out_manifest_file,
    ])
}

// Manifest Diff Function
// Command: hdiffz [options] -M-old#oldManifestFile -M-new#newManifestFile oldPath newPath outDiffFile
pub fn manifest_diff(
    old_manifest_file: &str,
    new_manifest_file: &str,
    old_path: &str,
    new_path: &str,
    out_diff_file: &str,
) -> Result<Output, std::io::Error> {
    execute_hdiffz(&[
        "-M-old#",
        old_manifest_file,
        "-M-new#",
        new_manifest_file,
        old_path,
        new_path,
        out_diff_file,
    ])
}
