use std::fs;
use std::path::Path;

fn main() {
    tauri_build::build();

    // 复制 AuditService.app bundle 到 bin 目录
    let audit_service_app = "../../audit-service/swift/AuditService.app";
    let bin_dir = "bin";

    // 确保 bin 目录存在
    fs::create_dir_all(bin_dir).ok();

    // 复制整个 app bundle
    if Path::new(audit_service_app).exists() {
        let target = format!("{}/AuditService.app", bin_dir);

        // 先删除旧的
        let _ = fs::remove_dir_all(&target);

        // 递归复制目录
        copy_dir_all(audit_service_app, &target).expect("Failed to copy AuditService.app");

        println!("cargo:warning=Copied AuditService.app to bin directory");
    } else {
        println!("cargo:warning=AuditService.app not found at {}", audit_service_app);
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
