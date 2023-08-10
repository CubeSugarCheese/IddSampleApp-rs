use console::Term;
use core::ffi::c_void;
use std::mem::size_of_val;
use windows::core::{HRESULT, PCWSTR};
use windows::w;
use windows::Win32::Devices::Enumeration::Pnp::{
    SWDeviceCapabilitiesDriverRequired, SWDeviceCapabilitiesRemovable,
    SWDeviceCapabilitiesSilentInstall, SwDeviceClose, SwDeviceCreate, HSWDEVICE,
    SW_DEVICE_CREATE_INFO,
};
use windows::Win32::Foundation::{HANDLE, WAIT_OBJECT_0};
use windows::Win32::System::Threading::{CreateEventW, SetEvent, WaitForSingleObject};

fn wait_for_chars_input(char_list: &[char]) {
    let term = Term::stdout();
    loop {
        let read_char = term.read_char();
        if read_char.is_ok_and(|read_char| char_list.contains(&read_char)) {
            break;
        }
    }
}

#[allow(unused_variables, non_snake_case)]
unsafe extern "system" fn CreationCallback(
    hswdevice: HSWDEVICE,
    createresult: HRESULT,
    pcontext: *const c_void,
    pszdeviceinstanceid: PCWSTR,
) {
    let event_handle = *(pcontext as *const HANDLE);
    SetEvent(event_handle);
}

fn main() {
    let event_handle = unsafe { CreateEventW(None, false, false, None).unwrap() };
    let mut create_info = SW_DEVICE_CREATE_INFO::default();
    create_info.cbSize = size_of_val(&create_info) as u32;
    create_info.pszzCompatibleIds = w!("IddSampleDriver\0\0");
    create_info.pszInstanceId = w!("IddSampleDriver");
    create_info.pszzHardwareIds = w!("IddSampleDriver\0\0");
    create_info.pszDeviceDescription = w!("Idd Sample Driver");

    create_info.CapabilityFlags = (SWDeviceCapabilitiesRemovable.0
        | SWDeviceCapabilitiesSilentInstall.0
        | SWDeviceCapabilitiesDriverRequired.0) as u32;

    // Create the device
    let software_device_handle = unsafe {
        SwDeviceCreate(
            w!("IddSampleDriver"),
            w!("HTREE\\ROOT\\0"),
            &create_info,
            None,
            Some(CreationCallback),
            Some(&event_handle as *const HANDLE as *const c_void),
        )
        .unwrap_or_else(|error| {
            panic!(
                "SwDeviceCreate failed with code: {code},reason: {reason}",
                code = error.code(),
                reason = error.message()
            )
        })
    };

    println!("Waiting for device to be created....");
    let wait_result = unsafe { WaitForSingleObject(event_handle, 10 * 1000) };
    if wait_result != WAIT_OBJECT_0 {
        panic!("Wait for device creation failed");
    }
    println!("Device created\n");

    // Now wait for user to indicate the device should be stopped
    println!("Press 'x' to exit and destroy the software device");
    wait_for_chars_input(&['x', 'X']);

    // Stop the device, this will cause the sample to be unloaded
    unsafe { SwDeviceClose(HSWDEVICE(software_device_handle)) };
}
