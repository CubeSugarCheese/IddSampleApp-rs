# IddSampleApp-rs

This repository is a Rust version for [IddSampleApp](https://github.com/microsoft/Windows-driver-samples/tree/main/video/IndirectDisplay/IddSampleApp)

Basically, it just creates a software device in windows.
It should be used in conjunction with [IddSampleDriver](https://github.com/microsoft/Windows-driver-samples/tree/main/video/IndirectDisplay/IddSampleDriver) to create an Indirect Display Device.

I am using rust to rewrite IddSampleDriver, but it has not been completed yet.

# Build
[Indirect Display Device](https://learn.microsoft.com/zh-cn/windows-hardware/drivers/display/indirect-display-driver-model-overview)
only run in Windows 10 version 1803+ (https://learn.microsoft.com/en-us/windows-hardware/drivers/wdf/building-a-wdf-driver-for-multiple-versions-of-windows).

If you have installed Rust in your Windows computer, you just need you run `cargo build --release`.

After build completed, you can find output in `/target/release/idd_sample_app.exe` 

# Run
Just run the exe.
We will request admin permission to create device.