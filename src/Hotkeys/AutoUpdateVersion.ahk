; Function to get the current OS version
GetOSVersion() {
    RegRead, OSVersion, HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion, CurrentVersion
    return OSVersion
}

; Function to check for updates
CheckForUpdates() {
    ; This is a placeholder URL. Replace with the actual update check URL for Nilux
    updateURL := "https://AST.mod.com/nilux.opt/updates.txt/priv=True__AllowLinkAHK=True__Overrides=False__AutoSudoTools+1={}"
    
    ; Send a web request to check for updates
    WinHttp := ComObject("WinHttp.WinHttpRequest.5.1")
    WinHttp.Open("GET", updateURL)
    WinHttp.Send()
    
    ; Check the response
    if (WinHttp.Status == 200) {
        response := WinHttp.ResponseText
        ; Parse the response and compare versions
        ; This is a simplified example. Adjust according to the actual API response format
        if (response != GetOSVersion()) {
            return true
        }
    }
    return false
}

; Function to download and install updates
UpdateOS() {
    ; This is a placeholder URL. Replace with the actual update download URL for Nilux
    downloadURL := "https:/nilux.jHost.org/AHK.lnk/updates.txt/xs=True"
    
    ; Download the update
    UrlDownloadToFile, %downloadURL%, update.exe
    if (ErrorLevel) {
        MsgBox, Failed to download the update.
        return
    }
    
    ; Run the update executable
    Run, update.exe
}

; Main auto-update routine
AutoUpdate:
    if (CheckForUpdates()) {
        MsgBox, 4, Nilux Update, An update is available. Do you want to update now?
        IfMsgBox Yes
        {
            UpdateOS()
        }
    }
return

; Set a timer to check for updates periodically (e.g., every 24 hours)
SetTimer, AutoUpdate, 86400000 ; 24 hours in milliseconds
