use ash::vk::{self, API_VERSION_1_3};

pub const VERSION: u32 = vk::make_api_version(0, 0, 1, 0);

fn main() {
    let entry =
        unsafe { ash::Entry::load().expect("Something went wrong with the entry creation") };
    // Instance creation info
    let app_info = app_info();
    let layer_names = vec![std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
    let layer_names_pointers = layer_names_pointers(layer_names);
    let extension_name_pointers = vec![ash::extensions::ext::DebugUtils::name().as_ptr()];
    let instance_create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        pp_enabled_layer_names: layer_names_pointers.as_ptr(),
        enabled_layer_count: layer_names_pointers.len() as _,
        pp_enabled_extension_names: extension_name_pointers.as_ptr(),
        enabled_extension_count: extension_name_pointers.len() as _,
        ..Default::default()
    };
    dbg!(&instance_create_info);
    // Create the instance
    let instance = unsafe {
        entry
            .create_instance(&instance_create_info, None)
            .expect("Something went wrong when creating the instance")
    };
    // Debug info
    let debug_utils = ash::extensions::ext::DebugUtils::new(&entry, &instance);
    let debugcreateinfo = vk::DebugUtilsMessengerCreateInfoEXT {
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        pfn_user_callback: Some(vulkan_debug_utils_callback),
        ..Default::default()
    };
    let utils_messenger = unsafe {
        debug_utils
            .create_debug_utils_messenger(&debugcreateinfo, None)
            .unwrap()
    };
    unsafe {
        debug_utils.destroy_debug_utils_messenger(utils_messenger, None);
        instance.destroy_instance(None);
    };
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let severity = format!("{:?}", message_severity).to_lowercase();
    let ty = format!("{:?}", message_type).to_lowercase();
    println!("[Debug][{}][{}] {:?}", severity, ty, message);
    vk::FALSE
}

fn app_info() -> vk::ApplicationInfo {
    let engine_name = std::ffi::CString::new("rungine").unwrap();
    let app_name = std::ffi::CString::new("rungine-basic").unwrap();
    vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        p_engine_name: engine_name.as_ptr(),
        engine_version: VERSION,
        application_version: VERSION,
        api_version: API_VERSION_1_3,
        ..Default::default()
    }
}

fn layer_names_pointers(layer_names: Vec<std::ffi::CString>) -> Vec<*const i8> {
    layer_names
        .iter()
        .map(|layer_name| layer_name.as_ptr())
        .collect()
}
