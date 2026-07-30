use serde::Serialize;
use std::sync::Mutex;
use tauri::menu::{Menu, Submenu, MenuItem, PredefinedMenuItem};

/// 设备信息结构体
#[derive(Debug, Clone, Serialize)]
pub struct DeviceInfo {
    /// 总线编号
    pub bus_number: u8,
    /// 设备地址
    pub address: u8,
    /// 厂商 ID
    pub vendor_id: String,
    /// 厂商名称（自动识别）
    pub vendor_name: String,
    /// 产品 ID
    pub product_id: String,
    /// 设备描述
    pub description: String,
    /// 设备类别
    pub class: String,
    /// 设备来源: "WMI" (Windows 全量检测) 或 "USB" (libusb)
    pub source: String,
    /// 设备状态: "正常" / "已断开" / "未知"
    pub status: String,
    /// 设备实例 ID（Windows PnP 路径）
    pub instance_id: String,
}

/// 设备检测结果
#[derive(Debug, Clone, Serialize)]
pub struct DetectionResult {
    /// 设备总数
    pub total_count: usize,
    /// 设备列表
    pub devices: Vec<DeviceInfo>,
    /// 检测时间戳
    pub timestamp: String,
    /// 检测方式
    pub method: String,
}

/// Tauri 应用状态
pub struct AppState {
    pub last_result: Mutex<Option<DetectionResult>>,
}

/// 常见 USB 厂商 ID → 厂商名称映射表
fn lookup_vendor_name(vid_hex: &str) -> &'static str {
    // vid_hex 格式: "0xXXXX"
    let vid = vid_hex.to_uppercase();
    match vid.as_str() {
        // 主流厂商
        "0x03F0" | "0X03F0" => "HP (惠普)",
        "0x046D" | "0X046D" => "Logitech (罗技)",
        "0x045E" | "0X045E" => "Microsoft (微软)",
        "0x04E8" | "0X04E8" => "Samsung (三星)",
        "0x04CC" | "0X04CC" => "ST-Ericsson",
        "0x05AC" | "0X05AC" => "Apple (苹果)",
        "0x0B05" | "0X0B05" => "ASUS (华硕)",
        "0x04F2" | "0X04F2" => "Chicony (群光)",
        "0x0781" | "0X0781" => "SanDisk (闪迪)",
        "0x0951" | "0X0951" => "Kingston (金士顿)",
        "0x0930" | "0X0930" => "Toshiba (东芝)",
        "0x04A9" | "0X04A9" => "Canon (佳能)",
        "0x04B0" | "0X04B0" => "Nikon (尼康)",
        "0x04B8" | "0X04B8" => "Epson (爱普生)",
        "0x046A" | "0X046A" => "Cherry (樱桃)",
        "0x04D9" | "0X04D9" => "Holtek",
        "0x0461" | "0X0461" => "Primax",
        "0x0471" | "0X0471" => "Philips (飞利浦)",
        "0x047D" | "0X047D" => "Kensington",
        "0x0483" | "0X0483" => "STMicroelectronics",
        "0x0489" | "0X0489" => "Foxconn / Hon Hai",
        "0x04B3" | "0X04B3" => "IBM",
        "0x04C5" | "0X04C5" => "Fujitsu (富士通)",
        "0x04CA" | "0X04CA" => "Lite-On",
        "0x04CB" | "0X04CB" => "Fuji Electric",
        "0x04D8" | "0X04D8" => "Microchip",
        "0x04E1" | "0X04E1" => "Hamamatsu",
        "0x04F3" | "0X04F3" => "Elan Microelectronics",
        "0x0525" | "0X0525" => "Netchip Technology",
        "0x054C" | "0X054C" => "Sony (索尼)",
        "0x058F" | "0X058F" => "Alcor Micro",
        "0x05E3" | "0X05E3" => "NEC",
        "0x0649" | "0X0649" => "Beyerdynamic",
        "0x067B" | "0X067B" => "Prolific Technology",
        "0x06A3" | "0X06A3" => "Saitek",
        "0x06CB" | "0X06CB" => "Synaptics",
        "0x0718" | "0X0718" => "Imation",
        "0x0764" | "0X0764" => "Cyber Power System",
        "0x07B8" | "0X07B8" => "AboCom",
        "0x0846" | "0X0846" => "NetGear",
        "0x08BD" | "0X08BD" => "Citizen Systems",
        "0x090C" | "0X090C" => "Silicon Motion",
        "0x093A" | "0X093A" => "Pixart Imaging",
        "0x0955" | "0X0955" => "NVidia",
        "0x0A5C" | "0X0A5C" => "Broadcom",
        "0x0A81" | "0X0A81" => "Chesen Electronics",
        "0x0B1F" | "0X0B1F" => "C-One Technology",
        "0x0BDA" | "0X0BDA" => "Realtek (瑞昱)",
        "0x0C45" | "0X0C45" => "Sonix Technology",
        "0x0CF3" | "0X0CF3" => "Atheros Communications",
        "0x0D8C" | "0X0D8C" => "C-Media Electronics",
        "0x0DB0" | "0X0DB0" => "Micro Star (MSI/微星)",
        "0x0DF6" | "0X0DF6" => "Sitecom",
        "0x0E0F" | "0X0E0F" => "VMware",
        "0x0E8D" | "0X0E8D" => "MediaTek (联发科)",
        "0x0F39" | "0X0F39" => "Total Technologies",
        "0x1058" | "0X1058" => "Western Digital (西部数据)",
        "0x105D" | "0X105D" => "IOI ( ITE )",
        "0x1199" | "0X1199" => "Sierra Wireless",
        "0x12D1" | "0X12D1" => "Huawei (华为)",
        "0x13D3" | "0X13D3" => "AzureWave",
        "0x148F" | "0X148F" => "Ralink Technology",
        "0x152D" | "0X152D" => "JMicron Technology",
        "0x154B" | "0X154B" => "PNY Technologies",
        "0x174F" | "0X174F" => "Syntek",
        "0x17EF" | "0X17EF" => "Lenovo (联想)",
        "0x18D1" | "0X18D1" => "Google",
        "0x1A2C" | "0X1A2C" => "China Resource Semico",
        "0x1A40" | "0X1A40" => "Terminus Technology",
        "0x1BCF" | "0X1BCF" => "Sunplus Technology",
        "0x1D6B" | "0X1D6B" => "Linux Foundation (USB Hub)",
        "0x2109" | "0X2109" => "VIA Labs (USB Hub)",
        "0x248A" | "0X248A" => "Maxxter",
        "0x258A" | "0X258A" => "Synaptics (触控板)",
        "0x2717" | "0X2717" => "Xiaomi (小米)",
        "0x294B" | "0X294B" => "Huawei",
        "0x8087" | "0X8087" => "Intel (英特尔) USB Hub",
        // 默认
        _ => "未知厂商",
    }
}


// ============================================================
// Windows: 使用 WMI 查询所有即插即用设备（含内置硬件）
// ============================================================
#[cfg(target_os = "windows")]
fn detect_pnp_devices_wmi() -> Result<Vec<DeviceInfo>, String> {
    use wmi::{COMLibrary, WMIConnection, Variant};
    use std::collections::HashMap;

    // 初始化 COM（WMI 依赖 COM）
    let com = COMLibrary::new().map_err(|e| format!("COM 初始化失败: {}", e))?;
    let wmi_conn = WMIConnection::new(com.into())
        .map_err(|e| format!("WMI 连接失败: {}", e))?;

    // 查询所有即插即用设备（包含内置硬件：显卡、网卡、磁盘、CPU 等）
    let results: Vec<HashMap<String, Variant>> = wmi_conn
        .raw_query("SELECT Name, Status, PNPClass, DeviceID, ConfigManagerErrorCode FROM Win32_PnPEntity")
        .map_err(|e| format!("WMI 查询失败: {}", e))?;

    let mut devices = Vec::new();

    for (i, props) in results.iter().enumerate() {
        // 设备名称
        let name = match props.get("Name") {
            Some(Variant::String(s)) if !s.is_empty() => s.clone(),
            _ => match props.get("DeviceID") {
                Some(Variant::String(id)) => id.clone(),
                _ => format!("未知设备 #{}", i),
            },
        };

        // 设备状态
        let status = match props.get("Status") {
            Some(Variant::String(s)) => match s.as_str() {
                "OK" => "正常",
                "Error" => "异常",
                "Degraded" => "降级",
                "Unknown" => "未知",
                _ => s.as_str(),
            },
            _ => "未知",
        };

        // 设备类别
        let class = match props.get("PNPClass") {
            Some(Variant::String(c)) if !c.is_empty() => translate_pnp_class(c),
            _ => "其他".to_string(),
        };

        // 设备实例 ID
        let device_id = match props.get("DeviceID") {
            Some(Variant::String(id)) => id.clone(),
            _ => String::new(),
        };

        // 从 DeviceID 解析厂商/产品 ID
        let (vendor_id, product_id) = parse_hardware_id(&device_id);
        let vendor_name = lookup_vendor_name(&vendor_id).to_string();

        devices.push(DeviceInfo {
            bus_number: 0,
            address: i as u8,
            vendor_id,
            vendor_name,
            product_id,
            description: name,
            class,
            source: "WMI".to_string(),
            status: status.to_string(),
            instance_id: device_id,
        });
    }

    Ok(devices)
}

/// 将 Windows PnP Class GUID 名称翻译为中文
#[cfg(target_os = "windows")]
fn translate_pnp_class(class: &str) -> String {
    match class {
        "Display" => "显示适配器".to_string(),
        "Net" => "网络适配器".to_string(),
        "DiskDrive" => "磁盘驱动器".to_string(),
        "CDROM" => "光驱".to_string(),
        "USB" => "USB 控制器".to_string(),
        "HIDClass" => "HID 设备".to_string(),
        "Keyboard" => "键盘".to_string(),
        "Mouse" => "鼠标".to_string(),
        "Monitor" => "监视器".to_string(),
        "Media" => "音频/视频".to_string(),
        "Sound" => "声音设备".to_string(),
        "Printer" => "打印机".to_string(),
        "SCSIAdapter" => "SCSI 适配器".to_string(),
        "System" => "系统设备".to_string(),
        "Processor" => "处理器".to_string(),
        "Volume" => "存储卷".to_string(),
        "FDC" => "软盘控制器".to_string(),
        "HDC" => "硬盘控制器".to_string(),
        "Ports" => "端口".to_string(),
        "Modem" => "调制解调器".to_string(),
        "Image" => "图像设备".to_string(),
        "Camera" => "摄像头".to_string(),
        "Bluetooth" => "蓝牙".to_string(),
        "Infrared" => "红外设备".to_string(),
        "1394" => "IEEE 1394".to_string(),
        "Pcmcia" => "PCMCIA".to_string(),
        "SmartCardReader" => "智能卡读卡器".to_string(),
        "SecurityDevices" => "安全设备".to_string(),
        "SoftwareDevice" => "软件设备".to_string(),
        "Firmware" => "固件".to_string(),
        "BusExtender" => "总线扩展器".to_string(),
        "LegacyDriver" => "旧版驱动".to_string(),
        "USBClass" => "USB 设备".to_string(),
        "NetTrans" => "网络协议".to_string(),
        "NetClient" => "网络客户端".to_string(),
        "NetService" => "网络服务".to_string(),
        "Battery" => "电池".to_string(),
        "Computer" => "计算机".to_string(),
        "Decoder" => "解码器".to_string(),
        "Diagnostic" => "诊断设备".to_string(),
        "Memory" => "内存".to_string(),
        _ => format!("{} 设备", class),
    }
}

/// 从 Windows DeviceID 字符串中解析厂商 ID 和产品 ID
#[cfg(target_os = "windows")]
fn parse_hardware_id(device_id: &str) -> (String, String) {
    // USB 设备: USB\VID_XXXX&PID_XXXX\...
    if let Some(vid_pos) = device_id.find("VID_") {
        let vid_start = vid_pos + 4;
        if let Some(vid) = device_id.get(vid_start..vid_start + 4) {
            let vendor_id = format!("0x{}", vid.to_uppercase());

            let product_id = if let Some(pid_pos) = device_id.find("PID_") {
                let pid_start = pid_pos + 4;
                device_id
                    .get(pid_start..pid_start + 4)
                    .map(|p| format!("0x{}", p.to_uppercase()))
                    .unwrap_or_else(|| "N/A".to_string())
            } else {
                "N/A".to_string()
            };

            return (vendor_id, product_id);
        }
    }

    // PCI 设备: PCI\VEN_XXXX&DEV_XXXX\...
    if let Some(ven_pos) = device_id.find("VEN_") {
        let ven_start = ven_pos + 4;
        if let Some(ven) = device_id.get(ven_start..ven_start + 4) {
            let vendor_id = format!("0x{}", ven.to_uppercase());

            let product_id = if let Some(dev_pos) = device_id.find("DEV_") {
                let dev_start = dev_pos + 4;
                device_id
                    .get(dev_start..dev_start + 4)
                    .map(|d| format!("0x{}", d.to_uppercase()))
                    .unwrap_or_else(|| "N/A".to_string())
            } else {
                "N/A".to_string()
            };

            return (vendor_id, product_id);
        }
    }

    // ACPI 设备: ACPI\XXXX\...
    if device_id.starts_with("ACPI\\") {
        let parts: Vec<&str> = device_id.split('\\').collect();
        if parts.len() >= 2 {
            return (format!("ACPI:{}", parts[1]), "N/A".to_string());
        }
    }

    ("N/A".to_string(), "N/A".to_string())
}

// ============================================================
// macOS: 使用 system_profiler 检测蓝牙设备
// ============================================================
#[cfg(target_os = "macos")]
fn detect_bluetooth_devices_macos() -> Result<Vec<DeviceInfo>, String> {
    use std::process::Command;

    // 使用 system_profiler 获取蓝牙设备信息（JSON 格式）
    let output = Command::new("system_profiler")
        .args(&["SPBluetoothDataType", "-json"])
        .output()
        .map_err(|e| format!("执行 system_profiler 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // 解析 JSON
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("JSON 解析失败: {}", e))?;

    let mut devices = Vec::new();
    let mut index: u8 = 0;

    // 获取 SPBluetoothDataType 数组
    if let Some(bt_array) = json.get("SPBluetoothDataType").and_then(|v| v.as_array()) {
        for bt_entry in bt_array {
            // 解析已连接设备
            if let Some(connected) = bt_entry.get("device_connected").and_then(|v| v.as_array()) {
                for device_obj in connected {
                    if let Some(obj) = device_obj.as_object() {
                        for (name, props) in obj {
                            let address = props.get("device_address")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let vendor_id = props.get("device_vendorID")
                                .and_then(|v| v.as_str())
                                .unwrap_or("N/A")
                                .to_string();
                            let product_id = props.get("device_productID")
                                .and_then(|v| v.as_str())
                                .unwrap_or("N/A")
                                .to_string();
                            let minor_type = props.get("device_minorType")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();

                            let description = if minor_type.is_empty() {
                                name.clone()
                            } else {
                                format!("{} ({})", name, minor_type)
                            };

                            let vendor_name = lookup_vendor_name(&vendor_id).to_string();

                            devices.push(DeviceInfo {
                                bus_number: 0,
                                address: index,
                                vendor_id,
                                vendor_name,
                                product_id,
                                description,
                                class: "蓝牙".to_string(),
                                source: "Bluetooth".to_string(),
                                status: "正常".to_string(),
                                instance_id: format!("BLUETOOTH:{}", address),
                            });
                            index += 1;
                        }
                    }
                }
            }
        }
    }

    Ok(devices)
}

// ============================================================
// 跨平台: USB 设备检测（libusb）
// ============================================================
fn detect_usb_devices() -> Result<Vec<DeviceInfo>, String> {
    let devices = rusb::devices()
        .map_err(|e| format!("无法枚举 USB 设备: {}", e))?;

    let mut device_list = Vec::new();

    for device in devices.iter() {
        let desc = device.device_descriptor()
            .map_err(|e| format!("无法读取设备描述符: {}", e))?;

        let vendor_id = format!("0x{:04X}", desc.vendor_id());
        let product_id = format!("0x{:04X}", desc.product_id());

        let description = match device.open() {
            Ok(handle) => {
                handle.read_product_string_ascii(&desc).unwrap_or_else(|_| {
                    format!("VID:{} PID:{}", vendor_id, product_id)
                })
            }
            Err(_) => format!("VID:{} PID:{}", vendor_id, product_id),
        };

        let class = match desc.class_code() {
            0x00 => "未指定".to_string(),
            0x01 => "音频设备".to_string(),
            0x02 => "通信设备".to_string(),
            0x03 => "HID 设备 (人机交互)".to_string(),
            0x05 => "物理接口设备".to_string(),
            0x06 => "图像设备".to_string(),
            0x07 => "打印机".to_string(),
            0x08 => "大容量存储".to_string(),
            0x09 => "USB Hub".to_string(),
            0x0A => "CDC 数据".to_string(),
            0x0B => "智能卡".to_string(),
            0x0D => "内容安全".to_string(),
            0x0E => "视频设备".to_string(),
            0x0F => "个人健康".to_string(),
            0x10 => "音视频".to_string(),
            0x11 => "广播设备".to_string(),
            0x12 => "蓝牙".to_string(),
            0x13 => "宽带".to_string(),
            0x14 => "健康设备".to_string(),
            0xDC => "诊断设备".to_string(),
            0xE0 => "无线设备".to_string(),
            0xEF => "杂项".to_string(),
            0xFE => "应用特定".to_string(),
            0xFF => "厂商特定".to_string(),
            _ => format!("其他 (0x{:02X})", desc.class_code()),
        };

        let vendor_name = lookup_vendor_name(&vendor_id).to_string();

        device_list.push(DeviceInfo {
            bus_number: device.bus_number(),
            address: device.address(),
            vendor_id,
            vendor_name,
            product_id,
            description,
            class,
            source: "USB".to_string(),
            status: "正常".to_string(),
            instance_id: format!("USB:{}:{}", device.bus_number(), device.address()),
        });
    }

    Ok(device_list)
}

// ============================================================
// Tauri 命令：检测所有设备
// ============================================================

/// 核心设备检测逻辑（内部函数）
fn do_detect_devices() -> Result<DetectionResult, String> {
    let (devices, method) = {
        #[cfg(target_os = "windows")]
        {
            // Windows: 优先使用 WMI 获取全部硬件（含内置设备）
            match detect_pnp_devices_wmi() {
                Ok(mut wmi_devices) => {
                    // 同时获取 USB 设备详情作为补充
                    if let Ok(usb_devices) = detect_usb_devices() {
                        wmi_devices.extend(usb_devices);
                    }
                    (wmi_devices, "WMI + USB (Windows)")
                }
                Err(e) => {
                    // WMI 失败时回退到纯 USB 检测
                    eprintln!("WMI 检测失败，回退到 USB 模式: {}", e);
                    (detect_usb_devices()?, "USB 回退模式")
                }
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            let mut all_devices = detect_usb_devices().unwrap_or_default();
            
            // macOS 额外检测蓝牙设备
            #[cfg(target_os = "macos")]
            {
                if let Ok(bt_devices) = detect_bluetooth_devices_macos() {
                    all_devices.extend(bt_devices);
                }
            }
            
            (all_devices, "USB + Bluetooth")
        }
    };

    let result = DetectionResult {
        total_count: devices.len(),
        devices,
        timestamp: chrono_now(),
        method: method.to_string(),
    };

    Ok(result)
}

/// Tauri 命令：detect_devices
#[tauri::command]
fn detect_devices() -> Result<DetectionResult, String> {
    do_detect_devices()
}

/// Tauri 命令：scan_devices（前端兼容别名）
#[tauri::command]
fn scan_devices() -> Result<DetectionResult, String> {
    do_detect_devices()
}

/// 获取当前时间字符串
fn chrono_now() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    format!("Unix timestamp: {}", secs)
}

/// 获取设备数量（轻量接口）
#[tauri::command]
fn get_device_count() -> Result<usize, String> {
    #[cfg(target_os = "windows")]
    {
        // Windows 上优先使用 WMI
        if let Ok(devices) = detect_pnp_devices_wmi() {
            return Ok(devices.len());
        }
    }
    let devices = rusb::devices()
        .map_err(|e| format!("无法枚举 USB 设备: {}", e))?;
    Ok(devices.iter().count())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            last_result: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![detect_devices, scan_devices, get_device_count])
        .setup(|app| {
            // 中文菜单
            let file_menu = Submenu::with_items(app, "文件", true, &[
                &MenuItem::with_id(app, "quit", "退出", true, Some("CmdOrCtrl+Q"))?,
            ])?;

            let edit_menu = Submenu::with_items(app, "编辑", true, &[
                &PredefinedMenuItem::undo(app, Some("撤销"))?,
                &PredefinedMenuItem::redo(app, Some("重做"))?,
                &PredefinedMenuItem::separator(app)?,
                &PredefinedMenuItem::cut(app, Some("剪切"))?,
                &PredefinedMenuItem::copy(app, Some("复制"))?,
                &PredefinedMenuItem::paste(app, Some("粘贴"))?,
                &PredefinedMenuItem::select_all(app, Some("全选"))?,
            ])?;

            let view_menu = Submenu::with_items(app, "视图", true, &[
                &MenuItem::with_id(app, "reload", "刷新", true, Some("CmdOrCtrl+R"))?,
                &MenuItem::with_id(app, "devtools", "开发者工具", true, Some("CmdOrCtrl+Shift+I"))?,
                &PredefinedMenuItem::separator(app)?,
                &MenuItem::with_id(app, "fullscreen", "进入全屏", true, Some("CmdOrCtrl+F"))?,
            ])?;

            let window_menu = Submenu::with_items(app, "窗口", true, &[
                &PredefinedMenuItem::close_window(app, Some("关闭窗口"))?,
                &PredefinedMenuItem::minimize(app, Some("最小化"))?,
                &MenuItem::with_id(app, "zoom", "缩放", true, None::<&str>)?,
            ])?;
            let help_menu = Submenu::with_items(app, "帮助", true, &[
                &MenuItem::with_id(app, "about", "关于硬件设备检测器", true, None::<&str>)?,
            ])?;

            let menu = Menu::with_items(app, &[
                &file_menu,
                &edit_menu,
                &view_menu,
                &window_menu,
                &help_menu,
            ])?;

            app.set_menu(menu)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用时出错");
}
