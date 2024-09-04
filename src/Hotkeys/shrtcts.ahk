; AutoHotkey script for Nilux OS UI and shortcuts

; Ensure single instance
#SingleInstance, Force
SetWorkingDir, %A_ScriptDir%

; Global variables
global niluxVersion := "5.13"

; UI elements
Gui, Add, Text, x10 y10, Welcome to Nilux OS v%niluxVersion%
Gui, Add, Button, x10 y40 w120 h30 gLaunchTerminal, Launch Terminal
Gui, Add, Button, x140 y40 w120 h30 gOpenFileManager, File Manager
Gui, Add, Button, x10 y80 w120 h30 gSystemSettings, System Settings
Gui, Add, Button, x140 y80 w120 h30 gToggleDesktop, Toggle Desktop

; Show the GUI
Gui, Show, w270 h120, Nilux OS Control Panel

; Shortcuts
^!t::LaunchTerminal()
#e::OpenFileManager()
#i::SystemSettings()
#d::ToggleDesktop()

; Functions
LaunchTerminal() {
    Run, cmd.exe
}

OpenFileManager() {
    Run, explorer.exe
}

SystemSettings() {
    Run, ms-settings:
}

ToggleDesktop() {
    Send, #d
}

; Exit script
GuiClose:
ExitApp

; Hotkey to reload the script
^!r::Reload
