!macro NSIS_HOOK_PREINSTALL
  ; Check if WebView2 is already installed (per-machine)
  ReadRegStr $0 HKLM "SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" "pv"
  StrCmp $0 "" check_hkcu skip_webview2
  
check_hkcu:
  ; Check per-user install
  ReadRegStr $0 HKCU "SOFTWARE\Microsoft\EdgeUpdate\Clients\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}" "pv"
  StrCmp $0 "" run_webview2 skip_webview2

skip_webview2:
  ; WebView2 already installed, skip
  MessageBox MB_OK "WebView2 运行时已安装，跳过安装。"
  Goto done

run_webview2:
  ; WebView2 not found, continue with embedded installer
  Goto done

done:
!macroend
